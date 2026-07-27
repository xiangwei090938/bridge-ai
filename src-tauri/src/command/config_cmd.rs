use tauri::State;
use crate::core::error::Result;
use crate::AppState;

#[tauri::command]
pub fn get_setting(state: State<'_, AppState>, key: String) -> Result<Option<String>> {
    let conn = crate::util::db::get_connection("")?;
    let result = conn.query_row(
        "SELECT value FROM app_settings WHERE key = ?1",
        rusqlite::params![key],
        |row| row.get::<_, String>(0),
    );
    match result {
        Ok(val) => Ok(Some(val)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(crate::core::error::AppError::Database(e.to_string())),
    }
}

#[tauri::command]
pub fn set_setting(state: State<'_, AppState>, key: String, value: String) -> Result<()> {
    let conn = crate::util::db::get_connection("")?;
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO app_settings (key, value, updated_at) VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = ?3",
        rusqlite::params![key, value, now],
    )?;
    Ok(())
}



#[tauri::command]
pub async fn get_proxy_status() -> Result<String> {
    let url = format!("http://127.0.0.1:{}/health", crate::PROXY_PORT);
    match reqwest::Client::builder().no_proxy().timeout(std::time::Duration::from_secs(3)).build() {
        Ok(cl) => match cl.get(&url).send().await {
            Ok(r) if r.status().is_success() => Ok("running".into()),
            Ok(r) => Ok(format!("error: unexpected status {}", r.status())),
            Err(e) => Ok(format!("error: {}", e)),
        },
        Err(e) => Ok(format!("error: {}", e)),
    }
}
