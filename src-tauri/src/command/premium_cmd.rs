use crate::core::error::Result;
use crate::service::premium::{PremiumManager, LicenseInfo};

#[tauri::command]
pub fn check_premium_status() -> Result<LicenseInfo> {
    PremiumManager::check_status()
}

#[tauri::command]
pub fn activate_premium(license_key: String) -> Result<LicenseInfo> {
    PremiumManager::activate(&license_key)
}

#[tauri::command]
pub fn start_premium_trial() -> Result<LicenseInfo> {
    PremiumManager::start_trial()
}
