use crate::core::error::Result;
use crate::service::skill_store::{SkillStore, SkillInfo, InstalledSkill};

#[tauri::command]
pub fn list_available_skills() -> Vec<SkillInfo> {
    SkillStore::list_available()
}

#[tauri::command]
pub fn install_skill(skill_id: String) -> Result<InstalledSkill> {
    let install_dir = crate::util::db::get_skill_dir();
    SkillStore::install_skill(&skill_id, &install_dir)
}

#[tauri::command]
pub fn list_installed_skills() -> Result<Vec<InstalledSkill>> {
    SkillStore::list_installed()
}

#[tauri::command]
pub fn uninstall_skill(skill_id: String) -> Result<()> {
    SkillStore::uninstall_skill(&skill_id)
}
