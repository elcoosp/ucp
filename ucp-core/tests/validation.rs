//! Structural validation contract for UCP specs (0.13.0).

use ucp_core::cam::{
    AbstractPropType, CanonicalAbstractComponent, CanonicalAbstractProp, PropSourceMapping,
    SemanticFingerprint,
};
use ucp_core::validation::{validate_component_set, Severity};

fn prop(name: &str) -> CanonicalAbstractProp {
    CanonicalAbstractProp {
        canonical_name: name.to_string(),
        abstract_type: AbstractPropType::ControlFlag,
        reactivity: ucp_core::cam::AbstractReactivity::Static,
        concrete_type: None,
        sources: vec![PropSourceMapping {
            repo_id: "a.tsx".to_string(),
            original_name: name.to_string(),
            original_type: "boolean".to_string(),
        }],
        confidence: 0.9,
        conflicts: vec![],
    }
}

fn comp(id: &str) -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: id.to_string(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "deadbeef".to_string(),
            normalized_prop_names: vec![],
        },
        props: vec![prop("disabled")],
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![ucp_core::cam::SourceAttribution {
            repo_url: "local".to_string(),
            file_path: "a.tsx".to_string(),
            line_start: 1,
        }],
        provided_context: None,
        consumed_contexts: vec![],
    }
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn healthy_spec_has_no_issues() {
    let issues = validate_component_set(&[comp("tsx:a.tsx:Button"), comp("rust:a.rs:Button")]);
    assert!(issues.is_empty(), "unexpected issues: {issues:?}");
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn duplicate_component_ids_are_errors() {
    let issues = validate_component_set(&[comp("tsx:a.tsx:Button"), comp("tsx:a.tsx:Button")]);
    assert!(issues
        .iter()
        .any(|i| i.severity == Severity::Error && i.code == "duplicate-id"));
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn props_without_name_are_errors() {
    let mut c = comp("tsx:a.tsx:Button");
    c.props[0].canonical_name = String::new();
    let issues = validate_component_set(&[c]);
    assert!(issues
        .iter()
        .any(|i| i.severity == Severity::Error && i.code == "prop-empty-name"));
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn out_of_range_confidence_is_an_error() {
    let mut c = comp("tsx:a.tsx:Button");
    c.props[0].confidence = 1.5;
    let issues = validate_component_set(&[c]);
    assert!(issues
        .iter()
        .any(|i| i.severity == Severity::Error && i.code == "prop-confidence-range"));
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn components_without_props_warn() {
    let mut c = comp("tsx:a.tsx:Button");
    c.props.clear();
    let issues = validate_component_set(&[c]);
    assert!(issues
        .iter()
        .any(|i| i.severity == Severity::Warning && i.code == "component-no-props"));
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn issue_message_names_the_component() {
    let issues = validate_component_set(&[comp("tsx:a.tsx:Button"), comp("tsx:a.tsx:Button")]);
    let dup = issues
        .iter()
        .find(|i| i.code == "duplicate-id")
        .expect("duplicate id issue");
    assert!(dup.message.contains("tsx:a.tsx:Button"));
}
