//! Anthropic Messages API web search backend.
//! Used by Claude (native) and DeepSeek (via /anthropic endpoint).
//!
//! API reference: https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool

use serde::{Deserialize, Serialize};

use crate::types::output::WebSearchOutput;

/// Search the web using the Anthropic Messages API with built-in web_search tool.
pub async fn search(
    base_url: &str,
    model: &str,
    api_key: &str,
    query: &str,
    allowed_domains: Option<&[String]>,
) -> Result<WebSearchOutput, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("http client: {e}"))?;

    let url = format!("{}/messages", base_url.trim_end_matches('/'));

    let request = AnthropicRequest {
        model: model.to_string(),
        max_tokens: 4096,
        messages: vec![AnthropicMessage {
            role: "user".to_string(),
            content: vec![AnthropicContentBlock::Text {
                text: query.to_string(),
            }],
        }],
        tools: vec![AnthropicTool {
            tool_type: "web_search_20250305".to_string(),
            name: "web_search".to_string(),
        }],
    };

    let response = client
        .post(&url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("request: {e}"))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API returned {status}: {body}"));
    }

    let resp: AnthropicResponse = response
        .json()
        .await
        .map_err(|e| format!("parse response: {e}"))?;

    // The Anthropic API returns multiple content blocks:
    // 1. text — Claude's thinking about searching
    // 2. server_tool_use — the search query actually used
    // 3. web_search_tool_result — results with url, title, page_age
    // 4. text — final answer with citations
    let mut content = String::new();
    let mut citations = Vec::new();

    for block in &resp.content {
        match block {
            AnthropicResponseContent::Text { text } => {
                if !text.is_empty() {
                    content.push_str(text);
                    content.push('\n');
                }
            }
            AnthropicResponseContent::WebSearchToolResult { content: results } => {
                for item in results {
                    citations.push(item.url.clone());
                    content.push_str(&format!("- [{}]({})\n", item.title, item.url));
                }
            }
            _ => {} // skip server_tool_use blocks
        }
    }

    if content.trim().is_empty() {
        content = format!("No search results found for '{}'.", query);
    }

    Ok(WebSearchOutput {
        query: query.to_string(),
        content: content.trim().to_string(),
        citations,
        allowed_domains: allowed_domains.map(|d| d.to_vec()),
        pre_formatted: None,
    })
}

// ── Anthropic API types ──

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    tools: Vec<AnthropicTool>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    /// Content is an array of content blocks, not a bare string.
    content: Vec<AnthropicContentBlock>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
}

#[derive(Serialize)]
struct AnthropicTool {
    /// "web_search_20250305" — versioned tool type identifier.
    #[serde(rename = "type")]
    tool_type: String,
    name: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicResponseContent>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum AnthropicResponseContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String },
    #[serde(rename = "server_tool_use")]
    ServerToolUse {
        name: String,
        id: String,
        input: serde_json::Value,
    },
    #[serde(rename = "web_search_tool_result")]
    WebSearchToolResult {
        #[serde(default)]
        content: Vec<WebSearchResultItem>,
    },
}

#[derive(Deserialize)]
struct WebSearchResultItem {
    #[serde(default)]
    title: String,
    #[serde(default)]
    url: String,
    #[serde(default, rename = "encrypted_content")]
    _encrypted_content: String,
    #[serde(default)]
    page_age: Option<String>,
}
