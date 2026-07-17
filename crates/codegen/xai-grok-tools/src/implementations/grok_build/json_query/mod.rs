//! `json_query` — deterministic JSON data operations.
//!
//! Adapted from Caelum's json_query tool.

use serde_json::Value;

use crate::types::tool::{ToolKind, ToolNamespace};

const MAX_ITEMS: usize = 500;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct JsonQueryInput {
    #[schemars(description = "JSON string to query")]
    pub json: String,

    #[schemars(description = "Operation: get, keys, count, filter, pick, sum, avg, min, max")]
    pub operation: String,

    #[schemars(description = "JSON path like 'users[0].name' (dot-notation, [N] indexing, [*] wildcard)")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[schemars(description = "Filter condition like 'price > 100'")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    #[schemars(description = "Comma-separated field names for pick operation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,

    #[schemars(description = "Field name for aggregate operations (sum/avg/min/max)")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregate_on: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JsonQueryOutput {
    pub result: String,
}
impl xai_tool_runtime::ToolOutput for JsonQueryOutput {}

#[derive(Debug, Default)]
pub struct JsonQueryTool;

impl crate::types::tool_metadata::ToolMetadata for JsonQueryTool {
    fn kind(&self) -> ToolKind { ToolKind::Other }
    fn tool_namespace(&self) -> ToolNamespace { ToolNamespace::GrokBuild }
    fn description_template(&self) -> &str {
        "Query JSON data deterministically. Operations: get (extract by path), \
         filter (filter array by condition), count, sum/avg/min/max (aggregate), \
         keys (list object keys), pick (select fields). Path: dot-notation with \
         [N] indexing. Results truncated to 500 items."
    }
}

impl xai_tool_runtime::Tool for JsonQueryTool {
    type Args = JsonQueryInput;
    type Output = JsonQueryOutput;

    fn id(&self) -> xai_tool_protocol::ToolId {
        xai_tool_protocol::ToolId::new("json_query").expect("valid")
    }
    fn description(&self, _: &xai_tool_runtime::ListToolsContext) -> xai_tool_types::ToolDescription {
        xai_tool_types::ToolDescription::new("json_query", crate::types::tool_metadata::ToolMetadata::description_template(self))
    }
    fn capabilities(&self) -> xai_tool_protocol::ToolCapabilities {
        xai_tool_protocol::ToolCapabilities { is_read_only: true, ..Default::default() }
    }

    async fn run(&self, _: xai_tool_runtime::ToolCallContext, input: JsonQueryInput) -> Result<JsonQueryOutput, xai_tool_runtime::ToolError> {
        let err = |s: &str| xai_tool_runtime::ToolError::execution(xai_tool_protocol::ToolId::new("json_query").expect("valid"), s.to_string());

        let root: Value = serde_json::from_str(&input.json).map_err(|_| err("invalid JSON"))?;
        let target = resolve_path(&root, input.path.as_deref().unwrap_or("")).unwrap_or(root.clone());

        let result = match input.operation.as_str() {
            "get" => serde_json::to_string_pretty(&target).unwrap_or_default(),
            "keys" => match &target {
                Value::Object(m) => serde_json::to_string(&m.keys().collect::<Vec<_>>()).unwrap_or_default(),
                _ => return Err(jq_err("not an object")),
            },
            "count" => match &target { Value::Array(a) => a.len().to_string(), _ => "1".into() },
            "filter" => filter_op(&target, input.condition.as_deref().unwrap_or(""))?,
            "pick" => pick_op(&target, input.fields.as_deref().unwrap_or(""))?,
            "sum" | "avg" | "min" | "max" => aggregate_op(&target, &input.operation, input.aggregate_on.as_deref().unwrap_or(""))?,
            _ => return Err(jq_err(&format!("unknown operation '{}'", input.operation))),
        };

        Ok(JsonQueryOutput { result })
    }
}

// ── path navigation ──

