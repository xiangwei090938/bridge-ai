use crate::core::error::Result;
use crate::core::ai_provider::{ProviderConfig, ProviderType, ChatMessage};
use crate::service::provider_mgr::ProviderManager;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationInfo {
    pub id: String,
    pub title: String,
    pub model_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub message_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageInfo {
    pub id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

fn create_provider(config: ProviderConfig) -> std::sync::Arc<dyn crate::core::ai_provider::AiProvider> {
    match config.provider_type {
        ProviderType::DeepSeek => std::sync::Arc::new(crate::core::deepseek::DeepSeekProvider::new(config)),
        ProviderType::Tongyi => std::sync::Arc::new(crate::core::tongyi::TongyiProvider::new(config)),
        ProviderType::Ollama => std::sync::Arc::new(crate::core::ollama::OllamaProvider::new(config)),
        _ => std::sync::Arc::new(crate::core::openai::OpenAIProvider::new(config)),
    }
}

#[tauri::command]
pub async fn send_message(
    conversation_id: String,
    content: String,
    model_id: String,
    provider_id: String,
) -> Result<String> {
    let now = chrono::Utc::now().to_rfc3339();

    // Get model config from JSON file
    let models = ProviderManager::load_models();
    let model = models.into_iter().find(|m| m.internal_id == provider_id)
        .ok_or_else(|| crate::core::error::AppError::NotFound(format!("Model not found: {}", provider_id)))?;

    let base_url = model.base_url;
    let api_key = model.api_key;
    let provider_type_name = model.name.clone();

    // Save user message to SQLite (conversations still in DB)
    {
        let conn = crate::util::db::get_connection("")?;
        let exists: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM conversations WHERE id = ?1",
            rusqlite::params![conversation_id],
            |row| row.get(0),
        ).unwrap_or(false);
        if !exists {
            let title = if content.len() > 30 { format!("{}...", &content[..30]) } else { content.clone() };
            conn.execute(
                "INSERT INTO conversations (id, title, model_id, provider_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![conversation_id, title, model_id, provider_id, now, now],
            )?;
        }
        let user_msg_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO messages (id, conversation_id, role, content, created_at) VALUES (?1, ?2, 'user', ?3, ?4)",
            rusqlite::params![user_msg_id, conversation_id, content, now],
        )?;
    }

    // Fetch conversation history
    let history: Vec<ChatMessage> = {
        let conn = crate::util::db::get_connection("")?;
        let sql = "SELECT role, content FROM messages WHERE conversation_id = ?1 ORDER BY created_at LIMIT 50";
        let mut stmt = conn.prepare(sql)?;
        let mut results: Vec<ChatMessage> = Vec::new();
        let rows = stmt.query_map(rusqlite::params![conversation_id], |row| {
            Ok(ChatMessage { role: row.get(0)?, content: row.get(1)? })
        })?;
        for row in rows {
            results.push(row?);
        }
        results
    };
    // Call AI
    let response_text = if !api_key.is_empty() && !model_id.is_empty() {
        let ptype_enum = match provider_type_name.to_lowercase().as_str() {
            "deepseek" => ProviderType::DeepSeek,
            "tongyi" => ProviderType::Tongyi,
            "ollama" => ProviderType::Ollama,
            _ => ProviderType::OpenAI,
        };
        let pconfig = ProviderConfig {
            id: provider_id.clone(),
            name: String::new(),
            display_name: provider_type_name,
            provider_type: ptype_enum,
            base_url: base_url.clone(),
            is_enabled: true,
        };
        let provider = create_provider(pconfig);
        match provider.chat(&api_key, &history, &model_id).await {
            Ok(resp) => resp.content,
            Err(e) => format!("[AI Error] {}", e),
        }
    } else {
        format!("[Bridge AI] No API key configured for provider {}. Please add your API key in Settings.", provider_id)
    };

    // Save assistant response
    {
        let conn = crate::util::db::get_connection("")?;
        let assistant_msg_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO messages (id, conversation_id, role, content, created_at) VALUES (?1, ?2, 'assistant', ?3, ?4)",
            rusqlite::params![assistant_msg_id, conversation_id, response_text, now],
        )?;
    }

    Ok(response_text)
}

#[tauri::command]
pub fn list_conversations() -> Result<Vec<ConversationInfo>> {
    let conn = crate::util::db::get_connection("")?;
    let mut stmt = conn.prepare(
        "SELECT c.id, c.title, c.model_id, c.created_at, c.updated_at, (SELECT COUNT(*) FROM messages m WHERE m.conversation_id = c.id) as msg_count FROM conversations c WHERE c.is_archived = 0 ORDER BY c.updated_at DESC"
    )?;
    let conversations = stmt.query_map([], |row| {
        Ok(ConversationInfo {
            id: row.get(0)?, title: row.get(1)?,
            model_id: row.get(2)?,
            created_at: row.get(3)?, updated_at: row.get(4)?,
            message_count: row.get(5)?,
        })
    })?.collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(conversations)
}

#[tauri::command]
pub fn get_messages(conversation_id: String) -> Result<Vec<MessageInfo>> {
    let conn = crate::util::db::get_connection("")?;
    let mut stmt = conn.prepare(
        "SELECT id, role, content, created_at FROM messages WHERE conversation_id = ?1 ORDER BY created_at"
    )?;
    let messages = stmt.query_map(rusqlite::params![conversation_id], |row| {
        Ok(MessageInfo {
            id: row.get(0)?, role: row.get(1)?,
            content: row.get(2)?, created_at: row.get(3)?,
        })
    })?.collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(messages)
}
