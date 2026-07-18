//! Web search router — auto-detects backend from model api_backend, falls back to Bing.

use crate::types::output::WebSearchOutput;

use super::WebSearchConfig;

/// Route a search query. Tries native search first, falls back to Bing.
pub async fn search(
    config: &WebSearchConfig,
    query: &str,
    allowed_domains: Option<Vec<String>>,
) -> Result<WebSearchOutput, xai_tool_runtime::ToolError> {
    let (api_key, base_url, model, api_backend) = match config {
        WebSearchConfig::Disabled => {
            // Community Edition: fall through to Bing free search
            return super::bing::search(query, allowed_domains.as_deref())
                .await
                .map_err(|e| {
                    xai_tool_runtime::ToolError::execution(
                        xai_tool_protocol::ToolId::new("web_search").expect("valid"),
                        format!("DDG search failed: {e}"),
                    )
                });
        }
        WebSearchConfig::Enabled {
            api_key,
            base_url,
            model,
            api_backend,
            ..
        } => (api_key, base_url, model, api_backend),
    };

    // 1. Try native search based on model's api_backend
    let native_result = match api_backend.as_str() {
        "responses" => {
            let inner = super::WebSearchConfig::Enabled {
                api_key: api_key.clone(),
                base_url: base_url.clone(),
                model: model.clone(),
                api_backend: "responses".to_string(),
                extra_headers: Default::default(),
                alpha_test_key: None,
            };
            let client = super::client::WebSearchClient::new(&inner, None)?;
            client
                .search(query, allowed_domains.clone())
                .await
                .map(|(content, citations)| WebSearchOutput {
                    query: query.to_string(),
                    content,
                    citations,
                    allowed_domains: allowed_domains.clone(),
                    pre_formatted: None,
                })
        }
        "messages" => {
            super::messages::search(base_url, model, api_key, query, allowed_domains.as_deref())
                .await
                .map_err(|e| {
                    xai_tool_runtime::ToolError::execution(
                        xai_tool_protocol::ToolId::new("web_search").expect("valid"),
                        e,
                    )
                })
        }
        "chat_completions" => {
            super::chat::search(base_url, model, api_key, query, allowed_domains.as_deref())
                .await
                .map_err(|e| {
                    xai_tool_runtime::ToolError::execution(
                        xai_tool_protocol::ToolId::new("web_search").expect("valid"),
                        e,
                    )
                })
        }
        _ => Err(xai_tool_runtime::ToolError::execution(
            xai_tool_protocol::ToolId::new("web_search").expect("valid"),
            format!("Unknown api_backend: {api_backend}"),
        )),
    };

    // 2. If native search succeeded, return it
    if native_result.is_ok() {
        return native_result;
    }

    // 3. Fall back to Bing
    super::bing::search(query, allowed_domains.as_deref())
        .await
        .map_err(|e| {
            xai_tool_runtime::ToolError::execution(
                xai_tool_protocol::ToolId::new("web_search").expect("valid"),
                format!("Native search failed, DDG fallback also failed: {e}"),
            )
        })
}
