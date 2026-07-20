use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use crate::core::error::Result;
use crate::util::fs::BridgeConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderEntry {
    pub name: String,
    pub url: Option<String>,
    #[serde(rename = "baseUrl")]
    pub base_url: Option<String>,
    #[serde(rename = "anthropicUrl")]
    pub anthropic_url: Option<String>,
    #[serde(rename = "modelId")]
    pub model_id: Option<String>,
    #[serde(rename = "modelIds")]
    pub model_ids: Option<Vec<String>>,
    pub region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDirectory {
    pub providers: Vec<ProviderEntry>,
}

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
    #[serde(rename = "anthropicUrl")]
    pub anthropic_url: Option<String>,
    #[serde(rename = "type")]
    pub model_type: String,
}

pub struct ProviderManager;
impl ProviderManager {
    pub fn load_directory() -> Option<ProviderDirectory> {
        BridgeConfig::read_json(&BridgeConfig::model_directory_path())
    }
    pub fn save_directory(dir: &ProviderDirectory) -> Result<()> {
        Ok(BridgeConfig::write_json(&BridgeConfig::model_directory_path(), dir)?)
    }
    pub fn load_models() -> Vec<UserModel> {
        BridgeConfig::read_json(&BridgeConfig::models_json_path()).unwrap_or_default()
    }
    pub fn save_models(models: &Vec<UserModel>) -> Result<()> {
        Ok(BridgeConfig::write_json(&BridgeConfig::models_json_path(), models)?)
    }
    pub fn add_model(model: UserModel) -> Result<()> {
        let mut models = Self::load_models();
        models.push(model);
        Self::save_models(&models)
    }
    pub fn update_model(internal_id: &str, model: UserModel) -> Result<()> {
        let mut models = Self::load_models();
        if let Some(pos) = models.iter().position(|m| m.internal_id == internal_id) {
            models[pos] = model;
        }
        Self::save_models(&models)
    }
    pub fn delete_model(internal_id: &str) -> Result<()> {
        let mut models = Self::load_models();
        models.retain(|m| m.internal_id != internal_id);
        Self::save_models(&models)
    }
    pub fn get_model(internal_id: &str) -> Option<UserModel> {
        Self::load_models().into_iter().find(|m| m.internal_id == internal_id)
    }
}

