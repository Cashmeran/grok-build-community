//! `csv_ops` — deterministic CSV/TSV data operations.
//!
//! Adapted from Caelum's csv_ops tool.

use std::collections::BTreeMap;

use crate::types::tool::{ToolKind, ToolNamespace};

const MAX_ROWS: usize = 500;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct CsvOpsInput {
    #[schemars(description = "CSV text to query")]
    pub csv_text: String,

    #[schemars(description = "Operation: schema, select, columns, sort, count, sum, avg")]
    pub operation: String,

    #[schemars(description = "Filter condition like 'price > 100'")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    #[schemars(description = "Comma-separated column names for columns operation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<String>,

    #[schemars(description = "Column name to sort by")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    #[schemars(description = "Sort descending (default: ascending)")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_desc: Option<bool>,

    #[schemars(description = "Column to aggregate on (sum/avg)")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregate_on: Option<String>,

    #[schemars(description = "Column to group by for aggregate operations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,

    #[schemars(description = "Delimiter character (default: comma)")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,

    #[schemars(description = "First row is header (default: true)")]
    #[serde(default = "default_true")]
    pub has_header: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CsvOpsOutput {
    pub result: String,
}
impl xai_tool_runtime::ToolOutput for CsvOpsOutput {}

#[derive(Debug, Default)]
pub struct CsvOpsTool;

impl crate::types::tool_metadata::ToolMetadata for CsvOpsTool {
    fn kind(&self) -> ToolKind {
        ToolKind::Other
    }
    fn tool_namespace(&self) -> ToolNamespace {
        ToolNamespace::GrokBuild
    }
    fn description_template(&self) -> &str {
        "Query CSV/TSV data deterministically. Operations: schema (column names + types), \
         select (filter rows by condition), columns (pick columns), sort, count/sum/avg \
         (aggregate with optional group_by). Supports custom delimiters. Results truncated to 500 rows."
    }
}

impl xai_tool_runtime::Tool for CsvOpsTool {
    type Args = CsvOpsInput;
    type Output = CsvOpsOutput;

    fn id(&self) -> xai_tool_protocol::ToolId {
        xai_tool_protocol::ToolId::new("csv_ops").expect("valid")
    }
    fn description(
        &self,
        _: &xai_tool_runtime::ListToolsContext,
    ) -> xai_tool_types::ToolDescription {
        xai_tool_types::ToolDescription::new(
            "csv_ops",
            crate::types::tool_metadata::ToolMetadata::description_template(self),
        )
    }
    fn capabilities(&self) -> xai_tool_protocol::ToolCapabilities {
        xai_tool_protocol::ToolCapabilities {
            is_read_only: true,
            ..Default::default()
        }
    }

    async fn run(
        &self,
        _: xai_tool_runtime::ToolCallContext,
        input: CsvOpsInput,
    ) -> Result<CsvOpsOutput, xai_tool_runtime::ToolError> {
        let delim = input
            .delimiter
            .as_deref()
            .unwrap_or(",")
            .chars()
            .next()
            .unwrap_or(',') as u8;
        let with_hdr = input.has_header;

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(delim)
            .has_headers(with_hdr)
            .flexible(true)
            .from_reader(input.csv_text.as_bytes());

        let headers: Vec<String> = if with_hdr {
            reader
                .headers()
                .map_err(|e: csv::Error| csv_err(&format!("header: {e}")))?
                .iter()
                .map(|s| s.to_string())
                .collect()
        } else {
            (0..).take(50).map(|i| format!("col_{i}")).collect()
        };

        let result = match input.operation.as_str() {
            "schema" => schema_op(&mut reader, &headers)?,
            "select" => select_op(
                &mut reader,
                &headers,
                input.condition.as_deref().unwrap_or(""),
            )?,
            "columns" => columns_op(
                &mut reader,
                &headers,
                input.columns.as_deref().unwrap_or(""),
            )?,
            "sort" => sort_op(
                &mut reader,
                &headers,
                input.sort_by.as_deref().unwrap_or(""),
                input.sort_desc.unwrap_or(false),
            )?,
            "count" | "sum" | "avg" => aggregate_csv_op(
                &mut reader,
                &headers,
                &input.operation,
                input.aggregate_on.as_deref().unwrap_or(""),
                input.group_by.as_deref().unwrap_or(""),
            )?,
            _ => return Err(csv_err(&format!("unknown operation '{}'", input.operation))),
        };

        Ok(CsvOpsOutput { result })
    }
}

// ── helpers ──

fn infer_type(values: &[&str]) -> &'static str {
    let ne: Vec<_> = values.iter().filter(|v| !v.trim().is_empty()).collect();
    if ne.is_empty() {
        return "empty";
    }
    if ne.iter().all(|v| v.trim().parse::<i64>().is_ok()) {
        return "integer";
    }
    if ne.iter().all(|v| v.trim().parse::<f64>().is_ok()) {
        return "float";
    }
    let bools = ne
        .iter()
        .filter(|v| {
            let t = v.trim().to_lowercase();
            t == "true" || t == "false" || t == "yes" || t == "no"
        })
        .count();
    if bools as f64 / ne.len() as f64 > 0.8 {
        "boolean"
    } else {
        "string"
    }
}

