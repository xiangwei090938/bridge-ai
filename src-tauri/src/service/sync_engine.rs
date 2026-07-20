use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub tool_id: String,
    pub tool_name: String,
    pub success: bool,
    pub message: String,
    pub backed_up: bool,
}

pub struct SyncEngine;
impl SyncEngine {

    pub fn sync_tool(config: &crate::service::tool_detector::ToolConfig,
                     tool_name: &str,
                     api_key: &str,
                     base_url: &str,
                     model_id: &str) -> SyncResult {
        let write_map = match &config.write {
            Some(w) => w.clone(),
            None => return SyncResult {
                tool_id: String::new(),
                tool_name: tool_name.to_string(),
                success: false,
                message: "No write mapping defined in config".into(),
                backed_up: false,
            },
        };

        let config_path = match &config.config_file {
            Some(p) => Self::expand_path(p),
            None => return SyncResult {
                tool_id: String::new(),
                tool_name: tool_name.to_string(),
                success: false,
                message: "No config file path defined".into(),
                backed_up: false,
            },
        };

        let config_path = Path::new(&config_path);
        
        // Create config file and parent directories if they don't exist
        if !config_path.exists() {
            if let Some(parent) = config_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            // Create empty JSON object or appropriate default content
            let default_content = match config.config_format.as_deref().unwrap_or("json") {
                "env" => String::new(),
                _ => "{}".to_string(),
            };
            if let Err(e) = std::fs::write(config_path, &default_content) {
                return SyncResult {
                    tool_id: String::new(),
                    tool_name: tool_name.to_string(),
                    success: false,
                    message: format!("Failed to create config file: {}", e),
                    backed_up: false,
                };
            }
        }

        // Backup original config
        let backup = format!("{}.bridge-ai.bak", config_path.to_string_lossy());
        let backed_up = std::fs::copy(config_path, &backup).is_ok();

        // Build values to write: map key -> actual value
        let mut to_write: HashMap<String, String> = HashMap::new();
        for chunk in write_map.chunks(2) {
            if chunk.len() < 2 { continue; }
            let target = &chunk[0];
            let field = &chunk[1];
            match field.as_str() {
                "model" => { to_write.insert(target.clone(), model_id.to_string()); }
                "baseUrl" => { to_write.insert(target.clone(), base_url.to_string()); }
                "apiKey" => { to_write.insert(target.clone(), api_key.to_string()); }
                _ => { to_write.insert(target.clone(), field.clone()); }
            }
        }

        // Write based on format
        let fmt = config.config_format.as_deref().unwrap_or("json");
        let result = match fmt {
            "env" => Self::write_env(config_path, &to_write),
            _ => Self::write_json(config_path, &to_write),
        };

        match result {
            Ok(_) => SyncResult {
                tool_id: String::new(),
                tool_name: tool_name.to_string(),
                success: true,
                message: format!("Synced {} to {}", tool_name, config_path.display()),
                backed_up,
            },
            Err(e) => SyncResult {
                tool_id: String::new(),
                tool_name: tool_name.to_string(),
                success: false,
                message: format!("Sync failed: {}", e),
                backed_up,
            },
        }
    }

    /// Write values to JSON config file, supporting nested paths like "providers.openai.apiKey"
    fn write_json(path: &Path, values: &HashMap<String, String>) -> Result<(), String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut json: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        
        for (key, value) in values {
            Self::set_nested_value(&mut json, key, value);
        }
        
        let out = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?;
        std::fs::write(path, out).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Set a nested value in JSON using dotted path notation
    /// Examples: "apiKey", "env.OPENAI_API_KEY", "providers.openai.apiKey"
    fn set_nested_value(json: &mut serde_json::Value, path: &str, value: &str) {
        let parts: Vec<&str> = path.split('.').collect();
        
        // Handle env.* prefix
        if parts[0] == "env" && parts.len() > 1 {
            let env_key = parts[1..].join(".");
            // Ensure env object exists
            if json.get("env").is_none() {
                json.as_object_mut().unwrap().insert("env".to_string(), serde_json::json!({}));
            }
            // Set value in env object
            if let Some(env_val) = json.get_mut("env") {
                if let Some(env_obj) = env_val.as_object_mut() {
                    env_obj.insert(env_key, serde_json::Value::String(value.to_string()));
                }
            }
            return;
        }

        // Simple case: single key
        if parts.len() == 1 {
            json.as_object_mut().unwrap().insert(path.to_string(), serde_json::Value::String(value.to_string()));
            return;
        }

        // Nested case: recursively navigate
        let first = parts[0];
        let rest = parts[1..].join(".");
        
        // Ensure the first level object exists
        if json.get(first).is_none() || !json.get(first).map(|v| v.is_object()).unwrap_or(false) {
            json.as_object_mut().unwrap().insert(first.to_string(), serde_json::json!({}));
        }
        
        // Recurse into the nested object
        if let Some(nested) = json.get_mut(first) {
            Self::set_nested_value(nested, &rest, value);
        }
    }

    fn write_env(path: &Path, values: &HashMap<String, String>) -> Result<(), String> {
        let mut content = std::fs::read_to_string(path).unwrap_or_default();
        for (key, value) in values {
            let env_key = if key.starts_with("env.") { &key[4..] } else { key };
            let line = format!("export {}=\"{}\"\n", env_key, value);
            if !content.contains(env_key) {
                content.push_str(&line);
            }
        }
        std::fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn expand_path(p: &str) -> String {
        let mut r = p.to_string();
        // Handle ~/ and $HOME/ prefixes (cross-platform)
        if r.starts_with("~/") || r.starts_with("~\\") {
            if let Ok(h) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
                r = h + &r[1..];
            }
        }
        // Handle $HOME variable
        if let Ok(home) = std::env::var("HOME") {
            r = r.replace("$HOME", &home);
        }
        // Handle $XDG_CONFIG_HOME
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            r = r.replace("$XDG_CONFIG_HOME", &xdg);
        }
        // Windows env vars
        #[cfg(target_os = "windows")]
        {
            for &(var, key) in &[("%LOCALAPPDATA%", "LOCALAPPDATA"),
                ("%APPDATA%", "APPDATA"), ("%USERPROFILE%", "USERPROFILE")]
            { if let Ok(v) = std::env::var(key) { r = r.replace(var, &v); } }
        }
        r
    }

}




