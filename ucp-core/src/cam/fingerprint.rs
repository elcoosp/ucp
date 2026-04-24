use serde::{Deserialize, Serialize};

/// A hash-based fingerprint used to group semantically equivalent components
/// across different codebases for conflict detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticFingerprint {
    pub purpose_hash: String,
    pub normalized_prop_names: Vec<String>,
}
