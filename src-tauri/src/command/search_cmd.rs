use crate::core::error::Result;
use crate::service::web_search::{WebSearch, SearchResult};

#[tauri::command]
pub async fn web_search(query: String, api_key: String) -> Result<Vec<SearchResult>> {
    WebSearch::search(&query, &api_key).await
}

#[tauri::command]
pub async fn web_search_context(query: String, api_key: String) -> Result<String> {
    WebSearch::get_search_context(&query, &api_key).await
}
