//! Bing RSS search backend.
//! Free, no API key required. Uses Bing RSS endpoint which works in China.

use std::time::Duration;

use crate::types::output::WebSearchOutput;

/// Search Bing via RSS and return formatted results.
pub async fn search(
    query: &str,
    allowed_domains: Option<&[String]>,
) -> Result<WebSearchOutput, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| format!("http client: {e}"))?;

    let url = format!(
        "https://www.bing.com/search?q={}&format=rss&count=10",
        url_encode(query)
    );

    let xml = client
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

    // Simple XML parsing for RSS <item> blocks
    let mut pos = 0;
    while let Some(item_start) = xml[pos..].find("<item>") {
        let abs_start = pos + item_start;
        let item_end = xml[abs_start..]
            .find("</item>")
            .unwrap_or(xml.len() - abs_start);
        let item = &xml[abs_start..abs_start + item_end];

        let title = extract_xml(item, "title");
        let link = extract_xml(item, "link");
        let desc = extract_xml(item, "description");

        if !title.is_empty() && !link.is_empty() {
            if let Some(allowed) = allowed_domains
                && !allowed.is_empty()
            {
                let host = link
                    .strip_prefix("https://")
                    .or_else(|| link.strip_prefix("http://"))
                    .and_then(|s| s.split('/').next())
                    .unwrap_or("");
                if !allowed
                    .iter()
                    .any(|d| host == d.as_str() || host.ends_with(&format!(".{d}")))
                {
                    pos = abs_start + item_end + 7;
                    continue;
                }
            }
            parts.push(format!("- **{}**  \n  {}\n  {}", title, desc, link));
            citations.push(link);
        }
        pos = abs_start + item_end + 7;
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

    Ok(WebSearchOutput {
        query: query.to_string(),
        content: parts.join("\n\n"),
        citations,
        allowed_domains: allowed_domains.map(|d| d.to_vec()),
        pre_formatted: None,
    })
}

fn extract_xml(xml: &str, tag: &str) -> String {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    if let Some(start) = xml.find(&open) {
        let after = &xml[start + open.len()..];
        if let Some(end) = after.find(&close) {
            return after[..end]
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#39;", "'")
                .trim()
                .to_string();
        }
    }
    String::new()
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
