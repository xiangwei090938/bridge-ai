use rusqlite::Connection;
use std::sync::Mutex;
use crate::core::error::Result;

static DB_CONNECTION: std::sync::OnceLock<Mutex<Connection>> = std::sync::OnceLock::new();

pub fn init_database(db_path: &std::path::Path) -> Result<()> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS conversations (
            id          TEXT PRIMARY KEY,
            title       TEXT,
            model_id    TEXT NOT NULL,
            provider_id TEXT NOT NULL,
            system_prompt TEXT,
            context_length INTEGER DEFAULT 4096,
            is_archived INTEGER DEFAULT 0,
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS messages (
            id              TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
            role            TEXT NOT NULL,
            content         TEXT NOT NULL,
            tokens          INTEGER,
            is_streaming    INTEGER DEFAULT 0,
            created_at      TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS installed_skills (
            id          TEXT PRIMARY KEY,
            skill_name  TEXT NOT NULL,
            skill_version TEXT NOT NULL DEFAULT '1.0.0',
            description TEXT,
            author      TEXT,
            install_path TEXT NOT NULL,
            is_enabled  INTEGER DEFAULT 1,
            size_bytes  INTEGER DEFAULT 0,
            installed_at TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS app_settings (
            key     TEXT PRIMARY KEY,
            value   TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS ai_news (
            id          TEXT PRIMARY KEY,
            title       TEXT NOT NULL,
            summary     TEXT,
            source      TEXT NOT NULL,
            category    TEXT NOT NULL DEFAULT '行业动态',
            url         TEXT UNIQUE NOT NULL,
            published_at TEXT NOT NULL,
            fetched_at  TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id, created_at);
        CREATE INDEX IF NOT EXISTS idx_conversations_updated ON conversations(updated_at DESC);
        CREATE INDEX IF NOT EXISTS idx_news_fetched ON ai_news(fetched_at DESC);
        CREATE INDEX IF NOT EXISTS idx_news_category ON ai_news(category);"
    )?;

    DB_CONNECTION.set(Mutex::new(conn)).ok();
    Ok(())
}

pub fn get_connection(_db_path: &str) -> Result<std::sync::MutexGuard<'static, Connection>> {
    let guard = DB_CONNECTION.get().ok_or_else(|| {
        crate::core::error::AppError::Database("Database not initialized".into())
    })?;
    Ok(guard.lock().unwrap())
}

pub fn get_skill_dir() -> std::path::PathBuf {
    crate::util::fs::BridgeConfig::skills_dir()
}
