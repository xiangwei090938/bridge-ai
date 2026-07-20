use crate::core::error::Result;
use crate::core::ai_provider::{AiProvider, ProviderType, ProviderConfig, ModelInfo, ChatMessage, ChatResponse};

pub struct DeepSeekProvider {
    config: ProviderConfig,
}

impl DeepSeekProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl AiProvider for DeepSeekProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::DeepSeek }
    fn config(&self) -> &ProviderConfig { &self.config }

    async fn test_connection(&self, api_key: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let url = format!("{}/models", self.config.base_url.trim_end_matches('/'));
        let resp = client.get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .timeout(std::time::Duration::from_secs(10)).send().await?;
        if resp.status().is_success() { Ok("Connection successful".into()) }
        else {
            let s = resp.status();
            let b = resp.text().await.unwrap_or_default();
            Err(crate::core::error::AppError::ApiKey(format!("Error ({}): {}", s, b)))
        }
    }

    async fn list_models(&self, api_key: &str) -> Result<Vec<ModelInfo>> {
        let client = reqwest::Client::new();
        let url = format!("{}/models", self.config.base_url.trim_end_matches('/'));
        let resp = client.get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .timeout(std::time::Duration::from_secs(15)).send().await?;
        let body: serde_json::Value = resp.json().await?;
        let mut models = Vec::new();
        if let Some(data) = body["data"].as_array() {
            for item in data {
                if let Some(id) = item["id"].as_str() {
                    models.push(ModelInfo {
                        id: uuid::Uuid::new_v4().to_string(),
                        provider_id: self.config.id.clone(),
                        model_id: id.to_string(),
                        display_name: id.to_string(),
                        context_length: 65536,
                        is_default: false,
                    });
                }
            }
        }
        Ok(models)
    }

    async fn chat(&self, api_key: &str, messages: &[ChatMessage], model: &str) -> Result<ChatResponse> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));
        let req_messages: Vec<serde_json::Value> = messages.iter().map(|m| serde_json::json!({"role": m.role, "content": m.content})).collect();
        let body = serde_json::json!({"model": model, "messages": req_messages, "stream": false});
        let resp = client.post(&url).header("Authorization", format!("Bearer {}", api_key)).json(&body).timeout(std::time::Duration::from_secs(60)).send().await?;
        let status = resp.status();
        let data: serde_json::Value = resp.json().await?;
        if !status.is_success() {
            let err_msg = data["error"]["message"].as_str().or_else(|| data["error"].as_str()).unwrap_or("API request failed");
            return Err(crate::core::error::AppError::ApiKey(format!("DeepSeek error ({}): {}", status, err_msg)));
        }
        let content = data["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        Ok(ChatResponse { content, model: model.to_string(), provider: self.config.display_name.clone() })
    }

    async fn chat_stream(&self, api_key: &str, messages: &[ChatMessage], model: &str, tx: tokio::sync::mpsc::UnboundedSender<String>) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));
        let req_messages: Vec<serde_json::Value> = messages.iter().map(|m| serde_json::json!({"role": m.role, "content": m.content})).collect();
        let body = serde_json::json!({"model": model, "messages": req_messages, "stream": true});
        let resp = client.post(&url).header("Authorization", format!("Bearer {}", api_key)).header("Accept", "text/event-stream").json(&body).timeout(std::time::Duration::from_secs(120)).send().await?;
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
                    if line.is_empty() || !line.starts_with("data: ") { continue; }
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
