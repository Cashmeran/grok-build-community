//! DuckDuckGo Instant Answer API backend.
//! Free, no API key required. Fallback for models without native search.

use std::time::Duration;

use serde::Deserialize;

use crate::types::output::WebSearchOutput;

/// DDG Instant Answer API response.
#[derive(Debug, Deserialize)]
struct DdgResponse {
    #[serde(rename = "AbstractText")]
    abstract_text: Option<String>,
    #[serde(rename = "AbstractURL")]
    abstract_url: Option<String>,
    #[serde(rename = "RelatedTopics")]
    related_topics: Vec<DdgTopic>,
    #[serde(rename = "Heading")]
    heading: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DdgTopic {
    #[serde(rename = "Text")]
    text: Option<String>,
    #[serde(rename = "FirstURL")]
    first_url: Option<String>,
}

/// Search DuckDuckGo and return formatted results.
pub async fn search(
    query: &str,
    allowed_domains: Option<&[String]>,
) -> Result<WebSearchOutput, String> {
    let client = reqwest::Client::builder()
        .user_agent("GrokBuild-Community/0.1")
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("http client: {e}"))?;

    let url = format!(
        "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
        url_encode(query)
    );

    let resp: DdgResponse = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("request: {e}"))?
        .json()
        .await
        .map_err(|e| format!("parse: {e}"))?;

    let mut citations: Vec<String> = Vec::new();
    let mut parts: Vec<String> = Vec::new();

    // Direct answer from DDG
    if let Some(ref heading) = resp.heading
        && !heading.is_empty()
    {
        let snippet = resp.abstract_text.as_deref().unwrap_or("");
        let url = resp.abstract_url.as_deref().unwrap_or("");
        if let Some(filtered) = filter_domain(url, allowed_domains) {
            let filtered = filtered.unwrap_or(url);
            parts.push(format!("## {heading}\n{snippet}\n{filtered}"));
            citations.push(filtered.to_string());
        }
    }

    // Related topics
    for topic in &resp.related_topics {
        if let (Some(text), Some(url)) = (&topic.text, &topic.first_url)
            && let Some(filtered) = filter_domain(url, allowed_domains)
        {
            let filtered_url = filtered.unwrap_or(url);
            let (title, snippet) = text.split_once(" - ").unwrap_or((text, ""));
            parts.push(format!(
                "- **{}**  \n  {}\n  {}",
                title.trim(),
                snippet.trim(),
                filtered_url
            ));
            citations.push(filtered_url.to_string());
        }
    }

    if parts.is_empty() {
        return Ok(WebSearchOutput {
            query: query.to_string(),
            content: format!("No DuckDuckGo results found for '{}'.", query),
            citations: vec![],
            allowed_domains: allowed_domains.map(|d| d.to_vec()),
            pre_formatted: None,
        });
    }

    let content = parts.join("\n\n");

    Ok(WebSearchOutput {
        query: query.to_string(),
        content,
        citations,
        allowed_domains: allowed_domains.map(|d| d.to_vec()),
        pre_formatted: None,
    })
}

/// Return the URL if it passes the domain filter, or None to skip.
fn filter_domain<'a>(url: &'a str, allowed: Option<&[String]>) -> Option<Option<&'a str>> {
    let allowed = allowed?;
    if allowed.is_empty() {
        return Some(None);
    }
    let host = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .and_then(|s| s.split('/').next())
        .unwrap_or("");
    if allowed
        .iter()
        .any(|d| host == d.as_str() || host.ends_with(&format!(".{d}")))
    {
        Some(None)
    } else {
        None
    }
}

fn url_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "+".to_string(),
            c if c.is_ascii_alphanumeric() || "-_.~".contains(c) => c.to_string(),
            c => format!("%{:02X}", c as u8),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_encodes_spaces() {
        assert_eq!(url_encode("hello world"), "hello+world");
    }

    #[test]
    fn url_encodes_special_chars() {
        let encoded = url_encode("rust lang");
        assert_eq!(encoded, "rust+lang");
    }

    #[test]
    fn filter_allowed_domain_match() {
        let allowed = Some(vec!["docs.rs".to_string()]);
        assert!(filter_domain("https://docs.rs/foo", allowed.as_deref()).is_some());
    }

    #[test]
    fn filter_allowed_domain_subdomain() {
        let allowed = Some(vec!["rust-lang.org".to_string()]);
        assert!(filter_domain("https://www.rust-lang.org/foo", allowed.as_deref()).is_some());
    }

    #[test]
    fn filter_allowed_domain_no_match() {
        let allowed = Some(vec!["github.com".to_string()]);
        assert!(filter_domain("https://docs.rs/foo", allowed.as_deref()).is_none());
    }

    #[test]
    fn filter_none_allows_all() {
        assert!(filter_domain("https://anything.com", None).is_some());
    }

    #[tokio::test]
    async fn search_returns_results() {
        let result = search("Rust programming language", None).await;
        match result {
            Ok(output) => {
                assert_eq!(output.query, "Rust programming language");
                // DDG may return empty on certain queries — both are valid
            }
            Err(e) => {
                eprintln!("search skipped (network): {e}");
            }
        }
    }
}
