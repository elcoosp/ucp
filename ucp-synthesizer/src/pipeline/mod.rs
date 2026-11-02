//! End-to-end synthesis pipeline: walk, parse, unify, enrich, report.

/// Incremental parse cache (0.13.0).
pub mod cache;
/// Fingerprint-based conflict detection.
pub mod conflicts;
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
