//! `text` — deterministic regex text operations.
//! Uses Rust `regex` crate (linear-time, no catastrophic backtracking).
//!
//! Adapted from Caelum's text tool.

use crate::types::tool::{ToolKind, ToolNamespace};

const MAX_INPUT_LEN: usize = 1_000_000;
const MAX_MATCHES: usize = 10_000;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TextInput {
    #[schemars(description = "Operation: extract, replace, or count")]
    pub mode: String,

    #[schemars(description = "Text to operate on")]
    pub input: String,

    #[schemars(description = "Regex pattern")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    #[schemars(description = "Replacement string ($1/$2 capture references)")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,

    #[schemars(description = "Count unit: chars, words, lines, or matches")]
    #[serde(default = "default_unit")]
    pub unit: String,

    #[schemars(description = "Regex flags: i (case-insensitive), m (multiline), s (dot-all)")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
}

fn default_unit() -> String {
    "chars".into()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextOutput {
    pub result: String,
}
impl xai_tool_runtime::ToolOutput for TextOutput {}

#[derive(Debug, Default)]
pub struct TextTool;

impl crate::types::tool_metadata::ToolMetadata for TextTool {
    fn kind(&self) -> ToolKind {
        ToolKind::Other
    }
    fn tool_namespace(&self) -> ToolNamespace {
        ToolNamespace::GrokBuild
    }
    fn description_template(&self) -> &str {
        "Perform regex operations on text. Modes: extract (find matches with capture groups), \
         replace (substitute with $1/$2 references), count (chars/words/lines/matches). \
         Uses linear-time regex — no ReDoS vulnerability."
    }
}

impl xai_tool_runtime::Tool for TextTool {
    type Args = TextInput;
    type Output = TextOutput;

    fn id(&self) -> xai_tool_protocol::ToolId {
        xai_tool_protocol::ToolId::new("text").expect("valid")
    }
    fn description(
        &self,
        _: &xai_tool_runtime::ListToolsContext,
    ) -> xai_tool_types::ToolDescription {
        xai_tool_types::ToolDescription::new(
            "text",
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
        input: TextInput,
    ) -> Result<TextOutput, xai_tool_runtime::ToolError> {
        let err = |s| {
            xai_tool_runtime::ToolError::execution(
                xai_tool_protocol::ToolId::new("text").expect("valid"),
                s,
            )
        };
        if input.input.len() > MAX_INPUT_LEN {
            return Err(err(format!("input too long (max {} bytes)", MAX_INPUT_LEN)));
        }
        let pattern = input.pattern.as_deref().unwrap_or("");
        let replacement = input.replacement.as_deref().unwrap_or("");
        let flags = input.flags.as_deref().unwrap_or("");

        let result = match input.mode.as_str() {
            "extract" => {
                if pattern.is_empty() {
                    return Err(err("pattern required for extract".to_string()));
                }
                do_extract(&input.input, pattern, flags)
            }
            "replace" => {
                if pattern.is_empty() {
                    return Err(err("pattern required for replace".to_string()));
                }
                do_replace(&input.input, pattern, replacement, flags)
            }
            "count" => do_count(&input.input, &input.unit, pattern, flags),
            _ => {
                return Err(err(format!(
                    "unknown mode '{}'. Use extract, replace, or count",
                    input.mode
                )));
            }
        };
        Ok(TextOutput { result })
    }
}

fn build_regex(pattern: &str, flags: &str) -> Result<regex::Regex, String> {
    let mut p = String::new();
    if flags.contains('i') {
        p.push_str("(?i)");
    }
    if flags.contains('m') {
        p.push_str("(?m)");
    }
    if flags.contains('s') {
        p.push_str("(?s)");
    }
    p.push_str(pattern);
    regex::Regex::new(&p).map_err(|e| format!("Invalid regex: {}", e))
}

fn do_extract(input: &str, pattern: &str, flags: &str) -> String {
    let re = match build_regex(pattern, flags) {
        Ok(r) => r,
        Err(e) => return e,
    };
    let mut matches: Vec<serde_json::Value> = Vec::new();
    for caps in re.captures_iter(input) {
        if matches.len() >= MAX_MATCHES {
            break;
        }
        matches.push(serde_json::json!({
            "text": caps.get(0).map(|m| m.as_str()).unwrap_or(""),
            "groups": caps.iter().skip(1).flatten().map(|m| m.as_str()).collect::<Vec<_>>(),
        }));
    }
    serde_json::json!({"mode":"extract","count":matches.len(),"matches":matches}).to_string()
}

fn do_replace(input: &str, pattern: &str, replacement: &str, flags: &str) -> String {
    let re = match build_regex(pattern, flags) {
        Ok(r) => r,
        Err(e) => return e,
    };
    let result = re.replace_all(input, replacement);
    serde_json::json!({"mode":"replace","result":result.to_string()}).to_string()
}

fn do_count(input: &str, unit: &str, pattern: &str, flags: &str) -> String {
    match unit {
        "chars" => {
            let n = unicode_segmentation::UnicodeSegmentation::graphemes(input, true).count();
            serde_json::json!({"mode":"count","unit":"graphemes","count":n}).to_string()
        }
        "words" => {
            let n = unicode_segmentation::UnicodeSegmentation::unicode_words(input).count();
            serde_json::json!({"mode":"count","unit":"words","count":n}).to_string()
        }
        "lines" => serde_json::json!({"mode":"count","unit":"lines","count":input.lines().count()})
            .to_string(),
        "matches" => {
            if pattern.is_empty() {
                return r#"{"error":"pattern required for count matches"}"#.into();
            }
            let re = match build_regex(pattern, flags) {
                Ok(r) => r,
                Err(e) => return e,
            };
            serde_json::json!({"mode":"count","unit":"matches","count":re.find_iter(input).take(MAX_MATCHES).count()}).to_string()
        }
        _ => format!(
            r#"{{"error":"unknown unit '{}'. Use chars, words, lines, or matches"}}"#,
            unit
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::resources::Resources;
    use crate::types::tool_metadata::test_ctx;

    async fn run(input: TextInput) -> Result<TextOutput, xai_tool_runtime::ToolError> {
        let r = Resources::new();
        xai_tool_runtime::Tool::run(&TextTool, test_ctx(r.into_shared()), input).await
    }

    #[tokio::test]
    async fn extract_digits() {
        let r = run(TextInput {
            mode: "extract".into(),
            input: "a1b2c3".into(),
            pattern: Some(r"\d".into()),
            replacement: None,
            unit: "chars".into(),
            flags: None,
        })
        .await
        .unwrap();
        let v: serde_json::Value = serde_json::from_str(&r.result).unwrap();
        assert_eq!(v["count"].as_u64().unwrap(), 3);
    }

    #[tokio::test]
    async fn replace_text() {
        let r = run(TextInput {
            mode: "replace".into(),
            input: "hello world".into(),
            pattern: Some("world".into()),
            replacement: Some("there".into()),
            unit: "chars".into(),
            flags: None,
        })
        .await
        .unwrap();
        let v: serde_json::Value = serde_json::from_str(&r.result).unwrap();
        assert_eq!(v["result"].as_str().unwrap(), "hello there");
    }

    #[tokio::test]
    async fn count_words() {
        let r = run(TextInput {
            mode: "count".into(),
            input: "hello world test".into(),
            pattern: None,
            replacement: None,
            unit: "words".into(),
            flags: None,
        })
        .await
        .unwrap();
        let v: serde_json::Value = serde_json::from_str(&r.result).unwrap();
        assert_eq!(v["count"].as_u64().unwrap(), 3);
    }

    #[test]
    fn tool_name() {
        assert_eq!(xai_tool_runtime::Tool::id(&TextTool).as_str(), "text");
    }
}
