use thiserror::Error;

#[derive(Debug, Error)]
pub enum UcpError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("SPDX License error: {0}")]
    License(String),

    #[error("LLM Inference error: {0}")]
    LlmInference(String),

    #[error("Unification conflict: {0}")]
    Conflict(String),

    #[error("Parsing error: {0}")]
    Parsing(String),
}

pub type Result<T> = std::result::Result<T, UcpError>;
