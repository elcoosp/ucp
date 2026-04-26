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

    /// Structured conflict with component and prop context.
    #[error("Conflict in {component}.{prop}: present_in={present_in:?}, absent_in={absent_in:?}")]
    ConflictDetailed {
        component: String,
        prop: String,
        present_in: Vec<String>,
        absent_in: Vec<String>,
    },

    /// LLM inference failure with optional model context.
    #[error("LLM inference failed for model {model:?}: {message}")]
    LlmInferenceDetailed {
        message: String,
        model: Option<String>,
    },
}

/// A `Result` alias for UCP operations.
pub type Result<T> = std::result::Result<T, UcpError>;
