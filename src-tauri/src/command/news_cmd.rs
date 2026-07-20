use crate::service::news_service;
use serde::Serialize;

#[derive(Serialize)]
pub struct NewsResponse {
    pub success: bool,
    pub items: Vec<news_service::NewsItem>,
    pub last_fetch: Option<String>,
    pub message: String,
}

/// 刷新资讯（从 RSS 源采集并保存到数据库）
#[tauri::command]
pub fn refresh_news() -> NewsResponse {
    log::info!("[Bridge AI] Refreshing AI news from RSS sources...");

    match news_service::fetch_news_from_rss() {
        Ok(items) => {
            let count = items.len();
            if let Err(e) = news_service::save_news_to_db(&items) {
                log::error!("[Bridge AI] Failed to save news to DB: {}", e);
                return NewsResponse {
                    success: false,
                    items: vec![],
                    last_fetch: None,
                    message: format!("采集成功但保存失败：{}", e),
                };
            }

            let last_fetch = news_service::get_last_fetch_time().ok().flatten();
            log::info!("[Bridge AI] Fetched {} news items", count);

            NewsResponse {
                success: true,
                items,
                last_fetch,
                message: format!("成功采集 {} 条最新资讯", count),
            }
        }
        Err(e) => {
            log::error!("[Bridge AI] Failed to fetch news: {}", e);
            NewsResponse {
                success: false,
                items: vec![],
                last_fetch: None,
                message: format!("采集失败：{}", e),
            }
        }
    }
}

/// 获取资讯列表（从数据库读取）
#[tauri::command]
pub fn get_news(category: Option<String>, limit: Option<i32>) -> NewsResponse {
    let last_fetch = news_service::get_last_fetch_time().ok().flatten();

    match news_service::get_news_from_db(category.as_deref(), limit) {
        Ok(items) => NewsResponse {
            success: true,
            items,
            last_fetch,
            message: String::new(),
        },
        Err(e) => NewsResponse {
            success: false,
            items: vec![],
            last_fetch,
            message: format!("获取资讯失败：{}", e),
        },
    }
}

/// 清除所有非中文资讯
#[tauri::command]
pub fn clear_non_chinese_news() -> NewsResponse {
    log::info!("[Bridge AI] Clearing non-Chinese news...");

    match news_service::clear_non_chinese_news() {
        Ok(deleted_count) => {
            log::info!("[Bridge AI] Deleted {} non-Chinese news items", deleted_count);
            NewsResponse {
                success: true,
                items: vec![],
                last_fetch: news_service::get_last_fetch_time().ok().flatten(),
                message: format!("已清除 {} 条非中文资讯", deleted_count),
            }
        }
        Err(e) => {
            log::error!("[Bridge AI] Failed to clear non-Chinese news: {}", e);
            NewsResponse {
                success: false,
                items: vec![],
                last_fetch: None,
                message: format!("清除失败：{}", e),
            }
        }
    }
}
