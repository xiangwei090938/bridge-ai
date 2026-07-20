use crate::core::error::Result;
use crate::service::local_model::{LocalModelDeploy, LocalModelInfo};

#[tauri::command]
pub async fn check_ollama() -> Result<bool> {
    LocalModelDeploy::check_ollama().await
}

#[tauri::command]
pub fn list_models_catalog() -> Vec<LocalModelInfo> {
    LocalModelDeploy::list_available_models()
}

#[tauri::command]
pub async fn pull_local_model(model_name: String) -> Result<String> {
    LocalModelDeploy::pull_model(&model_name).await
}

#[tauri::command]
pub async fn delete_local_model(model_name: String) -> Result<String> {
    LocalModelDeploy::delete_model(&model_name).await
}
