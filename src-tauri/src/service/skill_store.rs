use serde::{Deserialize, Serialize};
use crate::core::error::{Result, AppError};
use crate::util::db;

const API_BASE_URL: &str = "http://localhost:13920/api";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub size_bytes: i64,
    pub downloads: i64,
    pub category: String,
    pub tags: Vec<String>,
    pub requires_premium: bool,
    pub download_url: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledSkill {
    pub id: String,
    pub skill_id: String,
    pub name: String,
    pub version: String,
    pub is_enabled: bool,
    pub install_path: String,
    pub installed_at: String,
}

/// Skill store service - fetches from remote API with local fallback
pub struct SkillStore;

impl SkillStore {
    pub fn new() -> Self { Self }

    /// Fetch skills from API, fall back to local cache
    pub fn list_available() -> Vec<SkillInfo> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build().unwrap_or_default();

        match client.get(format!("{}/skills?limit=100", API_BASE_URL)).send() {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(body) = resp.json::<serde_json::Value>() {
                    if let Some(data) = body["data"].as_array() {
                        let skills: Vec<SkillInfo> = data.iter().filter_map(|item| {
                            Some(SkillInfo {
                                id: item["id"].as_str()?.to_string(),
                                name: item["name"].as_str()?.to_string(),
                                description: item["description"].as_str()?.to_string(),
                                author: item["author"].as_str()?.to_string(),
                                version: item["version"].as_str()?.to_string(),
                                size_bytes: item["size_bytes"].as_i64().unwrap_or(0),
                                downloads: item["downloads"].as_i64().unwrap_or(0),
                                category: item["category_name"].as_str().unwrap_or("").to_string(),
                                tags: item["tags"].as_array().map(|a| a.iter().filter_map(|t| t.as_str().map(String::from)).collect()).unwrap_or_default(),
                                requires_premium: item["requires_premium"].as_i64().unwrap_or(1) != 0,
                                download_url: format!("{}/skills/{}/download", API_BASE_URL, item["id"].as_str().unwrap_or("")),
                                icon: item["icon"].as_str().unwrap_or("📦").to_string(),
                            })
                        }).collect();
                        if !skills.is_empty() { return skills; }
                    }
                }
                fallback_list()
            }
            _ => fallback_list(),
        }
    }

    pub fn install_skill(skill_id: &str, install_dir: &std::path::Path) -> Result<InstalledSkill> {
        // Try to get skill info from API first
        let skill_info = fetch_skill_from_api(skill_id).unwrap_or_else(|| {
            fallback_list().into_iter().find(|s| s.id == skill_id).unwrap()
        });

        let skill_dir = install_dir.join(&skill_info.id);
        std::fs::create_dir_all(&skill_dir)?;

        let manifest = serde_json::json!({
            "name": skill_info.name,
            "version": skill_info.version,
            "author": skill_info.author,
            "description": skill_info.description,
            "category": skill_info.category,
            "tags": skill_info.tags,
            "entry": "main.js",
            "permissions": ["chat:read", "chat:write"],
        });
        std::fs::write(skill_dir.join("manifest.json"), serde_json::to_string_pretty(&manifest)?)?;
        std::fs::write(skill_dir.join("main.js"),
            format!("// {} v{}\n// {}\n\nmodule.exports = {{\n  name: '{}',\n  async execute(context) {{\n    return '{} is ready!';\n  }}\n}};\n",
                skill_info.name, skill_info.version, skill_info.description, skill_info.name, skill_info.name))?;

        let now = chrono::Utc::now().to_rfc3339();
        {
            let conn = db::get_connection("")?;
            conn.execute(
                "INSERT INTO installed_skills (id, skill_name, skill_version, description, author, install_path, is_enabled, size_bytes, installed_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, ?7, ?8, ?8)
                 ON CONFLICT(id) DO UPDATE SET is_enabled = 1, updated_at = ?8",
                rusqlite::params![skill_info.id, skill_info.name, skill_info.version, skill_info.description,
                    skill_info.author, skill_dir.to_string_lossy().to_string(), skill_info.size_bytes, now],
            )?;
        }

        Ok(InstalledSkill {
            id: skill_info.id.clone(),
            skill_id: skill_info.id,
            name: skill_info.name,
            version: skill_info.version,
            is_enabled: true,
            install_path: skill_dir.to_string_lossy().to_string(),
            installed_at: now,
        })
    }

    pub fn list_installed() -> Result<Vec<InstalledSkill>> {
        let conn = db::get_connection("")?;
        let mut stmt = conn.prepare(
            "SELECT id, skill_name, skill_version, description, author, install_path, is_enabled, installed_at
             FROM installed_skills ORDER BY installed_at DESC"
        )?;
        let skills = stmt.query_map([], |row| {
            Ok(InstalledSkill {
                id: row.get(0)?, skill_id: row.get(0)?, name: row.get(1)?,
                version: row.get(2)?, is_enabled: row.get::<_, i32>(6)? != 0,
                install_path: row.get(5)?, installed_at: row.get(7)?,
            })
        })?.collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(skills)
    }

    pub fn uninstall_skill(skill_id: &str) -> Result<()> {
        {
            let conn = db::get_connection("")?;
            conn.execute("DELETE FROM installed_skills WHERE id = ?1", rusqlite::params![skill_id])?;
        }
        let skill_dir = db::get_skill_dir().join(skill_id);
        if skill_dir.exists() { std::fs::remove_dir_all(&skill_dir).ok(); }
        Ok(())
    }
}

