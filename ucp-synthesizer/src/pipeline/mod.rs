//! End-to-end synthesis pipeline: walk, parse, unify, enrich, report.

/// Fingerprint-based conflict detection.
pub mod conflicts;
/// Optional LLM enrichment stage.
pub mod enrichment;
/// Directory walk + parallel parse + unify driver.
pub mod extraction;
/// Output/stats/options types.
pub mod output;
/// Raw-extraction -> CAM unification.
pub mod unification;

pub use extraction::run_pipeline;
pub use extraction::run_pipeline_with_options;
pub use output::{
    PipelineOptions, PipelineStats, SynthesisOutput, ANY_PENALTY_PER_PROP, BASE_CONFIDENCE_RUST,
    BASE_CONFIDENCE_TSX,
};