pub fn default_provider_directory() -> ProviderDirectory {
    ProviderDirectory { providers: vec![
        ProviderEntry {
            name: "\u{706b}\u{5c71}\u{5f15}\u{64ce}".into(),
            url: Some("https://www.volcengine.com".into()),
            base_url: Some("https://ark.cn-beijing.volces.com/api/v3".into()),
            anthropic_url: None,
            model_id: Some("doubao-pro".into()),
            model_ids: Some(vec!["doubao-pro".into(), "doubao-lite".into()]),
            region: Some("cn".into()),
        },
        ProviderEntry {
            name: "ERNIE \u{767e}\u{5ea6}\u{5343}\u{5e06}".into(),
            url: Some("https://console.bce.baidu.com".into()),
            base_url: Some("https://aip.baidubce.com/rpc/2.0/ai_custom/v1".into()),
            anthropic_url: None,
            model_id: Some("ernie-4.0".into()),
            model_ids: Some(vec!["ernie-4.0".into(), "ernie-3.5".into()]),
            region: Some("cn".into()),
        },
        ProviderEntry {
            name: "Qwen \u{963f}\u{91cc}\u{767e}\u{70bc}".into(),
            url: Some("https://bailian.console.alibabacloud.com".into()),
            base_url: Some("https://dashscope.aliyuncs.com/compatible-mode/v1".into()),
            anthropic_url: None,
            model_id: Some("qwen-max".into()),
            model_ids: Some(vec!["qwen-max".into(), "qwen-plus".into(), "qwen-turbo".into()]),
            region: Some("cn".into()),
        },
        ProviderEntry {
            name: "Hunyuan \u{817e}\u{8baf}\u{6df7}\u{5143}".into(),
            url: Some("https://console.cloud.tencent.com".into()),
            base_url: Some("https://api.hunyuan.cloud.tencent.com/v1".into()),
            anthropic_url: None,
            model_id: Some("hunyuan-pro".into()),
            model_ids: Some(vec!["hunyuan-pro".into(), "hunyuan-standard".into()]),
            region: Some("cn".into()),
        },
        ProviderEntry {
            name: "Stepfun \u{9636}\u{8dc3}\u{661f}\u{8fb0}".into(),
            url: Some("https://www.stepfun.com".into()),
            base_url: Some("https://api.stepfun.com/v1".into()),
            anthropic_url: None,
            model_id: Some("step-2-16k".into()),
            model_ids: Some(vec!["step-2-16k".into(), "step-1-128k".into()]),
            region: Some("cn".into()),
        },
        ProviderEntry {
            name: "UCloud \u{4f18}\u{4e91}\u{667a}\u{7b97}".into(),
            url: Some("https://passport.compshare.cn".into()),
            base_url: Some("https://api.compshare.cn/v1".into()),
            anthropic_url: None,
            model_id: Some("compshare-chat".into()),
            model_ids: Some(vec!["compshare-chat".into()]),
            region: Some("cn".into()),
        },
        ProviderEntry {
            name: "MiniMax".into(),
            url: Some("https://www.minimax.io".into()),
            base_url: Some("https://api.minimax.chat/v1".into()),
            anthropic_url: None,
            model_id: Some("abab6.5-chat".into()),
            model_ids: Some(vec!["abab6.5-chat".into(), "abab5.5-chat".into()]),
            region: Some("cn".into()),
        },
        ProviderEntry {
            name: "Z.ai".into(),
            url: Some("https://api.z.ai".into()),
            base_url: Some("https://api.z.ai/v1".into()),
            anthropic_url: None,
            model_id: Some("z-ai-chat".into()),
            model_ids: Some(vec!["z-ai-chat".into()]),
            region: Some("global".into()),
        },
        ProviderEntry {
            name: "Kimi Global".into(),
            url: Some("https://platform.kimi.ai".into()),
            base_url: Some("https://api.moonshot.cn/v1".into()),
            anthropic_url: None,
            model_id: Some("moonshot-v1-128k".into()),
            model_ids: Some(vec!["moonshot-v1-128k".into(), "moonshot-v1-32k".into(), "moonshot-v1-8k".into()]),
            region: Some("cn".into()),
        },
        ProviderEntry {
            name: "OpenAI".into(),
            url: Some("https://openai.com".into()),
            base_url: Some("https://api.openai.com/v1".into()),
            anthropic_url: None,
            model_id: Some("gpt-4o".into()),
            model_ids: Some(vec!["gpt-4o".into(), "gpt-4o-mini".into(), "gpt-4-turbo".into()]),
            region: Some("global".into()),
        },
        ProviderEntry {
            name: "Anthropic".into(),
            url: Some("https://anthropic.com".into()),
            base_url: Some("https://api.anthropic.com".into()),
            anthropic_url: Some("https://api.anthropic.com".into()),
            model_id: Some("claude-sonnet-4-20250514".into()),
            model_ids: Some(vec!["claude-sonnet-4-20250514".into(), "claude-haiku-4-20250514".into()]),
            region: Some("global".into()),
        },
        ProviderEntry {
            name: "Google Gemini".into(),
            url: Some("https://gemini.google.com".into()),
            base_url: Some("https://generativelanguage.googleapis.com/v1beta".into()),
            anthropic_url: None,
            model_id: Some("gemini-2.0-flash".into()),
            model_ids: Some(vec!["gemini-2.0-flash".into(), "gemini-1.5-pro".into()]),
            region: Some("global".into()),
        },
        ProviderEntry {
            name: "DeepSeek".into(),
            url: Some("https://www.deepseek.com".into()),
            base_url: Some("https://api.deepseek.com".into()),
            anthropic_url: Some("https://api.deepseek.com/anthropic".into()),
            model_id: Some("deepseek-v4-pro".into()),
            model_ids: Some(vec!["deepseek-v4-pro".into(), "deepseek-v4-flash".into()]),
            region: Some("cn".into()),
        },
        ProviderEntry {
            name: "Ollama".into(),
            url: Some("https://ollama.ai".into()),
            base_url: Some("http://localhost:11434/v1".into()),
            anthropic_url: None,
            model_id: Some("".into()),
            model_ids: None,
            region: Some("global".into()),
        },
    ]}
}

