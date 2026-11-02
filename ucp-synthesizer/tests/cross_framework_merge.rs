//! Cross-framework unification integration tests.
//!
//! These pin the 0.13 flagship behavior: the same component extracted from a
//! React port and a Leptos port merges into ONE `unified:` component, with
//! prop-set differences surfaced as presence conflicts instead of blocking
//! the merge.

use ucp_core::cam::AbstractPropType;
use ucp_synthesizer::extract::rust_ast::extract_rust_components;
use ucp_synthesizer::extract::tsx_ast::extract_tsx_components;
use ucp_synthesizer::merge::{merge_specs, MergeOptions};
use ucp_synthesizer::pipeline::output::{PipelineStats, SynthesisOutput};
use ucp_synthesizer::pipeline::unification::{unify_rust_component, unify_tsx_component};

const REACT_BUTTON_TSX: &str = r#"
export interface ButtonProps {
  variant?: "default" | "destructive" | "outline";
  size?: "sm" | "default" | "lg";
  disabled?: boolean;
  onClick?: (e: React.MouseEvent) => void;
  children?: React.ReactNode;
  className?: string;
}

export const Button = (props: ButtonProps) => <button className={props.className} />;
"#;

const LEPTOS_BUTTON_RS: &str = r"
use leptos::*;

#[component]
pub fn Button(
    #[prop(optional)] variant: String,
    #[prop(optional)] size: String,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional)] on_click: Callback<leptos::ev::MouseEvent>,
    children: Children,
) -> impl IntoView {
    view! { <button disabled=disabled on:click=on_click>{children}</button> }
}
";

fn react_spec() -> SynthesisOutput {
    let raw = &extract_tsx_components(REACT_BUTTON_TSX).unwrap()[0];
    let comp = unify_tsx_component(raw, "web/src/button.tsx").unwrap();
    spec_of(vec![comp])
}

fn leptos_spec() -> SynthesisOutput {
    let raw = &extract_rust_components(LEPTOS_BUTTON_RS).unwrap()[0];
    let comp = unify_rust_component(raw, "native/src/button.rs").unwrap();
    spec_of(vec![comp])
}

fn spec_of(components: Vec<ucp_core::cam::CanonicalAbstractComponent>) -> SynthesisOutput {
    let found = components.len();
    SynthesisOutput {
        ucp_version: ucp_core::UCP_SPEC_VERSION.to_string(),
        components,
        provenance: None,
        curation_log: None,
        stats: PipelineStats {
            files_scanned: 1,
            files_parsed: 1,
            components_found: found,
            conflicts_detected: 0,
            cache_hits: 0,
        },
    }
}

fn normalized_prop_names(comp: &ucp_core::cam::CanonicalAbstractComponent) -> Vec<String> {
    comp.props
        .iter()
        .map(|p| ucp_core::naming::normalize_prop_name(&p.canonical_name))
        .collect()
}

/// React `Button` + Leptos `Button` → exactly one unified component.
#[test]
fn cross_framework_buttons_unify_into_one_component() {
    let merged = merge_specs(&[react_spec(), leptos_spec()], MergeOptions::default()).unwrap();

    assert_eq!(merged.components.len(), 1, "one Button, not two ports");
    let button = &merged.components[0];
    assert_eq!(button.id, "unified:Button");

    let names = normalized_prop_names(button);
    for expected in ["variant", "size", "disabled", "click", "children"] {
        assert!(
            names.iter().any(|n| n == expected),
            "missing normalized prop {expected}; got {names:?}"
        );
    }
    // React-only prop also present.
    assert!(names.contains(&"classname".to_string()));
}

/// The `click` prop from both ports carries source mappings from both files.
#[test]
fn shared_props_carry_sources_from_both_ports() {
    let merged = merge_specs(&[react_spec(), leptos_spec()], MergeOptions::default()).unwrap();
    let button = &merged.components[0];
    let click = button
        .props
        .iter()
        .find(|p| ucp_core::naming::normalize_prop_name(&p.canonical_name) == "click")
        .expect("click prop merged from onClick + on_click");
    let repos: std::collections::BTreeSet<_> =
        click.sources.iter().map(|s| s.repo_id.as_str()).collect();
    assert!(repos.contains("web/src/button.tsx"), "{repos:?}");
    assert!(repos.contains("native/src/button.rs"), "{repos:?}");
}

/// A prop that exists in only one port is a presence conflict, not silence.
#[test]
fn prop_missing_from_one_port_is_a_presence_conflict() {
    let merged = merge_specs(&[react_spec(), leptos_spec()], MergeOptions::default()).unwrap();
    let button = &merged.components[0];
    let classname = button
        .props
        .iter()
        .find(|p| ucp_core::naming::normalize_prop_name(&p.canonical_name) == "classname")
        .expect("className prop present");
    assert!(
        !classname.conflicts.is_empty(),
        "presence drift must be flagged"
    );
    let conflict = &classname.conflicts[0];
    assert!(conflict.present_in.iter().any(|s| s.contains("button.tsx")));
    assert!(conflict.absent_in.iter().any(|s| s.contains("button.rs")));
}

/// `disabled` maps to ControlFlag-based types on both ports (no more `any`).
#[test]
fn boolean_props_are_control_flags_on_both_ports() {
    let merged = merge_specs(&[react_spec(), leptos_spec()], MergeOptions::default()).unwrap();
    let button = &merged.components[0];
    let disabled = button
        .props
        .iter()
        .find(|p| ucp_core::naming::normalize_prop_name(&p.canonical_name) == "disabled")
        .unwrap();
    let innermost = disabled.abstract_type.innermost();
    assert_eq!(
        innermost,
        &AbstractPropType::ControlFlag,
        "boolean prop must reduce to ControlFlag, got {:?}",
        disabled.abstract_type
    );
}

/// Same input, same order → byte-identical merged spec (determinism).
#[test]
fn merge_is_deterministic() {
    let inputs = [react_spec(), leptos_spec()];
    let a = merge_specs(&inputs, MergeOptions::default()).unwrap();
    let b = merge_specs(&inputs, MergeOptions::default()).unwrap();
    assert_eq!(
        serde_json::to_string_pretty(&a).unwrap(),
        serde_json::to_string_pretty(&b).unwrap()
    );
}

/// Events are derived with normalized names from both spellings.
#[test]
fn events_are_normalized_across_spellings() {
    let merged = merge_specs(&[react_spec(), leptos_spec()], MergeOptions::default()).unwrap();
    let button = &merged.components[0];
    let events: Vec<_> = button
        .events
        .iter()
        .map(|e| e.canonical_name.clone())
        .collect();
    assert!(events.contains(&"click".to_string()), "events: {events:?}");
    assert!(
        !events.iter().any(|e| e.starts_with("on")),
        "events: {events:?}"
    );
}
