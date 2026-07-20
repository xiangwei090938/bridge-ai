use serde::{Deserialize, Serialize};
use crate::core::error::Result;
use crate::util::db;
use chrono::{Utc, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub is_premium: bool,
    pub activated_at: Option<String>,
    pub expires_at: Option<String>,
    pub days_remaining: i64,
}

/// Premium/license management service
pub struct PremiumManager;

impl PremiumManager {
    pub fn new() -> Self { Self }

    /// Check if premium is active
    pub fn check_status() -> Result<LicenseInfo> {
        let conn = db::get_connection("")?;

        let activated_at: Option<String> = conn.query_row(
            "SELECT value FROM app_settings WHERE key = 'premium_activated_at'",
            [], |row| row.get(0),
        ).ok();

        let expires_at: Option<String> = conn.query_row(
            "SELECT value FROM app_settings WHERE key = 'premium_expires_at'",
            [], |row| row.get(0),
        ).ok();

        let is_premium = match &expires_at {
            Some(exp) => {
                if let Ok(exp_date) = chrono::DateTime::parse_from_rfc3339(exp) {
                    exp_date > Utc::now()
                } else { false }
            }
            None => false,
        };

        let days_remaining = match &expires_at {
            Some(exp) => {
                if let Ok(exp_date) = chrono::DateTime::parse_from_rfc3339(exp) {
                    let diff = exp_date.signed_duration_since(Utc::now());
                    std::cmp::max(0, diff.num_days())
                } else { 0 }
            }
            None => 0,
        };

        Ok(LicenseInfo {
            is_premium,
            activated_at,
            expires_at,
            days_remaining,
        })
    }

    /// Activate premium with a license key
    pub fn activate(license_key: &str) -> Result<LicenseInfo> {
        // In production, verify license key against a remote server
        // For MVP, accept a known test key or any properly formatted key
        if license_key.len() < 10 {
            return Err(crate::core::error::AppError::ApiKey("Invalid license key format".into()));
        }

        let now = Utc::now();
        let expires = now + Duration::days(365); // 1 year subscription
        let now_str = now.to_rfc3339();
        let exp_str = expires.to_rfc3339();

        let conn = db::get_connection("")?;
        conn.execute(
            "INSERT INTO app_settings (key, value, updated_at) VALUES ('premium_activated_at', ?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?1, updated_at = ?2",
            rusqlite::params![now_str, now_str],
        )?;
        conn.execute(
            "INSERT INTO app_settings (key, value, updated_at) VALUES ('premium_expires_at', ?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?1, updated_at = ?2",
            rusqlite::params![exp_str, now_str],
        )?;

        Ok(LicenseInfo {
            is_premium: true,
            activated_at: Some(now_str),
            expires_at: Some(exp_str),
            days_remaining: 365,
        })
    }

    /// Start a free trial (30 days for testing, but we'll use 7 for trial)
    pub fn start_trial() -> Result<LicenseInfo> {
        let now = Utc::now();
        let expires = now + Duration::days(7);
        let now_str = now.to_rfc3339();
        let exp_str = expires.to_rfc3339();

        let conn = db::get_connection("")?;
        conn.execute(
            "INSERT INTO app_settings (key, value, updated_at) VALUES ('premium_activated_at', ?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?1, updated_at = ?2",
            rusqlite::params![now_str, now_str],
        )?;
        conn.execute(
            "INSERT INTO app_settings (key, value, updated_at) VALUES ('premium_expires_at', ?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?1, updated_at = ?2",
            rusqlite::params![exp_str, now_str],
        )?;

        Ok(LicenseInfo {
            is_premium: true,
            activated_at: Some(now_str),
            expires_at: Some(exp_str),
            days_remaining: 7,
        })
    }
}
