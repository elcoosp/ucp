//! Pipeline output types: synthesis results, stats, options, and records.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ucp_core::cam::{CanonicalAbstractComponent, PackageManifest};

/// Base extraction confidence for Rust sources.
pub const BASE_CONFIDENCE_RUST: f32 = 0.95;
/// Base extraction confidence for TSX/TS sources.
pub const BASE_CONFIDENCE_TSX: f32 = 0.90;
/// Confidence penalty per unclassifiable (`Any`) prop.
pub const ANY_PENALTY_PER_PROP: f32 = 0.08;

/// Provenance record describing one merged input spec.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MergeRecord {
    /// Label of the input (e.g. `input-0`).
    pub source_spec_path: String,
    /// Cheap fingerprint of the input (component count).
    pub source_fingerprint: String,
    /// Unix timestamp of the merge.
    pub merged_at: String,
}

/// A curation decision recorded while resolving conflicts.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CurationDecision {
    /// Conflict the decision applies to.
    pub conflict_id: String,
    /// Resolution strategy chosen (Debug spelling).
    pub chosen_resolution: String,
    /// Optional human rationale.
    pub rationale: Option<String>,
    /// Unix timestamp of the decision.
    pub timestamp: String,
}

/// Top-level output of the UCP synthesis pipeline.
///
/// This is the canonical spec format: it is serialized to
/// `ucp-spec.json` and consumed by merge, curation, export, and the
/// code generators.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[must_use = "discarding synthesis output loses all extracted component data"]
pub struct SynthesisOutput {
    /// Version of the UCP specification this output conforms to.
    pub ucp_version: String,
    /// Extracted (or merged) components.
    pub components: Vec<CanonicalAbstractComponent>,
    /// Pipeline statistics.
    pub stats: PipelineStats,
    /// Provenance records, when this spec is the result of a merge.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub provenance: Option<Vec<MergeRecord>>,
    /// Curation log, when conflict resolutions have been applied.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub curation_log: Option<Vec<CurationDecision>>,
}

/// Counters collected while running the extraction pipeline.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct PipelineStats {
    /// Files considered during the scan.
    pub files_scanned: usize,
    /// Files that yielded at least one component.
    pub files_parsed: usize,
    /// Components found before deduplication.
    pub components_found: usize,
    /// Conflict attachments detected.
    pub conflicts_detected: usize,
    /// Whether any component was enriched by an LLM.
    pub llm_enriched: bool,
}

/// Configuration for the extraction pipeline.
#[derive(Debug, Clone)]
pub struct PipelineOptions {
    /// Ollama base URL; `None` disables LLM enrichment.
    pub ollama_url: Option<String>,
    /// Model identifier used for enrichment prompts.
    pub llm_model: String,
    /// Skip LLM enrichment even when `ollama_url` is set.
    pub dry_run: bool,
}

impl Default for PipelineOptions {
    fn default() -> Self {
        Self {
            ollama_url: None,
            llm_model: "glm-5:cloud".to_string(),
            dry_run: false,
        }
    }
}

impl SynthesisOutput {
    /// Load a spec from a JSON file.
    ///
    /// # Errors
    ///
    /// Returns I/O errors when the file cannot be read and JSON errors
    /// when the content is not a valid spec.
    pub fn load_from_file(path: &std::path::Path) -> ucp_core::Result<Self> {
        let content = std::fs::read_to_string(path).map_err(ucp_core::UcpError::Io)?;
        serde_json::from_str(&content).map_err(ucp_core::UcpError::Json)
    }

    /// Save the spec as pretty-printed JSON, creating parent directories.
    ///
    /// # Errors
    ///
    /// Returns I/O errors when directories cannot be created or the file
    /// cannot be written, and JSON errors on serialization failure.
    pub fn save_to_file(&self, path: &std::path::Path) -> ucp_core::Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(ucp_core::UcpError::Json)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(ucp_core::UcpError::Io)?;
        }
        std::fs::write(path, json).map_err(ucp_core::UcpError::Io)?;
        Ok(())
    }

    /// Convert into a [`PackageManifest`] for the code generators.
    #[must_use]
    pub fn to_package_manifest(
        &self,
        name: &str,
        version: &str,
        frameworks: Vec<String>,
    ) -> PackageManifest {
        PackageManifest {
            name: name.to_string(),
            version: version.to_string(),
            frameworks,
            components: self.components.clone(),
            global_styles: None,
            generated_by: format!("ucp v{}", self.ucp_version),
            generated_at: crate::utils::unix_timestamp_string(),
        }
    }

    /// Count unresolved conflict attachments across all components.
    #[must_use]
    pub fn unresolved_conflicts(&self) -> usize {
        self.components
            .iter()
            .map(|c| c.props.iter().filter(|p| !p.conflicts.is_empty()).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_default_is_zeroed() {
        let stats = PipelineStats::default();
        assert_eq!(stats.files_scanned, 0);
        assert!(!stats.llm_enriched);
    }

    #[test]
    fn unresolved_conflicts_counts_attachments() {
        use ucp_core::cam::{
            AbstractPropType, AbstractReactivity, CanonicalAbstractProp, Conflict,
            ResolutionStrategy, SemanticFingerprint,
        };
        let spec = SynthesisOutput {
            ucp_version: "4.0.0".into(),
            components: vec![CanonicalAbstractComponent {
                id: "x".into(),
                semantic_fingerprint: SemanticFingerprint {
                    purpose_hash: "h".into(),
                    normalized_prop_names: vec!["a".into()],
                },
                props: vec![CanonicalAbstractProp {
                    canonical_name: "a".into(),
                    abstract_type: AbstractPropType::ControlFlag,
                    reactivity: AbstractReactivity::Static,
                    concrete_type: None,
                    sources: vec![],
                    confidence: 1.0,
                    conflicts: vec![Conflict {
                        id: "c1".into(),
                        field: "props.a".into(),
                        present_in: vec![],
                        absent_in: vec![],
                        confidence: 0.5,
                        resolution_suggestion: ResolutionStrategy::IncludeMajority,
                    }],
                }],
                events: vec![],
                extracted_state_machine: None,
                extracted_parts: vec![],
                source_repos: vec![],
                provided_context: None,
                consumed_contexts: vec![],
            }],
            stats: PipelineStats::default(),
            provenance: None,
            curation_log: None,
        };
        assert_eq!(spec.unresolved_conflicts(), 1);
    }
}
