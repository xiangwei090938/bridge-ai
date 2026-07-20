use crate::core::error::Result;
use crate::core::ai_provider::{AiProvider, ProviderType, ProviderConfig, ModelInfo, ChatMessage, ChatResponse};

pub struct OpenAIProvider {
    config: ProviderConfig,
}

impl OpenAIProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl AiProvider for OpenAIProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::OpenAI }
    fn config(&self) -> &ProviderConfig { &self.config }

    async fn test_connection(&self, api_key: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let url = format!("{}/models", self.config.base_url.trim_end_matches('/'));
        let resp = client.get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .timeout(std::time::Duration::from_secs(10))
            .send().await?;
        if resp.status().is_success() { Ok("Connection successful".into()) }
        else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(crate::core::error::AppError::ApiKey(format!("Error ({}): {}", status, body)))
        }
    }

    async fn list_models(&self, api_key: &str) -> Result<Vec<ModelInfo>> {
        let client = reqwest::Client::new();
        let url = format!("{}/models", self.config.base_url.trim_end_matches('/'));
        let resp = client.get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .timeout(std::time::Duration::from_secs(15))
            .send().await?;
        let body: serde_json::Value = resp.json().await?;
        let mut models = Vec::new();
        if let Some(data) = body["data"].as_array() {
            for item in data {
                if let (Some(id), Some(name)) = (item["id"].as_str(), item.get("id").and_then(|x| x.as_str())) {
                    models.push(ModelInfo {
                        id: uuid::Uuid::new_v4().to_string(),
                        provider_id: self.config.id.clone(),
                        model_id: id.to_string(),
                        display_name: id.to_string(),
                        context_length: 4096,
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
        let req_messages: Vec<serde_json::Value> = messages.iter().map(|m| {
            serde_json::json!({"role": m.role, "content": m.content})
        }).collect();
        let body = serde_json::json!({
            "model": model,
            "messages": req_messages,
            "stream": false
        });
        let resp = client.post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .timeout(std::time::Duration::from_secs(60))
            .send().await?;
        let status = resp.status();
        let data: serde_json::Value = resp.json().await?;
        if !status.is_success() {
            let err_msg = data["error"]["message"].as_str().or_else(|| data["error"].as_str()).unwrap_or("API request failed");
            return Err(crate::core::error::AppError::ApiKey(format!("{} error ({}): {}", self.config.display_name, status, err_msg)));
        }
        let content = data["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        Ok(ChatResponse {
            content,
            model: model.to_string(),
            provider: self.config.display_name.clone(),
        })
    }

    async fn chat_stream(&self, api_key: &str, messages: &[ChatMessage], model: &str, tx: tokio::sync::mpsc::UnboundedSender<String>) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));
        let req_messages: Vec<serde_json::Value> = messages.iter().map(|m| {
            serde_json::json!({"role": m.role, "content": m.content})
        }).collect();
        let body = serde_json::json!({
            "model": model,
            "messages": req_messages,
            "stream": true
        });
        let resp = client.post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Accept", "text/event-stream")
            .json(&body)
            .timeout(std::time::Duration::from_secs(120))
            .send().await?;

        let stream = resp.bytes_stream();
        use futures::StreamExt;
        let mut buffer = String::new();
        tokio::pin!(stream);

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);
            buffer.push_str(&text);
            while let Some(line_end) = buffer.find('\n') {
                let line = buffer[..line_end].trim().to_string();
                buffer = buffer[line_end + 1..].to_string();
                if line.is_empty() { continue; }
                if line.starts_with("data: ") {
                    let data = &line[6..];
                    if data == "[DONE]" { break; }
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            let _ = tx.send(content.to_string());
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
