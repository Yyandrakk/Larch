use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaigaClientError {
    #[error("HTTP Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Authentication failed with status: {0}")]
    AuthFailed(StatusCode),

    #[error("Unknown error")]
    Unknown,
}
