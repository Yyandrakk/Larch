use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("Taiga client error: {0}")]
    TaigaClient(String),

    #[error("Keyring error: {0}")]
    Keyring(String),

    #[error("Serde error: {0}")]
    Serde(String),

    #[error("Tauri error: {0}")]
    Tauri(String),

    #[error("URL parse error: {0}")]
    UrlParse(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("IO error: {0}")]
    Io(String),
}

impl From<taiga_client::errors::TaigaClientError> for Error {
    fn from(e: taiga_client::errors::TaigaClientError) -> Self {
        Error::TaigaClient(e.to_string())
    }
}

impl From<keyring::Error> for Error {
    fn from(e: keyring::Error) -> Self {
        Error::Keyring(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e.to_string())
    }
}

impl From<tauri::Error> for Error {
    fn from(e: tauri::Error) -> Self {
        Error::Tauri(e.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::UrlParse(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
