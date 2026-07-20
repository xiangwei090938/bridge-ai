use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("API key error: {0}")]
    ApiKey(String),

    #[error("File IO error: {0}")]
    FileIo(String),

    #[error("Model connection error: {0}")]
    ModelConnection(String),

    #[error("Permission denied: {0}")]
    Permission(String),

    #[error("Config parse error: {0}")]
    ConfigParse(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Database(e.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Network(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::ConfigParse(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::FileIo(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
