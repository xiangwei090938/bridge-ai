use tauri::State;
use crate::core::error::Result;
use crate::service::provider_mgr::{ProviderManager, ProviderEntry, ProviderDirectory, UserModel};
use crate::AppState;

#[tauri::command]
pub fn list_providers() -> Result<ProviderDirectory> {
    Ok(ProviderManager::load_directory().unwrap_or_else(|| {
        let default = crate::service::provider_mgr::default_provider_directory();
        ProviderManager::save_directory(&default).ok();
        default
    }))
}

#[tauri::command]
pub fn list_models() -> Vec<UserModel> {
    ProviderManager::load_models()
}

#[tauri::command]
pub fn add_model(name: String, model_id: String, base_url: String, api_key: String,
    anthropic_url: Option<String>, model_type: String) -> Result<()> {
    let internal_id = format!("m-{}", uuid::Uuid::new_v4().to_string().split("-").next().unwrap_or("0000"));
    let model = UserModel {
        internal_id, name, model_id, base_url, api_key,
        anthropic_url, model_type,
    };
    ProviderManager::add_model(model)
}

#[tauri::command]
pub fn delete_model(internal_id: String) -> Result<()> {
    ProviderManager::delete_model(&internal_id)
}

#[tauri::command]
pub fn update_api_key(internal_id: String, api_key: String) -> Result<()> {
    let mut models = ProviderManager::load_models();
    if let Some(model) = models.iter_mut().find(|m| m.internal_id == internal_id) {
        model.api_key = api_key;
    }
    ProviderManager::save_models(&models)
}

#[tauri::command]
pub async fn test_connection(provider_name: String, base_url: String, api_key: String) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!("{}/models", base_url.trim_end_matches("/"));
    let resp = client.get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| crate::core::error::AppError::Network(format!("Connection failed: {}", e)))?;
    if resp.status().is_success() {
        Ok("Connection successful".to_string())
    } else {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        Err(crate::core::error::AppError::ApiKey(format!("API error ({}): {}", status, body)))
    }
}
