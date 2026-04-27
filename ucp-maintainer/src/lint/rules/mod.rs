//! Lint rules for spec conformance checking.

use std::path::PathBuf;
use crate::lint::usage::ComponentUsage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Note => write!(f, "note"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuleViolation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub rule_id: String,
    pub severity: Severity,
    pub component: String,
    pub message: String,
    pub suggestion: Option<String>,
}

pub trait Rule: Send + Sync {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn default_severity(&self) -> Severity;
    fn check(&self, usage: &ComponentUsage, spec: &ucp_synthesizer::pipeline::SynthesisOutput) -> Option<RuleViolation>;
}

#[derive(Debug, Clone)]
pub struct LintSummary {
    pub errors: usize,
    pub warnings: usize,
    pub clean: bool,
}

impl LintSummary {
    pub fn new(errors: usize, warnings: usize) -> Self {
        Self { errors, warnings, clean: errors == 0 && warnings == 0 }
    }
}

#[derive(Debug, Clone)]
pub struct LintResult {
    pub files_scanned: usize,
    pub violations: Vec<RuleViolation>,
    pub summary: LintSummary,
}

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_display() {
        assert_eq!(format!("{}", Severity::Error), "error");
        assert_eq!(format!("{}", Severity::Warning), "warning");
        assert_eq!(format!("{}", Severity::Note), "note");
    }

    #[test]
    fn summary_clean_when_empty() {
        assert!(LintSummary::new(0, 0).clean);
    }

    #[test]
    fn summary_dirty_with_errors() {
        assert!(!LintSummary::new(2, 1).clean);
    }
}
