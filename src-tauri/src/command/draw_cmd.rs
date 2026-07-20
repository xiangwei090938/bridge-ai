use crate::core::error::Result;
use crate::service::ai_draw::{AIDraw, DrawResult};

#[tauri::command]
pub async fn ai_draw(prompt: String, api_key: String, model: String) -> Result<DrawResult> {
    AIDraw::generate(&prompt, &api_key, &model).await
}
