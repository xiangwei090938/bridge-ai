
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub docs: Option<String>,
    #[serde(rename = "configFile")]
    pub config_file: Option<String>,
    #[serde(rename = "configFormat")]
    pub config_format: Option<String>,
    #[serde(rename = "configSection")]
    pub config_section: Option<String>,
    pub read: Option<Vec<String>>,
    pub write: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPaths {
    pub name: String,
    pub category: String,
    #[serde(rename = "apiProtocol")]
    pub api_protocol: Option<Vec<String>>,
    #[serde(rename = "installHints")]
    pub install_hints: Option<InstallHints>,
    pub win32: Vec<String>,
    pub darwin: Vec<String>,
    pub linux: Vec<String>,
    #[serde(rename = "launchUri")]
    pub launch_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallHints {
    #[serde(rename = "windowsDisplayNames")]
    pub windows_display_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let defs = Self::builtin_definitions();
        defs.into_iter().map(|def| {
            let (ipath, cpath) = Self::detect_tool(&def);
            Self::info_from_def(&def, ipath, cpath)
        }).collect()
    }

    pub fn scan_builtin() -> Vec<ToolInfo> {
        Self::scan_all()
    }

    pub fn load_definitions(_tools_dir: &Path) -> Vec<ToolDefinition> {
        // Return built-in definitions directly
        Self::builtin_definitions()
    }

    pub fn ensure_builtin_definitions(tools_dir: &Path) {
        use crate::util::fs::BridgeConfig;
        if tools_dir.exists() && std::fs::read_dir(tools_dir).map(|e| e.count() > 0).unwrap_or(false) {
            return;
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

    fn detect_tool(def: &ToolDefinition) -> (Option<String>, Option<String>) {
        if def.paths.win32.is_empty() && def.paths.darwin.is_empty() && def.paths.linux.is_empty() {
            return (Some("builtin".into()), None);
        }
        let exe = Self::exe_name(&def.paths);

        // Check defined paths first (fast, no process spawning)
        if let Some(p) = Self::find_in_paths(&def.paths) { return (Some(p), None); }
        // Scan common install directories
        if let Some(p) = Self::scan_common_locations(&def.paths) { return (Some(p), None); }
        // Check PATH
        if !exe.is_empty() {
            if let Some(p) = Self::find_on_path(&exe) { return (Some(p), None); }
        }
        // Check running processes (cross-platform)
        if let Some(p) = Self::find_by_process(&def.paths) { return (Some(p), None); }

        #[cfg(target_os = "windows")]
        {
            if let Some(p) = Self::find_windows_app(&def.paths) { return (Some(p), None); }
            if let Some(p) = Self::find_in_start_menu(&def.paths) { return (Some(p), None); }
            if let Some(p) = Self::scan_registry(&def.paths.name) { return (Some(p), None); }
            if let Some(ref hints) = def.paths.install_hints {
                if let Some(ref names) = hints.windows_display_names {
                    if let Some(p) = Self::scan_registry_by_display_names(names) { return (Some(p), None); }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(p) = Self::find_in_applications(&def.paths) { return (Some(p), None); }
            if let Some(p) = Self::find_in_homebrew(&def.paths) { return (Some(p), None); }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(p) = Self::find_in_linux_dirs(&def.paths) { return (Some(p), None); }
            if let Some(p) = Self::find_in_snap_flatpak(&def.paths) { return (Some(p), None); }
        }

        (None, None)
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

    fn exe_name(paths: &ToolPaths) -> String {
        let c = if cfg!(windows) { &paths.win32 }
            else if cfg!(target_os = "macos") { &paths.darwin }
            else { &paths.linux };
        c.first().and_then(|p| Path::new(p).file_stem()).and_then(|s| s.to_str()).unwrap_or("").to_string()
    }

    fn expand_env_vars(s: &str) -> String {
        let mut result = s.to_string();
        #[cfg(target_os = "windows")]
        {
            for (var, val) in [("LOCALAPPDATA", "%LOCALAPPDATA%"), ("APPDATA", "%APPDATA%"),
                ("ProgramFiles", "%ProgramFiles%"), ("ProgramFiles(x86)", "%ProgramFiles(x86)%"),
                ("USERPROFILE", "%USERPROFILE%")] {
                if let Ok(env_val) = std::env::var(var) {
                    result = result.replace(val, &env_val);
                }
            }
            if let Ok(v) = std::env::var("ProgramW6432") {
                result = result.replace("%ProgramW6432%", &v);
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            if let Ok(home) = std::env::var("HOME") {
                result = result.replace("$HOME", &home);
                result = result.replace("~", &home);
            }
            if let Ok(v) = std::env::var("XDG_CONFIG_HOME") {
                result = result.replace("$XDG_CONFIG_HOME", &v);
            }
        }
        result
    }

    fn find_in_paths(paths: &ToolPaths) -> Option<String> {
        let c = if cfg!(windows) { &paths.win32 }
            else if cfg!(target_os = "macos") { &paths.darwin }
            else { &paths.linux };
        for p in c {
            let e = Self::expand_env_vars(p);
            if Path::new(&e).exists() { return Some(e); }
        }
        None
    }

    fn find_on_path(exe: &str) -> Option<String> {
        if exe.is_empty() { return None; }
        let path_var = std::env::var("PATH").unwrap_or_default();
        for dir in std::env::split_paths(&path_var) {
            if !dir.exists() { continue; }
            let c = dir.join(exe);
            if c.exists() { return Some(c.to_string_lossy().to_string()); }
            #[cfg(target_os = "windows")]
            {
                let ce = dir.join(format!("{}.exe", exe));
                if ce.exists() { return Some(ce.to_string_lossy().to_string()); }
            }
        }
        None
    }

    fn find_by_process(paths: &ToolPaths) -> Option<String> {
        let name_lower = paths.name.to_lowercase();
        let mut system = System::new();
        system.refresh_processes(sysinfo::ProcessesToUpdate::All);
        for (_, process) in system.processes() {
            let exe_path = process.exe();
            if exe_path.is_none() { continue; }
            let exe = exe_path.unwrap().to_string_lossy().to_lowercase();
            let exe_stem = std::path::Path::new(&exe).file_stem()
                .and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
            if exe_stem.contains(&name_lower) {
                return Some(exe);
            }
        }
        None
    }

    #[cfg(target_os = "windows")]
    fn find_windows_app(paths: &ToolPaths) -> Option<String> {
        let name_lower = paths.name.to_lowercase();
        let win_apps_root = PathBuf::from(r"C:\Program Files\WindowsApps");
        if !win_apps_root.exists() { return None; }
        if let Ok(entries) = std::fs::read_dir(&win_apps_root) {
            for entry in entries.flatten() {
                let ename = entry.file_name().to_string_lossy().to_lowercase();
                // Scan the "app" subdirectory if it exists
                let app_dir = entry.path().join("app");
                if !app_dir.exists() { continue; }
                if let Ok(app_entries) = std::fs::read_dir(&app_dir) {
                    for app_entry in app_entries.flatten() {
                        let p = app_entry.path();
                        if p.extension().and_then(|e| e.to_str()) != Some("exe") { continue; }
                        // Match if folder name contains tool name, OR exe filename contains tool name
                        let exe_stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                        if ename.contains(&name_lower) || exe_stem.contains(&name_lower) {
                            return Some(p.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        None
    }

    #[cfg(not(target_os = "windows"))]
    fn find_windows_app(_paths: &ToolPaths) -> Option<String> { None }

    #[cfg(target_os = "windows")]
    fn find_in_start_menu(paths: &ToolPaths) -> Option<String> {
        let name_lower = paths.name.to_lowercase();
        let ap = std::env::var("APPDATA").ok()?;
        let dirs = vec![
            PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"),
            PathBuf::from(ap).join(r"Microsoft\Windows\Start Menu\Programs"),
        ];
        for d in &dirs {
            if !d.exists() { continue; }
            if let Ok(e) = std::fs::read_dir(d) {
                for en in e.flatten() {
                    if en.path().extension().and_then(|e| e.to_str()) != Some("lnk") { continue; }
                    let n = en.path().file_stem()?.to_string_lossy().to_lowercase();
                    if n.contains(&name_lower) || name_lower.contains(&n) {
                        return Some(en.path().to_string_lossy().to_string());
                    }
                }
            }
        }
        None
    }


    #[cfg(target_os = "macos")]
    fn find_in_applications(paths: &ToolPaths) -> Option<String> {
        let name_lower = paths.name.to_lowercase();
        let app_dirs = vec![
            PathBuf::from("/Applications"),
            PathBuf::from("/Applications/Utilities"),
            dirs::home_dir().map(|h| h.join("Applications")).unwrap_or_default(),
        ];
        for app_dir in &app_dirs {
            if !app_dir.exists() { continue; }
            if let Ok(entries) = std::fs::read_dir(app_dir) {
                for entry in entries.flatten() {
                    let ename = entry.file_name().to_string_lossy().to_lowercase();
                    if ename.contains(&name_lower) {
                        let app_path = entry.path();
                        if app_path.extension().and_then(|e| e.to_str()) == Some("app") {
                            let contents = app_path.join("Contents/MacOS");
                            if contents.exists() {
                                if let Ok(execs) = std::fs::read_dir(&contents) {
                                    for exec in execs.flatten() {
                                        if exec.path().is_file() {
                                            return Some(exec.path().to_string_lossy().to_string());
                                        }
                                    }
                                }
                            }
                            return Some(app_path.to_string_lossy().to_string());
                        }
                        if app_path.is_file() { return Some(app_path.to_string_lossy().to_string()); }
                    }
                }
            }
        }
        None
    }
    #[cfg(not(target_os = "macos"))]
    fn find_in_applications(_paths: &ToolPaths) -> Option<String> { None }

    #[cfg(target_os = "macos")]
    fn find_in_homebrew(paths: &ToolPaths) -> Option<String> {
        let name_lower = paths.name.to_lowercase();
        for brew_dir in &["/opt/homebrew/bin", "/usr/local/bin", "/opt/homebrew/Caskroom"] {
            let bd = PathBuf::from(brew_dir);
            if !bd.exists() { continue; }
            if let Ok(entries) = std::fs::read_dir(&bd) {
                for entry in entries.flatten() {
                    let ename = entry.file_name().to_string_lossy().to_lowercase();
                    if ename.contains(&name_lower) && entry.path().is_file() {
                        return Some(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
        None
    }
    #[cfg(not(target_os = "macos"))]
    fn find_in_homebrew(_paths: &ToolPaths) -> Option<String> { None }

    #[cfg(target_os = "linux")]
    fn find_in_linux_dirs(paths: &ToolPaths) -> Option<String> {
        let name_lower = paths.name.to_lowercase();
        let linux_dirs = vec![
            PathBuf::from("/usr/bin"),
            PathBuf::from("/usr/local/bin"),
            PathBuf::from("/opt"),
            dirs::home_dir().map(|h| h.join(".local/bin")).unwrap_or_default(),
        ];
        for linux_dir in &linux_dirs {
            if !linux_dir.exists() { continue; }
            if let Ok(entries) = std::fs::read_dir(linux_dir) {
                for entry in entries.flatten() {
                    let ename = entry.file_name().to_string_lossy().to_lowercase();
                    if ename.contains(&name_lower) && entry.path().is_file() {
                        return Some(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
        None
    }
    #[cfg(not(target_os = "linux"))]
    fn find_in_linux_dirs(_paths: &ToolPaths) -> Option<String> { None }

    #[cfg(target_os = "linux")]
    fn find_in_snap_flatpak(paths: &ToolPaths) -> Option<String> {
        let name_lower = paths.name.to_lowercase();
        for pkg_dir in &["/snap/bin", "/var/lib/snapd/snap", "/var/lib/flatpak/exports/bin"] {
            let pd = PathBuf::from(pkg_dir);
            if !pd.exists() { continue; }
            if let Ok(entries) = std::fs::read_dir(&pd) {
                for entry in entries.flatten() {
                    let ename = entry.file_name().to_string_lossy().to_lowercase();
                    if ename.contains(&name_lower) && entry.path().is_file() {
                        return Some(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
        None
    }
    #[cfg(not(target_os = "linux"))]
    fn find_in_snap_flatpak(_paths: &ToolPaths) -> Option<String> { None }

    #[cfg(not(target_os = "windows"))]
    fn scan_registry_by_display_names(_names: &[String]) -> Option<String> { None }

    fn scan_common_locations(paths: &ToolPaths) -> Option<String> {
        let name = paths.name.to_lowercase();
        let search_roots = vec![
            Self::expand_env_vars(r"%LOCALAPPDATA%\Programs"),
            Self::expand_env_vars(r"%ProgramFiles%"),
            Self::expand_env_vars(r"%ProgramFiles(x86)%"),
            Self::expand_env_vars(r"%USERPROFILE%\AppData\Local"),
            Self::expand_env_vars(r"%USERPROFILE%\.local\bin"),
            Self::expand_env_vars(r"%USERPROFILE%\AppData\Roaming"),
            Self::expand_env_vars(r"%USERPROFILE%\scoop\apps"),
            Self::expand_env_vars(r"%USERPROFILE%\AppData\Local\Microsoft\WinGet\Packages"),
            r"C:\Program Files\WindowsApps".to_string(),
            r"C:\ProgramData".to_string(),
        ];
        for root in &search_roots {
            if !Path::new(root).exists() { continue; }
            if let Ok(entries) = std::fs::read_dir(root) {
                for entry in entries.flatten() {
                    let ename = entry.file_name().to_string_lossy().to_lowercase();
                    if ename.contains(&name) || name.contains(&ename) {
                        let path = entry.path();
                        if path.is_file() {
                            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                            if ext == "exe" || ext == "lnk" { return Some(path.to_string_lossy().to_string()); }
                        }
                        if path.is_dir() {
                            if let Ok(sub) = std::fs::read_dir(&path) {
                                for sub_entry in sub.flatten() {
                                    let sub_path = sub_entry.path();
                                    if !sub_path.is_file() { continue; }
                                    let ext = sub_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                                    if ext == "exe" || ext == "lnk" {
                                        let stem = sub_path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                                        if stem.contains(&name) { return Some(sub_path.to_string_lossy().to_string()); }
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

    fn builtin_definitions() -> Vec<ToolDefinition> {
        vec![
            Self::builtin_def("aider", "Aider", "CLI Code", &["openai", "anthropic"],
                &[r"%USERPROFILE%\.local\bin\aider.exe",
                  r"%APPDATA%\Python\Scripts\aider.exe",
                  r"%LOCALAPPDATA%\Programs\Python\Python312\Scripts\aider.exe",
                  r"%LOCALAPPDATA%\Programs\Python\Python311\Scripts\aider.exe",
                  r"%LOCALAPPDATA%\Programs\Python\Python310\Scripts\aider.exe"],
                None, None),
            Self::builtin_def("chatgptdesktop", "ChatGPT", "Desktop", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\ChatGPT\ChatGPT.exe",
                  r"%LOCALAPPDATA%\Programs\Codex\Codex.exe"],
                None, None),
            Self::builtin_def("claudecode", "Claude Code", "CLI Code", &["anthropic"],
                &[r"%USERPROFILE%\.local\bin\claude.exe",
                  r"%APPDATA%\npm\claude.cmd",
                  r"%USERPROFILE%\.bun\bin\claude.exe",
                  r"%LOCALAPPDATA%\Microsoft\WinGet\Links\claude.exe",
                  r"%LOCALAPPDATA%\pnpm\claude.exe"],
                None, None),
            Self::builtin_def("claudedesktop", "Claude Desktop", "Desktop", &["anthropic"],
                &[r"%LOCALAPPDATA%\AnthropicClaude\Claude.exe",
                  r"%LOCALAPPDATA%\Programs\Claude\Claude.exe"],
                None, None),
            Self::builtin_def("claudescience", "Claude Science", "Science", &["anthropic"],
                &[],
                None, None),
            Self::builtin_def("codex", "Codex CLI", "CLI Code", &["openai"],
                &[r"%APPDATA%\npm\codex.cmd",
                  r"%USERPROFILE%\scoop\shims\codex.exe",
                  r"%USERPROFILE%\.bun\bin\codex.exe",
                  r"%USERPROFILE%\.local\bin\codex.exe",
                  r"%LOCALAPPDATA%\pnpm\codex.exe"],
                None, None),
            Self::builtin_def("coffeecli", "Coffee CLI", "Desktop", &["openai"],
                &[r"%LOCALAPPDATA%\Coffee CLI\coffee-cli.exe"],
                None, None),
            Self::builtin_def("cursor", "Cursor", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\cursor\Cursor.exe"],
                None, None),
            Self::builtin_def("geminidesktop", "Gemini Desktop", "Desktop", &["openai"],
                &[],
                None, None),
            Self::builtin_def("grok", "Grok Build", "CLI Code", &["openai"],
                &[r"%USERPROFILE%\.grok\bin\grok.exe",
                  r"%USERPROFILE%\.local\bin\grok.exe",
                  r"%LOCALAPPDATA%\Programs\grok\grok.exe",
                  r"%USERPROFILE%\.bun\bin\grok.exe",
                  r"%LOCALAPPDATA%\pnpm\grok.exe"],
                None, None),
            Self::builtin_def("hermes", "Hermes Desktop", "Desktop", &["openai"],
                &[r"%LOCALAPPDATA%\hermes\hermes-agent\apps\desktop\release\win-unpacked\Hermes.exe"],
                None, None),
            Self::builtin_def("kimicode", "Kimi Code", "CLI Code", &["openai"],
                &[r"%APPDATA%\npm\kimi.cmd",
                  r"%USERPROFILE%\.kimi-code\bin\kimi.exe",
                  r"%USERPROFILE%\.bun\bin\kimi.exe",
                  r"%LOCALAPPDATA%\pnpm\kimi.exe",
                  r"%USERPROFILE%\.npm-global\kimi.cmd"],
                None, None),
            Self::builtin_def("mimocode", "MiMo Code", "CLI Code", &["openai"],
                &[r"%APPDATA%\npm\mimo.cmd",
                  r"%USERPROFILE%\.bun\bin\mimo.exe",
                  r"%LOCALAPPDATA%\pnpm\mimo.exe",
                  r"%USERPROFILE%\.mimocode\bin\mimo.exe"],
                None, None),
            Self::builtin_def("openclaw", "OpenClaw", "CLI Code", &["openai", "anthropic"],
                &[r"%APPDATA%\npm\openclaw.cmd",
                  r"%LOCALAPPDATA%\OpenClaw\bin\openclaw.exe",
                  r"%USERPROFILE%\scoop\shims\openclaw.exe",
                  r"%LOCALAPPDATA%\pnpm\openclaw.exe",
                  r"%USERPROFILE%\.local\bin\openclaw.exe"],
                None, None),
            Self::builtin_def("opencode", "OpenCode", "CLI Code", &["openai"],
                &[r"%APPDATA%\npm\opencode.cmd",
                  r"%LOCALAPPDATA%\Programs\opencode\opencode.exe",
                  r"%USERPROFILE%\scoop\shims\opencode.exe",
                  r"%PROGRAMFILES%\opencode\opencode.exe",
                  r"%USERPROFILE%\.bun\bin\opencode.exe",
                  r"%USERPROFILE%\.opencode\bin\opencode.exe",
                  r"%LOCALAPPDATA%\pnpm\opencode.exe"],
                None, None),
            Self::builtin_def("opencodedesktop", "OpenCode Desktop", "Desktop", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\OpenCode\OpenCode.exe",
                  r"%LOCALAPPDATA%\Programs\opencode\OpenCode.exe",
                  r"%LOCALAPPDATA%\Programs\OpenCode Beta\OpenCode Beta.exe"],
                None, None),
            Self::builtin_def("openscience", "OpenScience", "Science", &["anthropic", "openai"],
                &[r"%APPDATA%\npm\openscience.cmd",
                  r"~\.openscience\bin\openscience.exe"],
                None, None),
            Self::builtin_def("pi", "Pi", "CLI Code", &["openai"],
                &[r"%APPDATA%\npm\pi.cmd",
                  r"%USERPROFILE%\.local\bin\pi.exe",
                  r"%USERPROFILE%\.bun\bin\pi.exe",
                  r"%USERPROFILE%\.hermes\node\bin\pi.exe",
                  r"%USERPROFILE%\.hermes\node\bin\pi.cmd",
                  r"%LOCALAPPDATA%\pnpm\pi.exe"],
                None, None),
            Self::builtin_def("qwencode", "QwenCode", "CLI Code", &["openai", "anthropic"],
                &[r"%APPDATA%\npm\qwen.cmd",
                  r"%USERPROFILE%\.local\bin\qwen.exe",
                  r"%USERPROFILE%\.bun\bin\qwen.exe",
                  r"%USERPROFILE%\scoop\shims\qwen.exe",
                  r"%LOCALAPPDATA%\pnpm\qwen.exe"],
                None, None),
            Self::builtin_def("reversi", "Reversi", "Game", &["openai", "anthropic"],
                &[],
                None, None),
            Self::builtin_def("trae", "Trae", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\Trae\Trae.exe"],
                None, None),
            Self::builtin_def("traecn", "Trae CN", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\Trae CN\Trae CN.exe",
                  r"%LOCALAPPDATA%\Programs\Trae-CN\Trae CN.exe"],
                None, None),
            Self::builtin_def("translator", "AI Translate", "Utility", &["openai", "anthropic"],
                &[],
                None, None),
            Self::builtin_def("vibe-trading", "Vibe-Trading", "AutoTrading", &["openai"],
                &[r"%USERPROFILE%\.local\bin\vibe-trading.exe",
                  r"%APPDATA%\Python\Scripts\vibe-trading.exe",
                  r"%LOCALAPPDATA%\Programs\Python\Python313\Scripts\vibe-trading.exe",
                  r"%LOCALAPPDATA%\Programs\Python\Python312\Scripts\vibe-trading.exe",
                  r"%LOCALAPPDATA%\Programs\Python\Python311\Scripts\vibe-trading.exe"],
                None, None),
            Self::builtin_def("vscode", "Visual Studio Code", "IDE", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\Microsoft VS Code\Code.exe",
                  r"%ProgramFiles%\Microsoft VS Code\Code.exe"],
                None, None),
            Self::builtin_def("workbuddy", "WorkBuddy", "Desktop", &["openai"],
                &[r"%LOCALAPPDATA%\Programs\WorkBuddy\WorkBuddy.exe"],
                None, None),
            Self::builtin_def("zcode", "ZCode", "Desktop", &["openai", "anthropic"],
                &[r"%LOCALAPPDATA%\Programs\ZCode\ZCode.exe"],
                None, None),
        ]
    }

    fn builtin_def(id: &str, name: &str, category: &str, protocols: &[&str],
                   win_paths: &[&str], launch_uri: Option<&str>,
                   config: Option<ToolConfig>) -> ToolDefinition {
        ToolDefinition {
            id: id.to_string(),
            paths: ToolPaths {
                name: name.to_string(),
                category: category.to_string(),
                api_protocol: Some(protocols.iter().map(|s| s.to_string()).collect()),
                install_hints: None,
                win32: win_paths.iter().map(|s| s.to_string()).collect(),
                darwin: vec![],
                linux: vec![],
                launch_uri: launch_uri.map(|s| s.to_string()),
            },
            config,
        }
    }

    /// Scan Windows Registry for installed programs
    #[cfg(not(target_os = "windows"))]
    fn find_in_start_menu(_paths: &ToolPaths) -> Option<String> { None }

    #[cfg(target_os = "windows")]
    fn scan_registry(name: &str) -> Option<String> {
        if !cfg!(windows) { return None; }
        let name_lower = name.to_lowercase();
        
        // Registry paths to scan for installed programs
        let reg_paths = vec![
            r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
            r"HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
            r"HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        ];
        
        for reg_path in &reg_paths {
            let output = Command::new("reg")
                .args(["query", reg_path, "/s", "/f", name, "/t", "REG_SZ"])
                .output();
            
            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                // Look for InstallLocation or DisplayIcon in the output
                for line in stdout.lines() {
                    let line_lower = line.to_lowercase();
                    if line_lower.contains("installlocation") || line_lower.contains("displayicon") {
                        if let Some(val_start) = line.find("REG_SZ") {
                            let val = line[val_start + 6..].trim();
                            // Clean up the path
                            let val = val.trim_matches('"');
                            let val = val.trim();
                            if !val.is_empty() {
                                // If it's an icon path like "C:\path\app.exe,0", extract just the exe
                                let val = val.split(',').next().unwrap_or(val);
                                let p = Path::new(val);
                                if p.exists() {
                                    return Some(val.to_string());
                                }
                                // Try appending .exe
                                let exe_path = format!("{}.exe", val);
                                if Path::new(&exe_path).exists() {
                                    return Some(exe_path);
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Scan registry by display name hints (for tools with multiple possible names)
    fn scan_registry_by_display_names(display_names: &[String]) -> Option<String> {
        for name in display_names {
            if let Some(path) = Self::scan_registry(name) {
                return Some(path);
            }
        }
        None
    }
}


