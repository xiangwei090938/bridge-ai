use crate::core::error::Result;
use crate::util::db::get_connection;
use feed_rs::parser;
use tokio_cron_scheduler::{Job, JobScheduler};

/// AI 资讯 RSS 源列表（全中文源 - 已验证可用）
const RSS_SOURCES: &[(&str, &str)] = &[
    // 机器之心 - 国内权威 AI 媒体
    ("机器之心", "https://www.jiqizhixin.com/rss"),
    // 量子位 - AI 行业资讯
    ("量子位", "https://www.qbitai.com/feed"),
    // 36 氪 - 科技商业
    ("36 氪", "https://36kr.com/feed"),
    // InfoQ 中文站
    ("InfoQ 中文", "https://www.infoq.cn/public/v1/my/recommend"),
    // 开源中国
    ("开源中国", "https://www.oschina.net/news/rss"),
    // 掘金 - 开发者社区
    ("掘金", "https://juejin.cn/backend/api/v1/tag/6809637773935370254/feed"),
];

/// AI 资讯分类关键词映射（中文）
const CATEGORY_KEYWORDS: &[(&str, &[&str])] = &[
    ("模型发布", &["GPT", "Claude", "Gemini", "Llama", "Qwen", "DeepSeek", "模型", "发布", "上线", "推出", "新版", "开源", "训练", "参数"]),
    ("工具更新", &["Cursor", "Copilot", "Codex", "IDE", "工具", "更新", "版本", "升级", "发布", "客户端", "插件", "扩展"]),
    ("开源动态", &["开源", "GitHub", "HuggingFace", "开放", "免费", "社区", "仓库", "贡献", "PR"]),
    ("行业分析", &["分析", "报告", "趋势", "市场", "调研", "洞察", "解读", "评论", "展望", "融资", "投资"]),
    ("技术应用", &["应用", "落地", "场景", "案例", "实践", "教程", "指南", "技巧", "方法"]),
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct NewsItem {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub source: String,
    pub category: String,
    pub tags: String,
    pub url: String,
    pub published_at: String,
    pub fetched_at: String,
}

/// 检测文本是否主要为中文（至少包含 5 个中文字符）
fn is_chinese_content(text: &str) -> bool {
    let chinese_count = text.chars().filter(|c| ('\u{4e00}'..='\u{9fff}').contains(c)).count();
    chinese_count >= 5
}

/// 自动分类并生成标签
fn categorize_and_tag(title: &str, summary: &str) -> (String, Vec<String>) {
    let text = format!("{} {}", title, summary);
    let mut tags = Vec::new();
    let mut category = "行业动态".to_string();

    for (cat, keywords) in CATEGORY_KEYWORDS {
        let mut matched = false;
        for keyword in keywords.iter() {
            if text.contains(keyword) {
                tags.push(keyword.to_string());
                matched = true;
                if category == "行业动态" {
                    category = cat.to_string();
                }
            }
        }
        if matched && tags.len() <= 5 {
            break;
        }
    }

    // 限制标签数量
    tags.truncate(5);
    if tags.is_empty() {
        tags.push("AI".to_string());
    }

    (category, tags)
}

/// 从 RSS 源采集最新资讯（仅保留中文文章，去重）
pub fn fetch_news_from_rss() -> Result<Vec<NewsItem>> {
    let mut all_items = Vec::new();

    for (source_name, feed_url) in RSS_SOURCES {
        match fetch_single_feed(source_name, feed_url) {
            Ok(items) => {
                // 只保留中文文章（标题或摘要包含至少 5 个中文字符）
                let chinese_items: Vec<NewsItem> = items
                    .into_iter()
                    .filter(|item| {
                        let title_chinese = is_chinese_content(&item.title);
                        let summary_chinese = is_chinese_content(&item.summary);
                        title_chinese || summary_chinese
                    })
                    .collect();
                all_items.extend(chinese_items);
            }
            Err(e) => {
                log::warn!("Failed to fetch RSS from {}: {}", source_name, e);
            }
        }
    }

    // 按发布时间排序，取最新 4 条
    all_items.sort_by(|a, b| b.published_at.cmp(&a.published_at));
    all_items.truncate(4);

    Ok(all_items)
}

fn fetch_single_feed(source_name: &str, feed_url: &str) -> Result<Vec<NewsItem>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("BridgeAI/1.0 (RSS Reader)")
        .build()
        .map_err(|e| crate::core::error::AppError::Network(e.to_string()))?;

    let resp = client
        .get(feed_url)
        .send()
        .map_err(|e| crate::core::error::AppError::Network(e.to_string()))?;

    let body_bytes = resp
        .bytes()
        .map_err(|e| crate::core::error::AppError::Network(e.to_string()))?;

    let feed = parser::parse(body_bytes.as_ref())
        .map_err(|e| crate::core::error::AppError::ConfigParse(e.to_string()))?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut items = Vec::new();

    for entry in feed.entries.iter().take(5) {
        let title = entry
            .title
            .as_ref()
            .map(|t| t.content.clone())
            .unwrap_or_default();

        let summary = entry
            .summary
            .as_ref()
            .map(|s| s.content.clone())
            .unwrap_or_default()
            .lines()
            .take(3)
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .take(200)
            .collect::<String>();

        let url = entry
            .links
            .first()
            .map(|l| l.href.clone())
            .unwrap_or_default();

        let published_at = entry
            .published
            .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| now.clone());

        let (category, tags) = categorize_and_tag(&title, &summary);

        items.push(NewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            summary,
            source: source_name.to_string(),
            category,
            tags: tags.join(","),
            url,
            published_at,
            fetched_at: now.clone(),
        });
    }

    Ok(items)
}

