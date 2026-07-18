//! Bing search backend.
//! Free, no API key required. Works without proxy in mainland China.

use std::time::Duration;

use regex::Regex;

use crate::types::output::WebSearchOutput;

/// Search Bing and return formatted results.
pub async fn search(
    query: &str,
    allowed_domains: Option<&[String]>,
) -> Result<WebSearchOutput, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| format!("http client: {e}"))?;

    let url = format!(
        "https://www.bing.com/search?q={}&setlang=en&count=10",
        url_encode(query)
    );

    let html = client
        .get(&url)
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
        .map_err(|e| format!("request: {e}"))?
        .text()
        .await
        .map_err(|e| format!("read body: {e}"))?;

    let mut citations: Vec<String> = Vec::new();
    let mut parts: Vec<String> = Vec::new();

    // Parse Bing result blocks — each result is in <li class="b_algo">
    let algo_re = Regex::new(
        r#"<li class="b_algo"[^>]*>[\s\S]*?</li>"#
    ).map_err(|e| format!("regex: {e}"))?;

    let title_re = Regex::new(r#"<h2[^>]*><a[^>]*href="([^"]*)"[^>]*>([\s\S]*?)</a></h2>"#)
        .map_err(|e| format!("regex: {e}"))?;

    let snippet_re = Regex::new(r#"<p[^>]*>([\s\S]*?)</p>"#)
        .map_err(|e| format!("regex: {e}"))?;

    for cap in algo_re.captures_iter(&html) {
        let block = cap.get(0).map(|m| m.as_str()).unwrap_or("");
        if let Some(tc) = title_re.captures(block) {
            let raw_url = tc.get(1).map(|m| m.as_str()).unwrap_or("");
            let title_html = tc.get(2).map(|m| m.as_str()).unwrap_or("");

            // Decode HTML entities in title
            let title = strip_html(title_html);

            // Extract snippet
            let snippet = snippet_re
                .captures(block)
                .and_then(|c| c.get(1))
                .map(|m| strip_html(m.as_str()))
                .unwrap_or_default();

            if title.is_empty() || raw_url.is_empty() {
                continue;
            }

            // Domain filter
            let display_url = match filter_domain(raw_url, allowed_domains) {
                Some(u) => u.unwrap_or(raw_url),
                None => continue,
            };

            parts.push(format!("- **{}**  \n  {}\n  {}", title, snippet, display_url));
            citations.push(display_url.to_string());
        }
    }

    if parts.is_empty() {
        return Ok(WebSearchOutput {
            query: query.to_string(),
            content: format!("No Bing results found for '{}'.", query),
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

/// Strip HTML tags and decode common entities.
fn strip_html(s: &str) -> String {
    let s = s
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&#x27;", "'");
    // Remove remaining HTML tags
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(&s, "").trim().to_string()
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
    fn strip_html_removes_tags() {
        assert_eq!(
            strip_html("<strong>Rust</strong> programming language"),
            "Rust programming language"
        );
    }

    #[test]
    fn strip_html_decodes_entities() {
        assert_eq!(strip_html("if a &lt; b &amp;&amp; c &gt; d"), "if a < b && c > d");
    }

    #[test]
    fn filter_allowed_domain_match() {
        let allowed = Some(vec!["docs.rs".to_string()]);
        assert!(filter_domain("https://docs.rs/foo", allowed.as_deref()).is_some());
    }

    #[test]
    fn filter_none_allows_all() {
        assert!(filter_domain("https://anything.com", None).is_some());
    }
}
