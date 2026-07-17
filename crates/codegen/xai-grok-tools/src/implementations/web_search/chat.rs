//! Chat Completions API web search backend.
//!
//! NOTE: `web_search_options` only works with OpenAI's search-preview models
//! (`gpt-4o-search-preview`, `gpt-4o-mini-search-preview`). Regular models
//! do NOT support this parameter. The router will fall back to DuckDuckGo
//! if this backend returns an error.
//!
//! API ref: https://platform.openai.com/docs/guides/tools-web-search

use serde::{Deserialize, Serialize};

use crate::types::output::WebSearchOutput;

/// Search via Chat Completions with web_search_options.
/// Only works with search-preview models; others will error and fall back to DDG.
pub async fn search(
    base_url: &str,
    model: &str,
    api_key: &str,
    query: &str,
    allowed_domains: Option<&[String]>,
) -> Result<WebSearchOutput, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("http client: {e}"))?;

    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));

    let request = ChatRequest {
        model: model.to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: query.to_string(),
        }],
        web_search_options: Some(WebSearchOptions {
            search_context_size: Some("medium".to_string()),
        }),
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("request: {e}"))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API returned {}: {body}", response.status()));
    }

    let resp: ChatResponse = response
        .json()
        .await
        .map_err(|e| format!("parse response: {e}"))?;

    let choice = resp
        .choices
        .first()
        .ok_or("no choices in response")?;

    let content = choice.message.content.clone().unwrap_or_default();
    if content.is_empty() {
        return Ok(WebSearchOutput {
            query: query.to_string(),
            content: format!("No search results found for '{}'.", query),
            citations: vec![],
            allowed_domains: allowed_domains.map(|d| d.to_vec()),
            pre_formatted: None,
        });
    }

    // Extract citations from url_citation annotations
    let mut citations = Vec::new();
    if let Some(ref annotations) = choice.message.annotations {
        for ann in annotations {
            if let Some(ref uc) = ann.url_citation {
                citations.push(uc.url.clone());
            }
        }
    }

    Ok(WebSearchOutput {
        query: query.to_string(),
        content,
        citations,
        allowed_domains: allowed_domains.map(|d| d.to_vec()),
        pre_formatted: None,
    })
}

// ── Chat API types ──

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_search_options: Option<WebSearchOptions>,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct WebSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    search_context_size: Option<String>,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatAssistantMessage,
}

#[derive(Deserialize)]
struct ChatAssistantMessage {
    content: Option<String>,
    #[serde(default)]
    annotations: Option<Vec<UrlAnnotation>>,
}

#[derive(Deserialize)]
struct UrlAnnotation {
    #[serde(rename = "url_citation")]
    url_citation: Option<UrlCitation>,
}

#[derive(Deserialize)]
struct UrlCitation {
    url: String,
    #[serde(default)]
    title: Option<String>,
}
