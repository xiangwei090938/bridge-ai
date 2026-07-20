use crate::core::error::Result;

#[tauri::command]
pub fn open_url(url: String) -> Result<()> {
    open::that(&url).map_err(|e| crate::core::error::AppError::Network(format!("Failed to open URL: {}", e)))?;
    Ok(())
}
