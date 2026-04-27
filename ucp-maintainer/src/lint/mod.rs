//! `ucp lint` — spec compliance linter for UCP.
//!
//! Extracts component usages from source files and checks them against the
//! canonical spec, reporting violations for unknown props, missing required props,
//! type mismatches, enum violations, and more.

pub mod usage;
pub mod rules;
pub mod extractors;
pub mod output;

pub use usage::{ComponentUsage, PropAssignment, DetectedType};
pub use rules::{Rule, RuleViolation, Severity, LintResult, LintSummary};

use std::path::PathBuf;
use ucp_synthesizer::pipeline::SynthesisOutput;

pub struct LintConfig {
    pub spec: PathBuf,
    pub source: PathBuf,
    pub format: String,
    pub rules: Option<Vec<String>>,
    pub ignore: Option<Vec<String>>,
}

pub async fn run_lint(_config: &LintConfig) -> anyhow::Result<LintResult> {
    todo!("v0.14 — implemented in later chunks")
}
