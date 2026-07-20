use serde::{Deserialize, Serialize};
use crate::core::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

/// Web search integration using SerpAPI/Google Search
pub struct WebSearch;

impl WebSearch {
    pub fn new() -> Self { Self }

    /// Perform a web search
    pub async fn search(query: &str, api_key: &str) -> Result<Vec<SearchResult>> {
        if api_key.is_empty() {
            return Err(crate::core::error::AppError::ApiKey("Search API key not configured".into()));
        }

        let client = reqwest::Client::new();
        let url = format!(
            "https://serpapi.com/search?q={}&api_key={}&num=5",
            urlencoding(&query),
            api_key
        );

        let resp = client.get(&url).timeout(std::time::Duration::from_secs(15)).send().await?;
        let body: serde_json::Value = resp.json().await?;

        let mut results = Vec::new();
        if let Some(organic) = body["organic_results"].as_array() {
            for item in organic {
                results.push(SearchResult {
                    title: item["title"].as_str().unwrap_or("").to_string(),
                    url: item["link"].as_str().unwrap_or("").to_string(),
                    snippet: item["snippet"].as_str().unwrap_or("").to_string(),
                });
            }
        }
        Ok(results)
    }

    /// Get search context for AI (concatenate results into a prompt)
    pub async fn get_search_context(query: &str, api_key: &str) -> Result<String> {
        let results = Self::search(query, api_key).await?;
        let mut context = String::from("Web search results:\n\n");
        for (i, r) in results.iter().enumerate() {
            context.push_str(&format!("{}. **{}**\n   {}\n   Source: {}\n\n", i+1, r.title, r.snippet, r.url));
        }
        Ok(context)
    }
}

fn urlencoding(s: &str) -> String {
    s.chars().map(|c| match c {
        'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
        ' ' => "+".to_string(),
        _ => format!("%{:02X}", c as u8),
    }).collect()
}

