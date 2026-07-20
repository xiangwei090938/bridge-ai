mod core;
mod service;
mod command;
mod util;

use tauri::Manager;

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

            app.manage(AppState);
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running Bridge AI");
}
