use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaigaClientError {
    #[error("HTTP Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("The requested endpoint was not found (404).")]
    EndpointNotFound(StatusCode),

    #[error("Invalid credentials or insufficient permissions (401/403).")]
    Unauthorized(StatusCode),

    #[error("Authentication failed with status: {0}")]
    AuthFailed(StatusCode),

    #[error("Version conflict: the resource was modified by another user (412)")]
    VersionConflict(StatusCode),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Invalid MIME type: {0}")]
    InvalidMimeType(String),

    #[error("Unknown error")]
    Unknown,
}
