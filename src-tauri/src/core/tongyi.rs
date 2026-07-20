use crate::core::error::Result;
use crate::core::ai_provider::{AiProvider, ProviderType, ProviderConfig, ModelInfo, ChatMessage, ChatResponse};
use std::time::Duration;

pub struct TongyiProvider {
    config: ProviderConfig,
}

impl TongyiProvider {
    pub fn new(config: ProviderConfig) -> Self { Self { config } }
}

#[async_trait::async_trait]
impl AiProvider for TongyiProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::Tongyi }
    fn config(&self) -> &ProviderConfig { &self.config }

    async fn test_connection(&self, api_key: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));
        let body = serde_json::json!({"model": "qwen-turbo", "messages": [{"role": "user", "content": "test"}], "max_tokens": 1});
        let resp = client.post(&url).header("Authorization", format!("Bearer {}", api_key)).json(&body).timeout(Duration::from_secs(10)).send().await?;
        if resp.status().is_success() || resp.status().as_u16() == 400 { Ok("Connection successful".into()) }
        else { Err(crate::core::error::AppError::ApiKey(format!("Error: {}", resp.status()))) }
    }

    async fn list_models(&self, _api_key: &str) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo { id: uuid::Uuid::new_v4().to_string(), provider_id: self.config.id.clone(), model_id: "qwen-turbo".into(), display_name: "Qwen Turbo".into(), context_length: 8192, is_default: true },
            ModelInfo { id: uuid::Uuid::new_v4().to_string(), provider_id: self.config.id.clone(), model_id: "qwen-plus".into(), display_name: "Qwen Plus".into(), context_length: 131072, is_default: false },
            ModelInfo { id: uuid::Uuid::new_v4().to_string(), provider_id: self.config.id.clone(), model_id: "qwen-max".into(), display_name: "Qwen Max".into(), context_length: 32768, is_default: false },
        ])
    }

    async fn chat(&self, api_key: &str, messages: &[ChatMessage], model: &str) -> Result<ChatResponse> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));
        let req_messages: Vec<serde_json::Value> = messages.iter().map(|m| serde_json::json!({"role": m.role, "content": m.content})).collect();
        let body = serde_json::json!({"model": model, "messages": req_messages});
        let resp = client.post(&url).header("Authorization", format!("Bearer {}", api_key)).json(&body).timeout(Duration::from_secs(60)).send().await?;
        let status = resp.status();
        let data: serde_json::Value = resp.json().await?;
        if !status.is_success() {
            let err_msg = data["error"]["message"].as_str().or_else(|| data["error"].as_str()).unwrap_or("API request failed");
            return Err(crate::core::error::AppError::ApiKey(format!("{} error ({}): {}", self.config.display_name, status, err_msg)));
        }
        let content = data["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        Ok(ChatResponse { content, model: model.to_string(), provider: self.config.display_name.clone() })
    }

    async fn chat_stream(&self, api_key: &str, messages: &[ChatMessage], model: &str, tx: tokio::sync::mpsc::UnboundedSender<String>) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));
        let req_messages: Vec<serde_json::Value> = messages.iter().map(|m| serde_json::json!({"role": m.role, "content": m.content})).collect();
        let body = serde_json::json!({"model": model, "messages": req_messages, "stream": true});
        let resp = client.post(&url).header("Authorization", format!("Bearer {}", api_key)).header("Accept", "text/event-stream").json(&body).timeout(Duration::from_secs(120)).send().await?;
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
