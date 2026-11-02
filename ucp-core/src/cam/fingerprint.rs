//! Semantic fingerprint used to cluster equivalent components.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A hash-based fingerprint used to group semantically equivalent components
/// across different codebases for conflict detection.
///
/// Two components whose [`purpose_hash`](SemanticFingerprint::purpose_hash)
/// matches are considered candidates for unification. The hash is computed
/// from the lower-cased component name plus its sorted prop names, so it is
/// stable across runs and Rust versions (FNV-1a, see [`ucp_core::fnv1a64`]).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SemanticFingerprint {
    /// Hex-encoded stable hash of the component purpose.
    pub purpose_hash: String,
    /// Sorted, normalized prop names that contributed to the hash.
    pub normalized_prop_names: Vec<String>,
}
