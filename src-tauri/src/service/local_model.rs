use serde::{Deserialize, Serialize};
use crate::core::error::{Result, AppError};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalModelInfo {
    pub name: String,
    pub size: String,
    pub is_installed: bool,
    pub is_running: bool,
}

/// One-click local model deployment via Ollama
pub struct LocalModelDeploy;

impl LocalModelDeploy {
    pub fn new() -> Self { Self }

    /// Check if Ollama is running
    pub async fn check_ollama() -> Result<bool> {
        let client = reqwest::Client::new();
        match client.get("http://localhost:11434/api/tags").timeout(Duration::from_secs(3)).send().await {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Get list of popular models available for install
    pub fn list_available_models() -> Vec<LocalModelInfo> {
        vec![
            LocalModelInfo { name: "qwen2:7b".into(), size: "4.4GB".into(), is_installed: false, is_running: false },
            LocalModelInfo { name: "qwen2:0.5b".into(), size: "352MB".into(), is_installed: false, is_running: false },
            LocalModelInfo { name: "llama3.2:3b".into(), size: "2.0GB".into(), is_installed: false, is_running: false },
            LocalModelInfo { name: "llama3.2:1b".into(), size: "781MB".into(), is_installed: false, is_running: false },
            LocalModelInfo { name: "deepseek-r1:7b".into(), size: "4.7GB".into(), is_installed: false, is_running: false },
            LocalModelInfo { name: "nomic-embed-text".into(), size: "274MB".into(), is_installed: false, is_running: false },
        ]
    }

    /// Pull a model via Ollama API
    pub async fn pull_model(model_name: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let body = serde_json::json!({"name": model_name});
        let resp = client.post("http://localhost:11434/api/pull")
            .json(&body)
            .timeout(Duration::from_secs(3600))
            .send()
            .await
            .map_err(|_| AppError::ModelConnection("Ollama is not running. Please start Ollama first.".into()))?;

        if resp.status().is_success() {
            Ok(format!("Model {} is being downloaded. Check Ollama terminal for progress.", model_name))
        } else {
            Err(AppError::ModelConnection(format!("Failed to pull model: {}", resp.status())))
        }
    }

    /// Delete a model
    pub async fn delete_model(model_name: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let body = serde_json::json!({"name": model_name});
        let resp = client.post("http://localhost:11434/api/delete")
            .json(&body)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .map_err(|_| AppError::ModelConnection("Ollama is not running".into()))?;

        Ok(format!("Model {} deleted", model_name))
    }
}
