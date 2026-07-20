use std::path::{Path, PathBuf};
use std::fs;

pub struct BridgeConfig;

impl BridgeConfig {
    /// 获取应用主目录（跨平台）
    /// Windows: %USERPROFILE%\.bridge-ai
    /// macOS:   ~/Library/Application Support/bridge-ai
    /// Linux:   ~/.config/bridge-ai
    pub fn home_dir() -> PathBuf {
        if cfg!(target_os = "macos") {
            if let Some(base) = dirs::data_dir() {
                return base.join("bridge-ai");
            }
        } else if cfg!(target_os = "linux") {
            if let Some(base) = dirs::config_dir() {
                return base.join("bridge-ai");
            }
        }
        // Windows 或 fallback
        let home = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".bridge-ai")
    }

    pub fn tools_dir() -> PathBuf { Self::home_dir().join("tools") }
    pub fn config_dir() -> PathBuf { Self::home_dir().join("config") }
    pub fn cache_dir() -> PathBuf {
        if let Some(cache) = dirs::cache_dir() {
            return cache.join("bridge-ai");
        }
        Self::home_dir().join("cache")
    }
    pub fn skills_dir() -> PathBuf { Self::home_dir().join("skills") }
    pub fn datalog_dir() -> PathBuf { Self::home_dir().join("datalog") }
    pub fn db_path() -> PathBuf { Self::home_dir().join("bridge-ai.db") }
    pub fn models_json_path() -> PathBuf { Self::config_dir().join("models.json") }
    pub fn model_directory_path() -> PathBuf { Self::cache_dir().join("model-directory.json") }
    pub fn settings_path() -> PathBuf { Self::home_dir().join("settings.json") }

    pub fn init() -> std::io::Result<()> {
        fs::create_dir_all(Self::home_dir())?;
        fs::create_dir_all(Self::tools_dir())?;
        fs::create_dir_all(Self::config_dir())?;
        fs::create_dir_all(Self::cache_dir())?;
        fs::create_dir_all(Self::skills_dir())?;
        fs::create_dir_all(Self::datalog_dir())?;
        Ok(())
    }

    pub fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Option<T> {
        if !path.exists() { return None; }
        fs::read_to_string(path).ok().and_then(|s| serde_json::from_str(&s).ok())
    }

    pub fn write_json<T: serde::Serialize>(path: &Path, data: &T) -> std::io::Result<()> {
        if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, json)
    }
}
