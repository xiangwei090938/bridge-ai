use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub docs: Option<String>,
    #[serde(rename = "configFile")]
    pub config_file: Option<String>,
    pub format: Option<String>,
    pub custom: Option<bool>,
    pub read: Option<ToolReadMapping>,
    pub write: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolReadMapping {
    pub model: Option<Vec<String>>,
    #[serde(rename = "baseUrl")]
    pub base_url: Option<Vec<String>>,
    #[serde(rename = "apiKey")]
    pub api_key: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPaths {
    pub name: String,
    pub category: String,
    #[serde(rename = "apiProtocol")]
    pub api_protocol: Option<Vec<String>>,
    pub command: Option<String>,
    #[serde(rename = "launchUri")]
    pub launch_uri: Option<String>,
    pub paths: ToolPlatformPaths,
    #[serde(rename = "installHints")]
    pub install_hints: Option<InstallHints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPlatformPaths {
    #[serde(default)]
    pub win32: Vec<String>,
    #[serde(default)]
    pub darwin: Vec<String>,
    #[serde(default)]
    pub linux: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallHints {
    #[serde(rename = "windowsDisplayNames")]
    pub windows_display_names: Option<Vec<String>>,
    #[serde(rename = "windowsPublisher")]
    pub windows_publisher: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ToolDefinition {
    pub id: String,
    pub paths: ToolPaths,
    pub config: Option<ToolConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub id: String,
    pub name: String,
    pub category: String,
    pub is_installed: bool,
    pub install_path: Option<String>,
    pub config_path: Option<String>,
    pub api_protocols: Vec<String>,
    pub has_config: bool,
    pub sync_supported: bool,
    pub launch_uri: Option<String>,
}

pub struct ToolDetector;

impl ToolDetector {
    pub fn scan_all() -> Vec<ToolInfo> {
        let tools_dir = crate::util::fs::BridgeConfig::tools_dir();
        let defs = Self::load_definitions(&tools_dir);
        
        // If definitions exist on disk, use them
        if !defs.is_empty() {
            return defs.into_iter().map(|def| {
                let (ipath, cpath) = Self::detect_tool(&def);
                Self::info_from_def(&def, ipath, cpath)
            }).collect();
        }
        
        // Fallback: use built-in definitions so detection always works
        Self::scan_builtin()
    }

    pub fn scan_builtin() -> Vec<ToolInfo> {
        let defs = Self::builtin_definitions();
        defs.into_iter().map(|def| {
            let (ipath, cpath) = Self::detect_tool(&def);
            Self::info_from_def(&def, ipath, cpath)
        }).collect()
    }

    fn info_from_def(def: &ToolDefinition, ipath: Option<String>, cpath: Option<String>) -> ToolInfo {
        ToolInfo {
            id: def.id.clone(),
            name: def.paths.name.clone(),
            category: def.paths.category.clone(),
            is_installed: ipath.is_some(),
            install_path: ipath,
            config_path: cpath,
            api_protocols: def.paths.api_protocol.clone().unwrap_or_default(),
            has_config: def.config.is_some(),
            sync_supported: def.config.as_ref().and_then(|c| c.write.as_ref()).is_some(),
            launch_uri: def.paths.launch_uri.clone(),
        }
    }

    pub fn load_definitions(tools_dir: &Path) -> Vec<ToolDefinition> {
        let mut defs = Vec::new();
        if !tools_dir.exists() { return defs; }
        if let Ok(entries) = std::fs::read_dir(tools_dir) {
            for entry in entries.flatten() {
                if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) { continue; }
                let id = entry.file_name().to_string_lossy().to_string();
                let pp = entry.path().join("paths.json");
                let cp = entry.path().join("config.json");
                if let Some(p) = crate::util::fs::BridgeConfig::read_json::<ToolPaths>(&pp) {
                    let c: Option<ToolConfig> = crate::util::fs::BridgeConfig::read_json(&cp);
                    defs.push(ToolDefinition { id, paths: p, config: c });
                }
            }
        }
        defs
    }

    /// Create built-in tool definition files on disk so the user can customize them later
    pub fn ensure_builtin_definitions(tools_dir: &Path) {
        use crate::util::fs::BridgeConfig;
        if tools_dir.exists() && std::fs::read_dir(tools_dir).map(|e| e.count() > 0).unwrap_or(false) {
            return; // Already has definitions
        }
        let builtin = Self::builtin_definitions();
        for def in &builtin {
            let dir = tools_dir.join(&def.id);
            let pp = dir.join("paths.json");
            if !pp.exists() {
                let _ = BridgeConfig::write_json(&pp, &def.paths);
            }
            if let Some(ref config) = def.config {
                let cp = dir.join("config.json");
                if !cp.exists() {
                    let _ = BridgeConfig::write_json(&cp, config);
                }
            }
        }
    }

    // ====== Built-in definitions for common AI tools ======
    fn builtin_definitions() -> Vec<ToolDefinition> {
        vec![
            Self::builtin_def("chatgptdesktop", "ChatGPT", "桌面端", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\ChatGPT\ChatGPT.exe",
                  r"%APPDATA%\Microsoft\Windows\Start Menu\Programs\ChatGPT.lnk"],
                Some("shell:AppsFolder\\OpenAI.ChatGPT_2p2nqsd0c76g0!App"), None),
            Self::builtin_def("claudedesktop", "Claude Desktop", "桌面端", &["anthropic"],
                &[r"%LOCALAPPDATA%\Programs\Claude\Claude.exe",
                  r"%APPDATA%\Microsoft\Windows\Start Menu\Programs\Claude Desktop.lnk"],
                Some("shell:AppsFolder\\Anthropic.Claude_9nn07rvydr1pw!App"), None),
            Self::builtin_def("cursor", "Cursor", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\cursor\Cursor.exe",
                  r"%USERPROFILE%\AppData\Local\Programs\cursor\Cursor.exe"], None, None),
            Self::builtin_def("opencode", "OpenCode", "桌面端", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\opencode\OpenCode.exe",
                  r"%USERPROFILE%\AppData\Local\opencode\OpenCode.exe"], None, None),
            Self::builtin_def("claudecode", "Claude Code", "CLI 命令行", &["anthropic"],
                &[r"%USERPROFILE%\.local\bin\claude.exe"], None, None),
            Self::builtin_def("hermes", "Hermes", "桌面端", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\hermes\Hermes.exe",
                  r"%USERPROFILE%\AppData\Local\hermes\Hermes.exe"], None, None),
            Self::builtin_def("vscode", "VS Code", "IDE", &[],
                &[r"%LOCALAPPDATA%\Programs\Microsoft VS Code\Code.exe",
                  r"%ProgramFiles%\Microsoft VS Code\Code.exe"], None, None),
            Self::builtin_def("geminidesktop", "Gemini Desktop", "桌面端", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\Gemini\Gemini.exe"], None, None),
            Self::builtin_def("aider", "Aider", "CLI 命令行", &["openai"],
                &[r"%USERPROFILE%\.local\bin\aider.exe"], None, None),
            Self::builtin_def("codex", "Codex CLI", "CLI 命令行", &["openai"],
                &[r"%USERPROFILE%\.codex\bin\codex.exe",
                  r"%LOCALAPPDATA%\Programs\codex\Codex.exe"], None, None),
            Self::builtin_def("claudescience", "Claude Science", "科学", &["anthropic"],
                &[r"%LOCALAPPDATA%\Programs\ClaudeScience\ClaudeScience.exe"], None, None),
            Self::builtin_def("opencodedesktop", "OpenCode Desktop", "桌面端", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\OpenCodeDesktop\OpenCodeDesktop.exe"], None, None),
            Self::builtin_def("openscience", "OpenScience", "科学", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\OpenScience\OpenScience.exe"], None, None),
            Self::builtin_def("kimicode", "Kimi Code", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\KimiCode\KimiCode.exe"], None, None),
            Self::builtin_def("qwencode", "Qwen Code", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\QwenCode\QwenCode.exe"], None, None),
            Self::builtin_def("trae", "Trae", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\Trae\Trae.exe"], None, None),
            Self::builtin_def("traecn", "Trae CN", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\TraeCN\TraeCN.exe"], None, None),
            Self::builtin_def("workbuddy", "WorkBuddy", "桌面端", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\WorkBuddy\WorkBuddy.exe"], None, None),
            Self::builtin_def("zcode", "Z Code", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\ZCode\ZCode.exe"], None, None),
            Self::builtin_def("mimocode", "Mimo Code", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\MimoCode\MimoCode.exe"], None, None),
            Self::builtin_def("grok", "Grok", "桌面端", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\Grok\Grok.exe"], None, None),
            Self::builtin_def("pi", "Pi Assistant", "桌面端", &[],
                &[r"%LOCALAPPDATA%\Programs\Pi\Pi.exe"], None, None),
            Self::builtin_def("coffeecli", "Coffee CLI", "CLI 命令行", &["openai"],
                &[r"%USERPROFILE%\.local\bin\coffee.exe"], None, None),
            Self::builtin_def("openclaw", "OpenClaw", "CLI 命令行", &["openai"],
                &[r"%USERPROFILE%\.local\bin\openclaw.exe"], None, None),
            // Built-in tools (always installed)
            Self::builtin_def("reversi", "Reversi", "内置工具", &[], &[], None, None),
            Self::builtin_def("translator", "Translator", "内置工具", &[], &[], None, None),
            Self::builtin_def("vibe-trading", "Vibe Trading", "内置工具", &[], &[], None, None),
        ]
    }

    fn builtin_def(id: &str, name: &str, category: &str, protocols: &[&str],
                   win_paths: &[&str], launch_uri: Option<&str>,
                   config: Option<ToolConfig>) -> ToolDefinition {
        let api_protocol = if protocols.is_empty() { None } else {
            Some(protocols.iter().map(|s| s.to_string()).collect())
        };
        ToolDefinition {
            id: id.to_string(),
            paths: ToolPaths {
                name: name.to_string(),
                category: category.to_string(),
                api_protocol,
                command: None,
                launch_uri: launch_uri.map(|s| s.to_string()),
                paths: ToolPlatformPaths {
                    win32: win_paths.iter().map(|s| s.to_string()).collect(),
                    darwin: vec![],
                    linux: vec![],
                },
                install_hints: None,
            },
            config,
        }
    }

    fn expand_env_vars(s: &str) -> String {
        let mut r = s.to_string();
        for &(var, key) in &[("%LOCALAPPDATA%", "LOCALAPPDATA"),
            ("%APPDATA%", "APPDATA"), ("%USERPROFILE%", "USERPROFILE"),
            ("%ProgramFiles%", "ProgramFiles")]
        { if let Ok(v) = std::env::var(key) { r = r.replace(var, &v); } }
        if r.starts_with("~/") || r.starts_with("~\\") {
            if let Ok(h) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
                r = h + &r[1..]; } }
        r
    }

    fn detect_tool(def: &ToolDefinition) -> (Option<String>, Option<String>) {
        // If it's a built-in tool with no paths, just return as installed
        if def.paths.paths.win32.is_empty() && def.paths.paths.darwin.is_empty() && def.paths.paths.linux.is_empty() {
            return (Some("builtin".into()), None);
        }
        let exe = Self::exe_name(&def.paths);
        // Priority 1: Running process (most reliable indicator)
        if !exe.is_empty() {
            if let Some(p) = Self::find_by_process(&exe) { return (Some(p), None); }
        }
        // Priority 2: Windows Store Apps (WindowsApps folder + Shell:AppsFolder)
        if let Some(p) = Self::find_windows_app(&def.paths) { return (Some(p), None); }
        // Priority 3: Start Menu shortcuts
        if let Some(p) = Self::find_in_start_menu(&def.paths) { return (Some(p), None); }
        // Priority 4: Defined paths on disk
        if let Some(p) = Self::find_in_paths(&def.paths) { return (Some(p), None); }
        // Priority 5: System PATH
        if let Some(p) = Self::find_on_path(&exe) { return (Some(p), None); }
        // Priority 6: Scan common install directories
        if let Some(p) = Self::scan_common_locations(&def.paths) { return (Some(p), None); }
        (None, None)
    }

    fn exe_name(paths: &ToolPaths) -> String {
        let c = if cfg!(windows) { &paths.paths.win32 }
            else if cfg!(target_os = "macos") { &paths.paths.darwin }
            else { &paths.paths.linux };
        c.first().and_then(|p| Path::new(p).file_name().map(|n| n.to_string_lossy().to_string())).unwrap_or_default()
    }

    fn find_in_start_menu(paths: &ToolPaths) -> Option<String> {
        let ap = std::env::var("APPDATA").ok()?;
        let dirs = vec![
            PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"),
            PathBuf::from(ap).join(r"Microsoft\Windows\Start Menu\Programs"),
        ];
        let name_lower = paths.name.to_lowercase();
        let id_lower = paths.id.to_lowercase();
        // Collect all known display names to match against
        let known_names: Vec<String> = {
            let mut kn = vec![name_lower.clone(), id_lower.clone()];
            if let Some(ref hints) = paths.install_hints {
                if let Some(ref names) = hints.windows_display_names {
                    kn.extend(names.iter().map(|n| n.to_lowercase()));
                }
            }
            kn
        };
        for d in &dirs {
            if !d.exists() { continue; }
            if let Ok(e) = std::fs::read_dir(d) {
                for en in e.flatten() {
                    let ext = en.path().extension().and_then(|e| e.to_str())?.to_lowercase();
                    if ext != "lnk" { continue; }
                    let n = en.path().file_stem()?.to_string_lossy().to_lowercase();
                    if known_names.iter().any(|x| n.contains(x) || x.contains(&n)) {
                        return Some(en.path().to_string_lossy().to_string());
                    }
                }
            }
        }
        None
    }

    fn find_by_process(exe: &str) -> Option<String> {
        if exe.is_empty() { return None; }
        let pn = exe.trim_end_matches(".exe");
        let s = format!("Get-Process -Name '{}' -ErrorAction SilentlyContinue | Select -ExpandProperty Path -First 1", pn);
        let o = Command::new("powershell").args(["-Command", &s]).output().ok()?;
        if o.status.success() { let t = String::from_utf8_lossy(&o.stdout).trim().to_string(); if !t.is_empty() { return Some(t); } } None
    }

    fn find_in_paths(paths: &ToolPaths) -> Option<String> {
        let c = if cfg!(windows) { &paths.paths.win32 }
            else if cfg!(target_os = "macos") { &paths.paths.darwin }
            else { &paths.paths.linux };
        for p in c { let e = Self::expand_env_vars(p); if Path::new(&e).exists() { return Some(e); } } None
    }

    fn find_on_path(exe: &str) -> Option<String> {
        if exe.is_empty() { return None; }
        let o = Command::new("where.exe").arg(exe).output().ok()?;
        if o.status.success() { let t = String::from_utf8_lossy(&o.stdout).lines().next()?.trim().to_string(); if !t.is_empty() { return Some(t); } } None
    }
    /// Find installed Windows App / Store app
    fn find_windows_app(paths: &ToolPaths) -> Option<String> {
        let name_lower = paths.name.to_lowercase();
        let id_lower = paths.id.to_lowercase();
        // Check common WindowsApps paths directly
        let win_apps_root = PathBuf::from(r"C:\Program Files\WindowsApps");
        if win_apps_root.exists() {
            if let Ok(entries) = std::fs::read_dir(&win_apps_root) {
                for entry in entries.flatten() {
                    let ename = entry.file_name().to_string_lossy().to_lowercase();
                    if ename.contains(&name_lower) || ename.contains(&id_lower) {
                        let app_dir = entry.path().join("app");
                        if let Ok(app_entries) = std::fs::read_dir(&app_dir) {
                            for app_entry in app_entries.flatten() {
                                let p = app_entry.path();
                                if p.extension().and_then(|e| e.to_str()) == Some("exe") {
                                    return Some(p.to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        // Try Shell:AppsFolder via PowerShell
        let s = format!(
            "Get-StartApps | Where-Object {{ _.Name -like '*{0}*' -or _.AppId -like '*{0}*' }} | Select-Object -ExpandProperty AppId -First 1",
            name_lower
        );
        let o = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &s])
            .output().ok()?;
        if o.status.success() {
            let t = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if !t.is_empty() {
                return Some(format!("shell:AppsFolder\\{}", t));
            }
        }
        None
    }

    /// Scan common install directories beyond defined paths
    fn scan_common_locations(paths: &ToolPaths) -> Option<String> {
        let name = paths.name.to_lowercase();
        let id = paths.id.to_lowercase();
        let search_roots = vec![
            Self::expand_env_vars(r"%LOCALAPPDATA%\Programs"),
            Self::expand_env_vars(r"%ProgramFiles%"),
            Self::expand_env_vars(r"%ProgramFiles(x86)%"),
            Self::expand_env_vars(r"%USERPROFILE%\AppData\Local"),
            Self::expand_env_vars(r"%USERPROFILE%\.local\bin"),
            r"C:\Program Files\WindowsApps".to_string(),
        ];
        for root in &search_roots {
            if !Path::new(&root).exists() { continue; }
            if let Ok(entries) = std::fs::read_dir(&root) {
                for entry in entries.flatten() {
                    let ename = entry.file_name().to_string_lossy().to_lowercase();
                    if ename.contains(&name) || ename.contains(&id) || name.contains(&ename) || id.contains(&ename) {
                        let path = entry.path();
                        // If it's a file directly, check extension
                        if path.is_file() {
                            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                            if ext == "exe" || ext == "lnk" {
                                return Some(path.to_string_lossy().to_string());
                            }
                        }
                        // If it's a directory, look for exe inside (first level)
                        if path.is_dir() {
                            if let Ok(sub) = std::fs::read_dir(&path) {
                                for sub_entry in sub.flatten() {
                                    let sub_path = sub_entry.path();
                                    if !sub_path.is_file() { continue; }
                                    let ext = sub_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                                    if ext == "exe" || ext == "lnk" {
                                        let stem = sub_path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                                        if stem.contains(&name) || stem.contains(&id) {
                                            return Some(sub_path.to_string_lossy().to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
}