/// 保存资讯到数据库（URL 去重）
pub fn save_news_to_db(items: &[NewsItem]) -> Result<(usize, usize)> {
    let conn = get_connection("bridge_ai")?;
    let mut saved = 0;
    let mut skipped = 0;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS ai_news (
            id          TEXT PRIMARY KEY,
            title       TEXT NOT NULL,
            summary     TEXT,
            source      TEXT NOT NULL,
            category    TEXT NOT NULL DEFAULT '行业动态',
            tags        TEXT DEFAULT '',
            url         TEXT UNIQUE NOT NULL,
            published_at TEXT NOT NULL,
            fetched_at  TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_news_fetched ON ai_news(fetched_at DESC);
        CREATE INDEX IF NOT EXISTS idx_news_category ON ai_news(category);
        CREATE INDEX IF NOT EXISTS idx_news_tags ON ai_news(tags);"
    )?;

    for item in items {
        match conn.execute(
            "INSERT INTO ai_news (id, title, summary, source, category, tags, url, published_at, fetched_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                item.id,
                item.title,
                item.summary,
                item.source,
                item.category,
                item.tags,
                item.url,
                item.published_at,
                item.fetched_at
            ],
        ) {
            Ok(_) => saved += 1,
            Err(_) => skipped += 1, // URL 重复，跳过
        }
    }

    Ok((saved, skipped))
}

/// 从数据库获取资讯列表
pub fn get_news_from_db(category: Option<&str>, limit: Option<i32>) -> Result<Vec<NewsItem>> {
    let conn = get_connection("bridge_ai")?;
    let limit = limit.unwrap_or(50);

    let sql = if let Some(cat) = category {
        format!(
            "SELECT id, title, summary, source, category, tags, url, published_at, fetched_at
             FROM ai_news WHERE category = '{}' ORDER BY fetched_at DESC LIMIT {}",
            cat.replace('\'', "''"),
            limit
        )
    } else {
        format!(
            "SELECT id, title, summary, source, category, tags, url, published_at, fetched_at
             FROM ai_news ORDER BY fetched_at DESC LIMIT {}",
            limit
        )
    };

    let mut stmt = conn.prepare(&sql)?;
    let items = stmt
        .query_map([], |row| {
            Ok(NewsItem {
                id: row.get(0)?,
                title: row.get(1)?,
                summary: row.get(2)?,
                source: row.get(3)?,
                category: row.get(4)?,
                tags: row.get(5)?,
                url: row.get(6)?,
                published_at: row.get(7)?,
                fetched_at: row.get(8)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(items)
}

/// 获取最后一次采集时间
pub fn get_last_fetch_time() -> Result<Option<String>> {
    let conn = get_connection("bridge_ai")?;
    let result: Option<String> = conn
        .query_row(
            "SELECT MAX(fetched_at) FROM ai_news",
            [],
            |row| row.get(0),
        )
        .ok()
        .flatten();
    Ok(result)
}

/// 清除所有非中文资讯
pub fn clear_non_chinese_news() -> Result<usize> {
    let conn = get_connection("bridge_ai")?;
    
    // 获取所有资讯
    let mut stmt = conn.prepare(
        "SELECT id, title FROM ai_news"
    )?;
    
    let news_items: Vec<(String, String)> = stmt
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?
        .filter_map(|r| r.ok())
        .collect();
    
    let mut deleted_count = 0;
    
    for (id, title) in news_items {
        // 如果标题不包含足够中文字符，删除
        if !is_chinese_content(&title) {
            conn.execute("DELETE FROM ai_news WHERE id = ?1", rusqlite::params![id])?;
            deleted_count += 1;
        }
    }
    
    Ok(deleted_count)
}

/// 启动定时任务（每天 8 点自动采集）
pub fn start_daily_scheduler() -> Result<()> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| crate::core::error::AppError::Internal(e.to_string()))?;

    rt.block_on(async {
        let scheduler = match JobScheduler::new().await {
            Ok(s) => s,
            Err(e) => {
                log::error!("[Bridge AI] 创建调度器失败：{}", e);
                return;
            }
        };

        // 每天 8:00 执行（Cron 表达式：0 0 8 * * *）
        let job = match Job::new_async("0 0 8 * * *", |_uuid, _l| {
            Box::pin(async move {
                log::info!("[Bridge AI] 定时任务触发：开始采集 AI 资讯");
                match fetch_news_from_rss() {
                    Ok(items) => {
                        match save_news_to_db(&items) {
                            Ok((saved, skipped)) => {
                                log::info!("[Bridge AI] 采集完成：新增 {} 条，跳过 {} 条重复", saved, skipped);
                            }
                            Err(e) => {
                                log::error!("[Bridge AI] 保存资讯失败：{}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("[Bridge AI] 采集资讯失败：{}", e);
                    }
                }
            })
        }) {
            Ok(j) => j,
            Err(e) => {
                log::error!("[Bridge AI] 创建定时任务失败：{}", e);
                return;
            }
        };

        if let Err(e) = scheduler.add(job).await {
            log::error!("[Bridge AI] 添加定时任务失败：{}", e);
            return;
        }

        if let Err(e) = scheduler.start().await {
            log::error!("[Bridge AI] 启动调度器失败：{}", e);
            return;
        }

        log::info!("[Bridge AI] 定时任务已启动：每天 8:00 自动采集 AI 资讯");

        // 保持调度器运行
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
        }
    });

    Ok(())
}