fn parse_csv_cond(text: &str) -> Option<(String, &str, String)> {
    for op in &[">=", "<=", "!=", "==", ">", "<", "=", "contains"] {
        if let Some(pos) = text.trim().find(op) {
            let col = text[..pos].trim().to_string();
            let val = text[pos + op.len()..].trim().to_string();
            return Some((col, if *op == "=" { "==" } else { *op }, val));
        }
    }
    None
}

fn row_matches(r: &csv::StringRecord, hdrs: &[String], col: &str, op: &str, val: &str) -> bool {
    let Some(idx) = hdrs.iter().position(|h| h == col) else {
        return false;
    };
    let cell = r.get(idx).unwrap_or("");
    if op == "contains" {
        return cell
            .to_lowercase()
            .contains(&val.trim_matches('"').to_lowercase());
    }
    let a: Option<f64> = cell.trim().parse().ok();
    let b: Option<f64> = val.trim().parse().ok();
    match (a, b) {
        (Some(x), Some(y)) => match op {
            ">" => x > y,
            "<" => x < y,
            ">=" => x >= y,
            "<=" => x <= y,
            "==" => (x - y).abs() < f64::EPSILON,
            "!=" => (x - y).abs() >= f64::EPSILON,
            _ => false,
        },
        _ => match op {
            "==" => cell.trim() == val.trim_matches('"'),
            "!=" => cell.trim() != val.trim_matches('"'),
            _ => false,
        },
    }
}

fn format_csv(hdrs: &[String], rows: &[csv::StringRecord], total: Option<usize>) -> String {
    let mut out = String::new();
    out.push_str(&hdrs.join(","));
    out.push('\n');
    for r in rows.iter().take(MAX_ROWS) {
        let fields: Vec<String> = (0..hdrs.len())
            .map(|i| {
                let f = r.get(i).unwrap_or("");
                if f.contains(',') || f.contains('"') || f.contains('\n') {
                    format!("\"{}\"", f.replace('"', "\"\""))
                } else {
                    f.to_string()
                }
            })
            .collect();
        out.push_str(&fields.join(","));
        out.push('\n');
    }
    if let Some(n) = total {
        if n > MAX_ROWS {
            out.push_str(&format!("(truncated: {} of {})\n", MAX_ROWS, n));
        }
    } else if rows.len() >= MAX_ROWS {
        out.push_str(&format!("(truncated: {})\n", MAX_ROWS));
    }
    out
}

// ── operations ──

fn schema_op<R: std::io::Read>(
    reader: &mut csv::Reader<R>,
    headers: &[String],
) -> Result<String, xai_tool_runtime::ToolError> {
    let mut samples: Vec<Vec<String>> = vec![Vec::new(); headers.len()];
    for rec in reader.records().take(MAX_ROWS) {
        let rec = rec.map_err(|e: csv::Error| csv_err(&e.to_string()))?;
        for (i, f) in rec.iter().enumerate() {
            if i < samples.len() {
                samples[i].push(f.to_string());
            }
        }
    }
    let mut out = String::from("column,type,samples\n");
    for (i, h) in headers.iter().enumerate() {
        let vals: Vec<&str> = samples[i].iter().map(|s| s.as_str()).collect();
        out.push_str(&format!("{},{},{}\n", h, infer_type(&vals), vals.len()));
    }
    Ok(out)
}

