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

    #[error(
        "Version conflict: the issue was modified by another user. Please refresh and try again."
    )]
    VersionConflict,
}

impl From<taiga_client::errors::TaigaClientError> for Error {
    /// Convert a `taiga_client::errors::TaigaClientError` into the crate's centralized `Error`.
    ///
    /// Maps `TaigaClientError::VersionConflict(_)` to `Error::VersionConflict`. All other
    /// `TaigaClientError` variants are converted to `Error::TaigaClient` containing the
    /// source error's string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// // Construct a Taiga client error and convert it into the crate `Error`.
    /// let tc_err = taiga_client::errors::TaigaClientError::VersionConflict(String::from("conflict"));
    /// let err: Error = tc_err.into();
    /// assert!(matches!(err, Error::VersionConflict));
    /// ```
    fn from(e: taiga_client::errors::TaigaClientError) -> Self {
        match e {
            taiga_client::errors::TaigaClientError::VersionConflict(_) => Error::VersionConflict,
            other => Error::TaigaClient(other.to_string()),
        }
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