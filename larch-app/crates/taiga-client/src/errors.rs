use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaigaClientError {
    #[error("HTTP Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Failed to parse JSON response")]
    JsonParse,

    #[error("Unknown error")]
    Unknown,
}