fn select_op<R: std::io::Read>(
    reader: &mut csv::Reader<R>,
    headers: &[String],
    cond: &str,
) -> Result<String, xai_tool_runtime::ToolError> {
    if cond.is_empty() {
        return Err(csv_err("condition required"));
    }
    let Some((col, op, val)) = parse_csv_cond(cond) else {
        return Err(csv_err("invalid condition"));
    };
    let mut rows = Vec::new();
    let mut total = 0;
    for rec in reader.records() {
        let rec = rec.map_err(|e: csv::Error| csv_err(&e.to_string()))?;
        if row_matches(&rec, headers, &col, op, &val) {
            if rows.len() < MAX_ROWS {
                rows.push(rec);
            }
            total += 1;
        }
    }
    Ok(format_csv(headers, &rows, Some(total)))
}

fn columns_op<R: std::io::Read>(
    reader: &mut csv::Reader<R>,
    headers: &[String],
    cols_str: &str,
) -> Result<String, xai_tool_runtime::ToolError> {
    let cols: Vec<&str> = cols_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if cols.is_empty() {
        return Err(csv_err("columns required"));
    }
    let idxs: Vec<(usize, String)> = cols
        .iter()
        .filter_map(|c| {
            headers
                .iter()
                .position(|h| h == *c)
                .map(|i| (i, c.to_string()))
        })
        .collect();
    if idxs.is_empty() {
        return Err(csv_err("no matching columns"));
    }
    let mut out = String::new();
    out.push_str(
        &idxs
            .iter()
            .map(|(_, c)| c.as_str())
            .collect::<Vec<_>>()
            .join(","),
    );
    out.push('\n');
    let mut n = 0;
    for rec in reader.records().take(MAX_ROWS) {
        let rec = rec.map_err(|e: csv::Error| csv_err(&e.to_string()))?;
        let fields: Vec<String> = idxs
            .iter()
            .map(|(i, _)| {
                let f = rec.get(*i).unwrap_or("");
                if f.contains(',') || f.contains('"') {
                    format!("\"{}\"", f.replace('"', "\"\""))
                } else {
                    f.to_string()
                }
            })
            .collect();
        out.push_str(&fields.join(","));
        out.push('\n');
        n += 1;
    }
    if n >= MAX_ROWS {
        out.push_str(&format!("(truncated: {})\n", MAX_ROWS));
    }
    Ok(out)
}

fn sort_op<R: std::io::Read>(
    reader: &mut csv::Reader<R>,
    headers: &[String],
    col: &str,
    desc: bool,
) -> Result<String, xai_tool_runtime::ToolError> {
    if col.is_empty() {
        return Err(csv_err("sort_by required"));
    }
    let Some(idx) = headers.iter().position(|h| h == col) else {
        return Err(csv_err(&format!("column '{col}' not found")));
    };
    let mut recs: Vec<csv::StringRecord> = Vec::new();
    for rec in reader.records().take(MAX_ROWS) {
        recs.push(rec.map_err(|e: csv::Error| csv_err(&e.to_string()))?);
    }
    recs.sort_by(|a, b| {
        let va = a.get(idx).unwrap_or("").trim().to_string();
        let vb = b.get(idx).unwrap_or("").trim().to_string();
        if let (Ok(na), Ok(nb)) = (va.parse::<f64>(), vb.parse::<f64>()) {
            if desc {
                nb.partial_cmp(&na).unwrap_or(std::cmp::Ordering::Equal)
            } else {
                na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal)
            }
        } else if desc {
            vb.cmp(&va)
        } else {
            va.cmp(&vb)
        }
    });
    Ok(format_csv(headers, &recs, None))
}

