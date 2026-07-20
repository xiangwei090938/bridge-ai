use crate::core::crypto;
use crate::core::error::Result;
use crate::util::db;
use serde::{Deserialize, Serialize};

/// Get or create a master key using the system keychain (keyring)
/// Falls back to a derived key if keychain is unavailable
fn get_master_key() -> String {
    use keyring::Entry;
    
    // Try to use system keychain first
    let service = "bridge-ai";
    let user = "master-key";
    
    match Entry::new(service, user) {
        Ok(entry) => {
            // Try to retrieve existing key
            match entry.get_password() {
                Ok(key) => key,
                Err(_) => {
                    // Generate a new random master key
                    use ring::rand::{SecureRandom, SystemRandom};
                    let rng = SystemRandom::new();
                    let mut key_bytes = [0u8; 32];
                    rng.fill(&mut key_bytes).expect("Failed to generate random key");
                    let key = base64_encode(&key_bytes);
                    
                    // Store in keychain (ignore errors - will use fallback)
                    let _ = entry.set_password(&key);
                    key
                }
            }
        }
        Err(_) => {
            // Fallback: derive key from machine-specific info
            // This is less secure but allows the app to function
            let machine_id = std::env::var("COMPUTERNAME")
                .or_else(|_| std::env::var("HOSTNAME"))
                .unwrap_or_else(|_| "unknown".to_string());
            format!("bridge-ai-{}-v1", machine_id)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyEntry {
    pub id: String,
    pub provider_id: String,
    pub alias: String,
    pub key_prefix: String,
    pub is_active: bool,
    pub created_at: String,
}

/// KeyVault manages secure storage and retrieval of API keys
pub struct KeyVault {
    db_path: String,
}

impl KeyVault {
    pub fn new(db_path: &str) -> Self {
        Self {
            db_path: db_path.to_string(),
        }
    }

    /// Save an API key (encrypted)
    pub fn save_key(&self, provider_id: &str, alias: &str, api_key: &str) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let master_key = get_master_key();
        let encrypted = crypto::encrypt(api_key, &master_key)?;
        let encrypted_b64 = base64_encode(&encrypted);
        let key_prefix = if api_key.len() > 8 { &api_key[..8] } else { api_key };
        let now = chrono::Utc::now().to_rfc3339();

        let conn = db::get_connection(&self.db_path)?;
        conn.execute(
            "INSERT INTO api_keys (id, provider_id, alias, key_encrypted, key_prefix, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![id, provider_id, alias, encrypted_b64, key_prefix, now, now],
        )?;

        Ok(id)
    }

    /// Retrieve an API key (decrypted)
    pub fn get_key(&self, key_id: &str) -> Result<String> {
        let conn = db::get_connection(&self.db_path)?;
        let encrypted_b64: String = conn.query_row(
            "SELECT key_encrypted FROM api_keys WHERE id = ?1",
            rusqlite::params![key_id],
            |row| row.get(0),
        ).map_err(|_| crate::core::error::AppError::NotFound(format!("Key not found: {}", key_id)))?;

        let encrypted = base64_decode(&encrypted_b64)?;
        let master_key = get_master_key();
        crypto::decrypt(&encrypted, &master_key)
    }

    /// List all API key entries (without exposing the actual key)
    pub fn list_keys(&self, provider_id: &str) -> Result<Vec<ApiKeyEntry>> {
        let conn = db::get_connection(&self.db_path)?;
        let mut stmt = conn.prepare(
            "SELECT id, provider_id, alias, key_prefix, is_active, created_at FROM api_keys WHERE provider_id = ?1"
        )?;

        let keys = stmt.query_map(rusqlite::params![provider_id], |row| {
            Ok(ApiKeyEntry {
                id: row.get(0)?,
                provider_id: row.get(1)?,
                alias: row.get(2)?,
                key_prefix: row.get(3)?,
                is_active: row.get::<_, i32>(4)? != 0,
                created_at: row.get(5)?,
            })
        })?.collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(keys)
    }

    /// Delete an API key
    pub fn delete_key(&self, key_id: &str) -> Result<()> {
        let conn = db::get_connection(&self.db_path)?;
        conn.execute("DELETE FROM api_keys WHERE id = ?1", rusqlite::params![key_id])?;
        Ok(())
    }
}

fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

fn base64_decode(data: &str) -> Result<Vec<u8>> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.decode(data)
        .map_err(|e| crate::core::error::AppError::Encryption(e.to_string()))
}
