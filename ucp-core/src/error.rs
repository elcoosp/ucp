use thiserror::Error;

#[derive(Debug, Error)]
pub enum UcpError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("SPDX License error: {0}")]
    License(String),

    #[error("LLM Inference error: {0}")]
    LlmInference(String),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Unification conflict: {0}")]
    Conflict(String),

    #[error("Parsing error: {0}")]
    Parsing(String),
}

/// A `Result` alias for UCP operations.
/// Note: `#[must_use]` cannot be applied to type aliases in Rust.
/// The inner `Result` already carries `#[must_use]` from the standard library.
pub type Result<T> = std::result::Result<T, UcpError>;
