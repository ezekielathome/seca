pub mod api;

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

    /// Typically occurs if the request is missing information or api key is wrong
    #[error("not found")]
    SecaNotFound(),
    /// The provided encrypted app ticket is invalid
    #[error("invalid encrypted app ticket")]
    SecaInvalidSteam(),
    /// Unknown seca error
    #[error("unknown seca error: {0}")]
    SecaGenericError(serde_json::Error),
}
