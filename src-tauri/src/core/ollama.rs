use crate::core::error::{Result, AppError};
use crate::core::ai_provider::{AiProvider, ProviderType, ProviderConfig, ModelInfo, ChatMessage, ChatResponse};
use std::time::Duration;

pub struct OllamaProvider {
    config: ProviderConfig,
}

impl OllamaProvider {
    pub fn new(config: ProviderConfig) -> Self { Self { config } }
}

#[async_trait::async_trait]
impl AiProvider for OllamaProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::Ollama }
    fn config(&self) -> &ProviderConfig { &self.config }

    async fn test_connection(&self, _api_key: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let url = format!("{}/../api/tags", self.config.base_url.trim_end_matches("/v1"));
        let resp = client.get(&url).timeout(Duration::from_secs(5)).send().await
            .map_err(|e| AppError::ModelConnection(format!("Ollama not running: {}", e)))?;
        if resp.status().is_success() { Ok("Ollama 已连接".into()) }
        else { Err(AppError::ModelConnection("Ollama 连接失败".into())) }
    }

    async fn list_models(&self, _api_key: &str) -> Result<Vec<ModelInfo>> {
        let client = reqwest::Client::new();
        let url = format!("{}/../api/tags", self.config.base_url.trim_end_matches("/v1"));
        let resp = client.get(&url).timeout(Duration::from_secs(5)).send().await?;
        let body: serde_json::Value = resp.json().await?;
        let mut models = Vec::new();
        if let Some(data) = body["models"].as_array() {
            for item in data {
                if let Some(name) = item["name"].as_str() {
                    models.push(ModelInfo {
                        id: uuid::Uuid::new_v4().to_string(),
                        provider_id: self.config.id.clone(),
                        model_id: name.into(),
                        display_name: name.into(),
                        context_length: 4096,
                        is_default: false,
                    });
                }
            }
        }
        Ok(models)
    }

    async fn chat(&self, _api_key: &str, messages: &[ChatMessage], model: &str) -> Result<ChatResponse> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));
        let req_messages: Vec<serde_json::Value> = messages.iter().map(|m| serde_json::json!({"role": m.role, "content": m.content})).collect();
        let body = serde_json::json!({"model": model, "messages": req_messages, "stream": false});
        let resp = client.post(&url).json(&body).timeout(Duration::from_secs(120)).send().await?;
        let status = resp.status();
        let data: serde_json::Value = resp.json().await?;
        if !status.is_success() {
            let err_msg = data["error"]["message"].as_str().or_else(|| data["error"].as_str()).unwrap_or("API request failed");
            return Err(crate::core::error::AppError::ApiKey(format!("{} error ({}): {}", self.config.display_name, status, err_msg)));
        }
        let content = data["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        Ok(ChatResponse { content, model: model.to_string(), provider: self.config.display_name.clone() })
    }

    async fn chat_stream(&self, _api_key: &str, messages: &[ChatMessage], model: &str, tx: tokio::sync::mpsc::UnboundedSender<String>) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));
        let req_messages: Vec<serde_json::Value> = messages.iter().map(|m| serde_json::json!({"role": m.role, "content": m.content})).collect();
        let body = serde_json::json!({"model": model, "messages": req_messages, "stream": true});
        let resp = client.post(&url).header("Accept", "text/event-stream").json(&body).timeout(Duration::from_secs(120)).send().await?;
        let stream = resp.bytes_stream();
        use futures::StreamExt;
        let mut buffer = String::new();
        tokio::pin!(stream);
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));
            loop {
                if let Some(pos) = buffer.find('\n') {
                    let line = buffer[..pos].trim().to_string();
                    buffer = buffer[pos+1..].to_string();
                    if !line.starts_with("data: ") { continue; }
                    let data = &line[6..];
                    if data == "[DONE]" { return Ok(()); }
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(c) = json["choices"][0]["delta"]["content"].as_str() {
                            let _ = tx.send(c.to_string());
                        }
                    }
                } else { break; }
            }
        }
        Ok(())
    }
}