fn aggregate_csv_op<R: std::io::Read>(
    reader: &mut csv::Reader<R>,
    headers: &[String],
    op: &str,
    agg_col: &str,
    gb: &str,
) -> Result<String, xai_tool_runtime::ToolError> {
    if gb.is_empty() {
        if op == "count" {
            return Ok(reader.records().count().to_string());
        }
        let Some(agg_idx) = headers.iter().position(|h| h == agg_col) else {
            return Err(csv_err(&format!("column '{agg_col}' not found")));
        };
        let mut sum = 0f64;
        let mut cnt = 0u64;
        for rec in reader.records() {
            let rec = rec.map_err(|e: csv::Error| csv_err(&e.to_string()))?;
            if let Ok(v) = rec.get(agg_idx).unwrap_or("").trim().parse::<f64>() {
                sum += v;
                cnt += 1;
            }
        }
        return Ok(if op == "sum" {
            format!("{sum}")
        } else if cnt > 0 {
            format!("{:.4}", sum / cnt as f64)
        } else {
            "0".into()
        });
    }
    let Some(gb_idx) = headers.iter().position(|h| h == gb) else {
        return Err(csv_err(&format!("group_by '{gb}' not found")));
    };
    let agg_idx = if agg_col.is_empty() || op == "count" {
        0
    } else {
        match headers.iter().position(|h| h == agg_col) {
            Some(i) => i,
            None => return Err(csv_err(&format!("column '{agg_col}' not found"))),
        }
    };
    let mut groups: BTreeMap<String, (f64, u64)> = BTreeMap::new();
    for rec in reader.records() {
        let rec = rec.map_err(|e: csv::Error| csv_err(&e.to_string()))?;
        let key = rec.get(gb_idx).unwrap_or("").trim().to_string();
        let e = groups.entry(key).or_insert((0.0, 0));
        if op == "count" {
            e.1 += 1;
        } else if let Ok(v) = rec.get(agg_idx).unwrap_or("").trim().parse::<f64>() {
            e.0 += v;
            e.1 += 1;
        }
    }
    let mut out = format!("{},result\n", gb);
    for (k, (sv, cnt)) in groups {
        let r = if op == "count" {
            format!("{cnt}")
        } else if op == "sum" {
            format!("{sv}")
        } else if cnt > 0 {
            format!("{:.4}", sv / cnt as f64)
        } else {
            "0".into()
        };
        out.push_str(&format!("{k},{r}\n"));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::resources::Resources;
    use crate::types::tool_metadata::test_ctx;

    const CSV: &str = "name,age,city\nAlice,30,NYC\nBob,25,LA\nCarol,35,NYC\n";

    async fn run(input: CsvOpsInput) -> Result<CsvOpsOutput, xai_tool_runtime::ToolError> {
        let r = Resources::new();
        xai_tool_runtime::Tool::run(&CsvOpsTool, test_ctx(r.into_shared()), input).await
    }

    #[tokio::test]
    async fn schema() {
        let r = run(CsvOpsInput {
            csv_text: CSV.into(),
            operation: "schema".into(),
            condition: None,
            columns: None,
            sort_by: None,
            sort_desc: None,
            aggregate_on: None,
            group_by: None,
            delimiter: None,
            has_header: true,
        })
        .await
        .unwrap();
        assert!(r.result.contains("name,string"));
        assert!(r.result.contains("age,integer"));
    }

    #[tokio::test]
    async fn select_op() {
        let r = run(CsvOpsInput {
            csv_text: CSV.into(),
            operation: "select".into(),
            condition: Some("age > 25".into()),
            columns: None,
            sort_by: None,
            sort_desc: None,
            aggregate_on: None,
            group_by: None,
            delimiter: None,
            has_header: true,
        })
        .await
        .unwrap();
        assert!(r.result.contains("Alice"));
    }

    #[tokio::test]
    async fn count() {
        let r = run(CsvOpsInput {
            csv_text: CSV.into(),
            operation: "count".into(),
            condition: None,
            columns: None,
            sort_by: None,
            sort_desc: None,
            aggregate_on: None,
            group_by: None,
            delimiter: None,
            has_header: true,
        })
        .await
        .unwrap();
        assert_eq!(r.result.trim(), "3");
    }

    #[test]
    fn tool_name() {
        assert_eq!(xai_tool_runtime::Tool::id(&CsvOpsTool).as_str(), "csv_ops");
    }
}
fn csv_err(s: &str) -> xai_tool_runtime::ToolError {
    xai_tool_runtime::ToolError::execution(
        xai_tool_protocol::ToolId::new("csv_ops").expect("valid"),
        s.to_string(),
    )
}
