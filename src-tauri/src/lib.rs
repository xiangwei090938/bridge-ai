mod core;
mod service;
mod command;
mod util;

use tauri::Manager;

pub const PROXY_PORT: u16 = 53682;

// App state shared across commands (lightweight - most services are stateless)
pub struct AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            // Initialize Bridge AI config directory (~/.bridge-ai/)
            util::fs::BridgeConfig::init().expect("Failed to init Bridge AI config directory");

            // Write built-in tool definitions so detection works out of the box
            let tools_dir = util::fs::BridgeConfig::tools_dir();
            service::tool_detector::ToolDetector::ensure_builtin_definitions(&tools_dir);

            // Initialize database (for conversations, messages, installed skills, ai_news)
            let db_path = util::fs::BridgeConfig::db_path();
            util::db::init_database(&db_path).expect("Failed to init database");

            // 启动定时任务：每天 8:00 自动采集 AI 资讯（后台线程）
            std::thread::spawn(|| {
                if let Err(e) = service::news_service::start_daily_scheduler() {
                    log::error!("[Bridge AI] 定时任务启动失败：{}", e);
                }
            });

            log::info!("Bridge AI started, config dir: {:?}", util::fs::BridgeConfig::home_dir());
            log::info!("Bridge AI tools dir: {:?}", tools_dir);

            let proxy_path = dirs::home_dir().unwrap_or_default().join(".bridge-ai").join("codex_proxy.json");
            if !proxy_path.exists() {
                let models = service::provider_mgr::ProviderManager::load_models();
                if let Some(m) = models.first() {
                    let _ = std::fs::create_dir_all(proxy_path.parent().unwrap());
                    let cfg = serde_json::json!({"apiKey": m.api_key, "baseUrl": m.base_url, "modelId": m.model_id});
                    let _ = std::fs::write(&proxy_path, serde_json::to_string_pretty(&cfg).unwrap_or_default());
                    log::info!("Initialized proxy config from model: {}", m.name);
                }
            }

            let codex_cfg = dirs::home_dir().unwrap_or_default().join(".codex").join("config.toml");
            if codex_cfg.exists() {
                if let Ok(toml) = std::fs::read_to_string(&codex_cfg) {
                    let url = format!("http://127.0.0.1:{}/v1", PROXY_PORT);
                    let new_line = format!("base_url = \"{}\"", url);
                    let mut out = String::new();
                    let mut ch = false;
                    for l in toml.lines() {
                        let t = l.trim();
                        if t.starts_with("base_url") && t.contains("127.0.0.1:") {
                            out.push_str(&format!("{}{}\n", &l[..l.len()-t.len()], new_line));
                            ch = true;
                        } else {
                            out.push_str(l);
                            out.push('\n');
                        }
                    }
                    if ch {
                        let _ = std::fs::write(&codex_cfg, &out);
                        log::info!("[Bridge AI] Updated Codex config.toml to {}", url);
                    }
                }
            }
            tauri::async_runtime::spawn(async {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                    let u = format!("http://127.0.0.1:{}/health", PROXY_PORT);
                    if let Ok(cl) = reqwest::Client::builder().no_proxy().timeout(std::time::Duration::from_secs(5)).build() {
                        match cl.get(&u).send().await {
                            Ok(r) if r.status().is_success() => {}
                            _ => {
                                log::warn!("[Bridge AI] Proxy health check failed, restarting...");
                                match service::codex_proxy::start_proxy().await {
                                    Ok(a) => log::info!("[Bridge AI] Proxy recovered on http://{}", a),
                                    Err(e) => log::error!("[Bridge AI] Proxy restart failed: {}", e),
                                }
                            }
                        }
                    }
                }
            });

            tauri::async_runtime::spawn(async {
                match service::codex_proxy::start_proxy().await {
                    Ok(a) => log::info!("[Bridge AI] Codex proxy listening on http://{}", a),
                    Err(e) => log::error!("[Bridge AI] Failed to start proxy on port {}: {}", PROXY_PORT, e),
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::model_cmd::list_providers,
            command::model_cmd::list_models,
            command::model_cmd::add_model,
            command::model_cmd::delete_model,
            command::model_cmd::update_api_key,
            command::model_cmd::test_connection,
            command::chat_cmd::send_message,
            command::chat_cmd::list_conversations,
            command::chat_cmd::get_messages,
            command::config_cmd::get_setting,
            command::config_cmd::set_setting,
            command::tool_cmd::scan_tools,
            command::tool_cmd::sync_tool_config,
            command::tool_cmd::sync_all_tools,
            command::tool_cmd::launch_tool,
            command::tool_cmd::install_tool,
            command::premium_cmd::check_premium_status,
            command::premium_cmd::activate_premium,
            command::premium_cmd::start_premium_trial,
            command::skill_cmd::list_available_skills,
            command::skill_cmd::install_skill,
            command::skill_cmd::list_installed_skills,
            command::skill_cmd::uninstall_skill,
            command::search_cmd::web_search,
            command::search_cmd::web_search_context,
            command::model_deploy_cmd::check_ollama,
            command::model_deploy_cmd::list_models_catalog,
            command::model_deploy_cmd::pull_local_model,
            command::model_deploy_cmd::delete_local_model,
            command::draw_cmd::ai_draw,
            command::open_url_cmd::open_url,
            command::news_cmd::refresh_news,
            command::news_cmd::get_news,
            command::news_cmd::clear_non_chinese_news,
            command::chat_cmd::chat,
            command::premium_cmd::purchase_premium,
            command::skill_cmd::get_skill_detail,
            command::skill_cmd::enable_skill,
            command::skill_cmd::get_skill_templates,
            command::skill_cmd::list_all_skill_templates,
            command::skill_cmd::install_github_skill,
            command::config_cmd::get_proxy_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Bridge AI");
}
