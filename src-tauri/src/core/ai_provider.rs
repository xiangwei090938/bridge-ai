use serde::{Deserialize, Serialize};
use crate::core::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderType {
    OpenAI,
    DeepSeek,
    Tongyi,
    Ollama,
    Custom(String),
}

impl ProviderType {
    pub fn default_base_url(&self) -> &str {
        match self {
            ProviderType::OpenAI => "https://api.openai.com/v1",
            ProviderType::DeepSeek => "https://api.deepseek.com/v1",
            ProviderType::Tongyi => "https://dashscope.aliyuncs.com/compatible-mode/v1",
            ProviderType::Ollama => "http://localhost:11434/v1",
            ProviderType::Custom(url) => url,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ProviderType::OpenAI => "OpenAI",
            ProviderType::DeepSeek => "DeepSeek",
            ProviderType::Tongyi => "通义千问",
            ProviderType::Ollama => "Ollama",
            ProviderType::Custom(name) => name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub provider_type: ProviderType,
    pub base_url: String,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub provider_id: String,
    pub model_id: String,
    pub display_name: String,
    pub context_length: i64,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,  // "user" | "assistant" | "system"
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub model: String,
    pub provider: String,
}

/// AI Provider trait — all providers must implement this
#[async_trait::async_trait]
pub trait AiProvider: Send + Sync {
    fn provider_type(&self) -> ProviderType;
    fn config(&self) -> &ProviderConfig;

    /// Test API connection with the given key
    async fn test_connection(&self, api_key: &str) -> Result<String>;

    /// List available models from this provider
    async fn list_models(&self, api_key: &str) -> Result<Vec<ModelInfo>>;

    /// Send a chat message and return the response
    async fn chat(&self, api_key: &str, messages: &[ChatMessage], model: &str) -> Result<ChatResponse>;

    /// Send a chat message with streaming support
    async fn chat_stream(
        &self,
        api_key: &str,
        messages: &[ChatMessage],
        model: &str,
        tx: tokio::sync::mpsc::UnboundedSender<String>,
    ) -> Result<()>;
}
