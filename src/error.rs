//! The library's error type.

/// An error from one of the library's fallible functions.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The input was not a valid greeting document.
    #[error("invalid greeting JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// The greeting's name was empty or only whitespace.
    #[error("a greeting needs a non-blank name")]
    BlankName,
}
