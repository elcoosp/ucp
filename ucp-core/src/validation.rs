//! Structural validation for component sets.
//!
//! Specs are consumed by generators, exporters, and CI. Consumers need a
//! machine-readable statement of whether a spec is structurally sound:
//! unique ids, non-empty prop names, in-range confidences, and a warning
//! when a component declares no props at all.

use crate::cam::CanonicalAbstractComponent;

/// Issue severity for a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Spec is unusable downstream; CI must fail.
    Error,
    /// Suspicious but not blocking.
    Warning,
}

/// One structural validation finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    /// Stable machine-readable code (e.g. `duplicate-id`).
    pub code: &'static str,
    /// Severity of the finding.
    pub severity: Severity,
    /// Component id the finding applies to, when applicable.
    pub component_id: String,
    /// Human-readable explanation.
    pub message: String,
}

/// Validate a set of components for structural soundness.
///
/// The result is deterministic: issues are ordered by component id, then by
/// code, so CI output is diff-friendly.
#[must_use]
pub fn validate_component_set(components: &[CanonicalAbstractComponent]) -> Vec<ValidationIssue> {
    let mut issues: Vec<ValidationIssue> = Vec::new();

    // Duplicate ids.
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for comp in components {
        if !seen.insert(comp.id.as_str()) {
            issues.push(ValidationIssue {
                code: "duplicate-id",
                severity: Severity::Error,
                component_id: comp.id.clone(),
                message: format!("duplicate component id `{}`", comp.id),
            });
        }
    }

    for comp in components {
        if comp.props.is_empty() {
            issues.push(ValidationIssue {
                code: "component-no-props",
                severity: Severity::Warning,
                component_id: comp.id.clone(),
                message: format!("component `{}` declares no props", comp.id),
            });
        }
        for prop in &comp.props {
            if prop.canonical_name.is_empty() {
                issues.push(ValidationIssue {
                    code: "prop-empty-name",
                    severity: Severity::Error,
                    component_id: comp.id.clone(),
                    message: format!(
                        "component `{}` has a prop with an empty canonical name",
                        comp.id
                    ),
                });
            }
            if !(0.0..=1.0).contains(&prop.confidence) {
                issues.push(ValidationIssue {
                    code: "prop-confidence-range",
                    severity: Severity::Error,
                    component_id: comp.id.clone(),
                    message: format!(
                        "component `{}` prop `{}` has confidence {} outside [0, 1]",
                        comp.id, prop.canonical_name, prop.confidence
                    ),
                });
            }
        }
    }

    issues.sort_by(|a, b| (&a.component_id, a.code).cmp(&(&b.component_id, b.code)));
    issues
}

/// Whether the issue list blocks downstream consumption (any Error present).
#[must_use]
pub fn has_blocking_issues(issues: &[ValidationIssue]) -> bool {
    issues.iter().any(|i| i.severity == Severity::Error)
}
