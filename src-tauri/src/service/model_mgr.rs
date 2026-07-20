use serde::{Deserialize, Serialize};
use crate::core::error::Result;
use crate::util::fs::BridgeConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModel {
    #[serde(rename = "internalId")]
    pub internal_id: String,
    pub name: String,
    #[serde(rename = "modelId")]
    pub model_id: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "anthropicUrl", skip_serializing_if = "Option::is_none")]
    pub anthropic_url: Option<String>,
    #[serde(rename = "type")]
    pub model_type: String,
}

pub struct ModelManager;

impl ModelManager {
    pub fn new() -> Self { Self }

    pub fn list_models() -> Vec<UserModel> {
        BridgeConfig::read_json(&BridgeConfig::models_json_path()).unwrap_or_default()
    }

    pub fn add_model(model: UserModel) -> Result<()> {
        let mut models = Self::list_models();
        models.push(model);
        Ok(BridgeConfig::write_json(&BridgeConfig::models_json_path(), &models)?)
    }

    pub fn update_model(internal_id: &str, model: UserModel) -> Result<()> {
        let mut models = Self::list_models();
        if let Some(pos) = models.iter().position(|m| m.internal_id == internal_id) {
            models[pos] = model;
        }
        Ok(BridgeConfig::write_json(&BridgeConfig::models_json_path(), &models)?)
    }

    pub fn delete_model(internal_id: &str) -> Result<()> {
        let mut models = Self::list_models();
        models.retain(|m| m.internal_id != internal_id);
        Ok(BridgeConfig::write_json(&BridgeConfig::models_json_path(), &models)?)
    }

    pub fn get_model(internal_id: &str) -> Option<UserModel> {
        Self::list_models().into_iter().find(|m| m.internal_id == internal_id)
    }

    pub fn update_api_key(internal_id: &str, api_key: &str) -> Result<()> {
        let mut models = Self::list_models();
        if let Some(model) = models.iter_mut().find(|m| m.internal_id == internal_id) {
            model.api_key = api_key.to_string();
        }
        Ok(BridgeConfig::write_json(&BridgeConfig::models_json_path(), &models)?)
    }
}