enum PathSegment { Key(String), Index(usize), Wildcard }

fn parse_path(path: &str) -> Vec<PathSegment> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let chars: Vec<char> = path.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '.' => { if !cur.is_empty() { out.push(PathSegment::Key(std::mem::take(&mut cur))); } i += 1; }
            '[' => {
                if !cur.is_empty() { out.push(PathSegment::Key(std::mem::take(&mut cur))); }
                i += 1; let mut idx = String::new();
                while i < chars.len() && chars[i] != ']' { idx.push(chars[i]); i += 1; }
                i += 1;
                if idx == "*" { out.push(PathSegment::Wildcard); }
                else if let Ok(n) = idx.parse::<usize>() { out.push(PathSegment::Index(n)); }
            }
            c => { cur.push(c); i += 1; }
        }
    }
    if !cur.is_empty() { out.push(PathSegment::Key(cur)); }
    out
}

fn resolve_path(root: &Value, path: &str) -> Option<Value> {
    if path.is_empty() || path == "." { return Some(root.clone()); }
    let mut current = root.clone();
    for seg in parse_path(path) {
        current = match seg {
            PathSegment::Key(k) => current.get(&k)?.clone(),
            PathSegment::Index(i) => current.get(i)?.clone(),
            PathSegment::Wildcard => return Some(current),
        };
    }
    Some(current)
}

// ── operations ──

fn parse_cond(cond: &str) -> Option<(&str, &str, &str)> {
    for op in &[">=", "<=", "!=", "==", ">", "<", "=", "contains"] {
        if let Some(pos) = cond.trim().find(op) {
            let f = cond[..pos].trim();
            let v = cond[pos + op.len()..].trim();
            return Some((f, if *op == "=" { "==" } else { *op }, v));
        }
    }
    None
}

fn eval_cond(item: &Value, field: &str, op: &str, val_str: &str) -> bool {
    let fv = item.get(field);
    if op == "contains" {
        return fv.and_then(|v| v.as_str()).map(|s| s.to_lowercase().contains(&val_str.trim_matches('"').to_lowercase())).unwrap_or(false);
    }
    let a = fv.and_then(|v| v.as_f64());
    let b: Option<f64> = val_str.parse().ok();
    match (a, b) {
        (Some(x), Some(y)) => match op {
            ">" => x > y, "<" => x < y, ">=" => x >= y, "<=" => x <= y,
            "==" => (x - y).abs() < f64::EPSILON, "!=" => (x - y).abs() >= f64::EPSILON,
            _ => false,
        },
        _ => {
            let a_s = fv.and_then(|v| v.as_str()).unwrap_or("");
            match op { "==" => a_s == val_str.trim_matches('"'), "!=" => a_s != val_str.trim_matches('"'), _ => false }
        }
    }
}

fn jq_err(s: &str) -> xai_tool_runtime::ToolError {
    xai_tool_runtime::ToolError::execution(xai_tool_protocol::ToolId::new("json_query").expect("valid"), s.to_string())
}

fn filter_op(target: &Value, cond: &str) -> Result<String, xai_tool_runtime::ToolError> {
    if cond.is_empty() { return Err(jq_err("condition required")); }
    let Some((f, op, val)) = parse_cond(cond) else { return Err(jq_err("invalid condition format")); };
    match target {
        Value::Array(arr) => {
            let matching: Vec<_> = arr.iter().filter(|it| eval_cond(it, f, op, val)).collect();
            let total = matching.len();
            let out: Vec<_> = matching.into_iter().take(MAX_ITEMS).collect();
            let mut r = serde_json::to_string_pretty(&out).unwrap_or_default();
            if total > MAX_ITEMS { r.push_str(&format!("\n(truncated: {} of {})", MAX_ITEMS, total)); }
            Ok(r)
        }
        _ => Err(jq_err("filter requires an array")),
    }
}

