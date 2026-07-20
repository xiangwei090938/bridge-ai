use serde::{Deserialize, Serialize};
use crate::core::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct DrawResult {
    pub image_url: String,
    pub revised_prompt: String,
}

/// AI image generation plugin
pub struct AIDraw;

impl AIDraw {
    pub fn new() -> Self { Self }

    /// Generate an image using DALL-E API
    pub async fn generate(prompt: &str, api_key: &str, model: &str) -> Result<DrawResult> {
        let client = reqwest::Client::new();
        let body = serde_json::json!({
            "model": model,
            "prompt": prompt,
            "n": 1,
            "size": "1024x1024"
        });

        let resp = client.post("https://api.openai.com/v1/images/generations")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await?;

        let data: serde_json::Value = resp.json().await?;
        let image_url = data["data"][0]["url"].as_str().unwrap_or("").to_string();
        let revised_prompt = data["data"][0]["revised_prompt"].as_str().unwrap_or("").to_string();

        Ok(DrawResult { image_url, revised_prompt })
    }
}