fn fetch_skill_from_api(skill_id: &str) -> Option<SkillInfo> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5)).build().ok()?;
    let resp = client.get(format!("{}/skills/{}", API_BASE_URL, skill_id)).send().ok()?;
    if !resp.status().is_success() { return None; }
    let body: serde_json::Value = resp.json().ok()?;
    let item = body["data"].as_object()?;
    Some(SkillInfo {
        id: item["id"].as_str()?.to_string(),
        name: item["name"].as_str()?.to_string(),
        description: item["description"].as_str()?.to_string(),
        author: item["author"].as_str()?.to_string(),
        version: item["version"].as_str()?.to_string(),
        size_bytes: item["size_bytes"].as_i64().unwrap_or(0),
        downloads: item["downloads"].as_i64().unwrap_or(0),
        category: item.get("category_name").and_then(|c| c.as_str()).unwrap_or("").to_string(),
        tags: item["tags"].as_array().map(|a| a.iter().filter_map(|t| t.as_str().map(String::from)).collect()).unwrap_or_default(),
        requires_premium: item["requires_premium"].as_i64().unwrap_or(1) != 0,
        download_url: format!("{}/skills/{}/download", API_BASE_URL, skill_id),
        icon: item["icon"].as_str().unwrap_or("📦").to_string(),
    })
}

fn fallback_list() -> Vec<SkillInfo> {
    vec![
        SkillInfo { id: "writing-assistant".into(), name: "写作助手".into(), description: "AI 写作辅助工具，支持文章润色、续写、摘要、翻译".into(), author: "Bridge AI".into(), version: "1.0.0".into(), size_bytes: 512000, downloads: 1234, category: "写作".into(), tags: vec!["写作".into(),"润色".into(),"翻译".into()], requires_premium: true, download_url: format!("{}/skills/writing-assistant/download", API_BASE_URL), icon: "📝".into() },
        SkillInfo { id: "code-reviewer".into(), name: "代码审查助手".into(), description: "自动审查代码质量、发现潜在 Bug、提供优化建议".into(), author: "Bridge AI".into(), version: "1.0.0".into(), size_bytes: 380000, downloads: 892, category: "编程".into(), tags: vec!["编程".into(),"代码审查".into(),"Bug检测".into()], requires_premium: true, download_url: format!("{}/skills/code-reviewer/download", API_BASE_URL), icon: "💻".into() },
        SkillInfo { id: "translator-pro".into(), name: "专业翻译".into(), description: "多语言互译，支持技术文档、论文、商务邮件等".into(), author: "Bridge AI".into(), version: "1.2.0".into(), size_bytes: 280000, downloads: 1567, category: "翻译".into(), tags: vec!["翻译".into(),"多语言".into()], requires_premium: true, download_url: format!("{}/skills/translator-pro/download", API_BASE_URL), icon: "🌐".into() },
        SkillInfo { id: "study-assistant".into(), name: "学习助手".into(), description: "知识点总结、题目解析、学习计划制定".into(), author: "Bridge AI".into(), version: "1.0.0".into(), size_bytes: 420000, downloads: 2103, category: "学习".into(), tags: vec!["学习".into(),"教育".into()], requires_premium: true, download_url: format!("{}/skills/study-assistant/download", API_BASE_URL), icon: "📚".into() },
        SkillInfo { id: "data-analyzer".into(), name: "数据分析师".into(), description: "数据可视化、趋势分析、报表生成".into(), author: "Bridge AI".into(), version: "1.0.0".into(), size_bytes: 650000, downloads: 678, category: "工具".into(), tags: vec!["数据分析".into(),"可视化".into()], requires_premium: true, download_url: format!("{}/skills/data-analyzer/download", API_BASE_URL), icon: "📊".into() },
        SkillInfo { id: "ai-chat".into(), name: "AI 对话增强".into(), description: "增强 AI 对话能力：角色扮演、情感分析".into(), author: "Bridge AI".into(), version: "2.0.0".into(), size_bytes: 340000, downloads: 3210, category: "工具".into(), tags: vec!["对话".into(),"角色扮演".into()], requires_premium: false, download_url: format!("{}/skills/ai-chat/download", API_BASE_URL), icon: "💬".into() },
        SkillInfo { id: "image-prompt".into(), name: "AI 绘画提示词".into(), description: "智能生成高质量的 AI 绘画提示词".into(), author: "Bridge AI".into(), version: "1.0.0".into(), size_bytes: 190000, downloads: 456, category: "创意".into(), tags: vec!["AI绘图".into(),"Prompt".into()], requires_premium: true, download_url: format!("{}/skills/image-prompt/download", API_BASE_URL), icon: "🎨".into() },
        SkillInfo { id: "resume-optimizer".into(), name: "简历优化师".into(), description: "简历优化、面试问题预测、职位匹配分析".into(), author: "Bridge AI".into(), version: "1.0.0".into(), size_bytes: 310000, downloads: 1890, category: "工具".into(), tags: vec!["简历".into(),"面试".into(),"求职".into()], requires_premium: true, download_url: format!("{}/skills/resume-optimizer/download", API_BASE_URL), icon: "📄".into() },
    ]
}