fn pick_op(target: &Value, fields: &str) -> Result<String, xai_tool_runtime::ToolError> {
    let cols: Vec<&str> = fields.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    if cols.is_empty() { return Err(jq_err("fields required")); }
    match target {
        Value::Array(arr) => {
            let picked: Vec<Value> = arr.iter().take(MAX_ITEMS).filter_map(|item| {
                let mut obj = serde_json::Map::new();
                for f in &cols { if let Some(v) = item.get(*f) { obj.insert(f.to_string(), v.clone()); } }
                if obj.is_empty() { None } else { Some(Value::Object(obj)) }
            }).collect();
            Ok(serde_json::to_string_pretty(&picked).unwrap_or_default())
        }
        Value::Object(obj) => {
            let mut m = serde_json::Map::new();
            for f in &cols { if let Some(v) = obj.get(*f) { m.insert(f.to_string(), v.clone()); } }
            Ok(serde_json::to_string_pretty(&Value::Object(m)).unwrap_or_default())
        }
        _ => Err(jq_err("pick requires array or object")),
    }
}

fn aggregate_op(target: &Value, op: &str, field: &str) -> Result<String, xai_tool_runtime::ToolError> {
    if field.is_empty() { return Err(jq_err("aggregate_on required")); }
    let values: Vec<f64> = match target {
        Value::Array(arr) => arr.iter().filter_map(|v| v.get(field)?.as_f64()).collect(),
        _ => return Err(jq_err("aggregate requires an array")),
    };
    if values.is_empty() { return Err(jq_err(&format!("no numeric values for '{}'", field))); }
    Ok(match op {
        "sum" => format!("{}", values.iter().sum::<f64>()),
        "avg" => format!("{:.4}", values.iter().sum::<f64>() / values.len() as f64),
        "min" => format!("{}", values.iter().fold(f64::INFINITY, |a, &b| a.min(b))),
        "max" => format!("{}", values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))),
        _ => unreachable!(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::resources::Resources;
    use crate::types::tool_metadata::test_ctx;

    async fn run(input: JsonQueryInput) -> Result<JsonQueryOutput, xai_tool_runtime::ToolError> {
        let r = Resources::new();
        xai_tool_runtime::Tool::run(&JsonQueryTool, test_ctx(r.into_shared()), input).await
    }

    #[tokio::test]
    async fn get_by_path() {
        let r = run(JsonQueryInput {
            json: r#"{"user":{"name":"Alice","age":30}}"#.into(),
            operation: "get".into(), path: Some("user.name".into()),
            condition: None, fields: None, aggregate_on: None,
        }).await.unwrap();
        assert!(r.result.contains("Alice"));
    }

    #[tokio::test]
    async fn count_array() {
        let r = run(JsonQueryInput {
            json: r#"{"items":[1,2,3,4,5]}"#.into(),
            operation: "count".into(), path: Some("items".into()),
            condition: None, fields: None, aggregate_on: None,
        }).await.unwrap();
        assert_eq!(r.result.trim(), "5");
    }

    #[tokio::test]
    async fn filter_numeric() {
        let r = run(JsonQueryInput {
            json: r#"{"items":[{"price":50},{"price":150},{"price":200}]}"#.into(),
            operation: "filter".into(), path: Some("items".into()),
            condition: Some("price > 100".into()), fields: None, aggregate_on: None,
        }).await.unwrap();
        assert!(r.result.contains("150"));
    }

    #[tokio::test]
    async fn aggregate_sum() {
        let r = run(JsonQueryInput {
            json: r#"{"orders":[{"total":10.5},{"total":20.0},{"total":30.0}]}"#.into(),
            operation: "sum".into(), path: Some("orders".into()),
            condition: None, fields: None, aggregate_on: Some("total".into()),
        }).await.unwrap();
        assert!(r.result.contains("60.5"));
    }

    #[test]
    fn tool_name() {
        assert_eq!(xai_tool_runtime::Tool::id(&JsonQueryTool).as_str(), "json_query");
    }
}
