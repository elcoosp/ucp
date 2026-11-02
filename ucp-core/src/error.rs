//! Unified error type for all UCP crates.

use thiserror::Error;

/// Errors that can occur during UCP synthesis, parsing, and I/O.
#[derive(Debug, Error)]
pub enum UcpError {
    /// Filesystem I/O failure.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON (de)serialization failure.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// SPDX license parsing or policy rejection.
    #[error("SPDX License error: {0}")]
    License(String),

    /// LLM inference transport or protocol failure.
    #[error("LLM Inference error: {0}")]
    LlmInference(String),

    /// HTTP client failure (GitHub discovery, Ollama transport).
    #[error("HTTP error: {0}")]
    Http(String),

    /// Unification conflict that cannot be resolved automatically.
    #[error("Unification conflict: {0}")]
    Conflict(String),

    /// Source-code or SMDL parsing failure.
    #[error("Parsing error: {0}")]
    Parsing(String),

    /// Structured conflict with component and prop context.
    #[error("Conflict in {component}.{prop}: present_in={present_in:?}, absent_in={absent_in:?}")]
    ConflictDetailed {
        /// Component where the conflict was detected.
        component: String,
        /// Prop that disagrees between sources.
        prop: String,
        /// Sources that define the prop.
        present_in: Vec<String>,
        /// Sources that lack the prop.
        absent_in: Vec<String>,
    },

    /// LLM inference failure with optional model context.
    #[error("LLM inference failed for model {model:?}: {message}")]
    LlmInferenceDetailed {
        /// Human-readable failure description.
        message: String,
        /// Model identifier, when known.
        model: Option<String>,
    },
}

/// A `Result` alias for UCP operations.
pub type Result<T> = std::result::Result<T, UcpError>;
