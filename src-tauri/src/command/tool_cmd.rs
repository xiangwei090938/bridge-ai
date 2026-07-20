use tauri::State;
use crate::core::error::Result;
use crate::service::tool_detector::{ToolDetector, ToolInfo};
use crate::service::sync_engine::{SyncEngine, SyncResult};
use crate::service::provider_mgr::ProviderManager;
use crate::AppState;

#[tauri::command]
pub async fn scan_tools() -> Vec<ToolInfo> {
    tokio::task::spawn_blocking(move || {
        ToolDetector::scan_all()
    }).await.unwrap_or_default()
}

#[tauri::command]
pub async fn sync_tool_config(tool_id: String, model_internal_id: String) -> Result<SyncResult> {
    let tools = tokio::task::spawn_blocking(move || {
        ToolDetector::scan_all()
    }).await.map_err(|e| crate::core::error::AppError::Internal(e.to_string()))?;
    let tool = tools.into_iter().find(|t| t.id == tool_id)
        .ok_or_else(|| crate::core::error::AppError::NotFound(format!("Tool not found: {}", tool_id)))?;
    let models = ProviderManager::load_models();
    let model = models.into_iter().find(|m| m.internal_id == model_internal_id)
        .ok_or_else(|| crate::core::error::AppError::NotFound(format!("Model not found: {}", model_internal_id)))?;
    let defs = ToolDetector::load_definitions(&crate::util::fs::BridgeConfig::tools_dir());
    let def = defs.into_iter().find(|d| d.id == tool_id)
        .ok_or_else(|| crate::core::error::AppError::NotFound(format!("Tool definition not found: {}", tool_id)))?;
    if let Some(config) = &def.config {
        Ok(SyncEngine::sync_tool(config, &tool.name, &model.api_key, &model.base_url, &model.model_id))
    } else {
        Ok(SyncResult {
            tool_id: tool.id.clone(),
            tool_name: tool.name,
            success: false,
            message: "No config mapping defined for this tool".into(),
            backed_up: false,
        })
    }
}

#[tauri::command]
pub async fn sync_all_tools(model_internal_id: String) -> Vec<SyncResult> {
    let tools = tokio::task::spawn_blocking(move || {
        ToolDetector::scan_all()
    }).await.unwrap_or_default();
    let models = ProviderManager::load_models();
    let model = match models.into_iter().find(|m| m.internal_id == model_internal_id) {
        Some(m) => m,
        None => return vec![]
    };
    let defs = ToolDetector::load_definitions(&crate::util::fs::BridgeConfig::tools_dir());
    tools.iter().filter_map(|tool| {
        let def = defs.iter().find(|d| d.id == tool.id)?;
        if let Some(config) = &def.config {
            Some(SyncEngine::sync_tool(config, &tool.name, &model.api_key, &model.base_url, &model.model_id))
        } else { None }
    }).collect()
}

#[tauri::command]
pub async fn launch_tool(tool_id: String) -> Result<String> {
    let tools = tokio::task::spawn_blocking(move || {
        ToolDetector::scan_all()
    }).await.map_err(|e| crate::core::error::AppError::Internal(e.to_string()))?;
    let tool = tools.into_iter().find(|t| t.id == tool_id)
        .ok_or_else(|| crate::core::error::AppError::NotFound(format!("Tool not found: {}", tool_id)))?;

    // Priority 1: launch_uri (e.g. shell:AppsFolder\\...)
    if let Some(ref uri) = tool.launch_uri {
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const NO_WINDOW: u32 = 0x08000000;
            std::process::Command::new("cmd")
                .args(["/c", "start", "", uri])
                .creation_flags(NO_WINDOW)
                .spawn()
                .map_err(|e| crate::core::error::AppError::Internal(format!("Failed to launch via URI: {}", e)))?;
        }
        #[cfg(not(windows))]
        {
            std::process::Command::new("open")
                .arg(uri)
                .spawn()
                .map_err(|e| crate::core::error::AppError::Internal(format!("Failed to launch via URI: {}", e)))?;
        }
        return Ok(format!("Launched {} via URI: {}", tool.name, uri));
    }

    // Priority 2: install_path (direct exe)
    if let Some(ref path) = tool.install_path {
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const NO_WINDOW: u32 = 0x08000000;
            std::process::Command::new(path)
                .creation_flags(NO_WINDOW)
                .spawn()
                .map_err(|e| crate::core::error::AppError::Internal(format!("Failed to launch: {}", e)))?;
        }
        #[cfg(not(windows))]
        {
            std::process::Command::new(path)
                .spawn()
                .map_err(|e| crate::core::error::AppError::Internal(format!("Failed to launch: {}", e)))?;
        }
        return Ok(format!("Launched {} from: {}", tool.name, path));
    }

    Err(crate::core::error::AppError::Internal("No launch method available for this tool".into()))
}

#[tauri::command]
pub async fn install_tool(tool_name: String) -> Result<String> {
    let result = tokio::task::spawn_blocking(move || {
        let defs = ToolDetector::load_definitions(&crate::util::fs::BridgeConfig::tools_dir());
        let def = defs.into_iter().find(|d| d.id == tool_name || d.paths.name.to_lowercase() == tool_name.to_lowercase())?;
        let exe_name = def.paths.win32.first()?;
        let path = std::path::Path::new(exe_name);
        let dir = path.parent()?;
        Some(format!("Detected: {} at {}", def.paths.name, dir.to_string_lossy()))
    }).await;
    match result {
        Ok(Some(msg)) => Ok(msg),
        Ok(None) => Err(crate::core::error::AppError::Internal("Tool install path not found. Try downloading from the tool website.".into())),
        Err(e) => Err(crate::core::error::AppError::Internal(format!("Install failed: {}", e)))
    }
}
