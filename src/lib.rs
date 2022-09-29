pub mod api;
pub mod stats;

use thiserror::Error;

/// Error type for seca
#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred in the reqwest library (HTTP)
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    /// An error occurred when serializing/deserializing JSON
    #[error("error in json serialization/deserialization: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    /// An error occurred when parsing a URL
    #[error("error when parsing URL: {0}")]
    UrlParseError(#[from] url::ParseError),
    /// Generic SECA error, typically occurs if api key or auth ticket is wrong.
    #[error("not found")]
    SecaNotFound(),
}
