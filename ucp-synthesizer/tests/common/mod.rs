use ucp_core::cam::*;
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};

/// Create a minimal component with disabled (bool) and label (String) props.
pub fn make_minimal_component(name: &str, framework: &str) -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: format!("{}:test.rs:{}", framework, name),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "test".into(),
            normalized_prop_names: vec!["disabled".into(), "label".into()],
        },
        props: vec![
            CanonicalAbstractProp {
                canonical_name: "disabled".into(),
                abstract_type: AbstractPropType::ControlFlag,
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("bool".into()),
                sources: vec![], confidence: 1.0, conflicts: vec![],
            },
            CanonicalAbstractProp {
                canonical_name: "label".into(),
                abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("String".into()),
                sources: vec![], confidence: 1.0, conflicts: vec![],
            },
        ],
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    }
}

/// Create a component with buttons and events for full snapshot testing.
pub fn make_full_component() -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: "rust:button.rs:Button".into(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "abc".into(),
            normalized_prop_names: vec!["disabled".into(), "label".into(), "variant".into()],
        },
        props: vec![
            make_prop("disabled", AbstractPropType::ControlFlag, Some("bool")),
            make_prop("label", AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)), Some("String")),
            make_prop("variant", AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)), Some("enum: Default, Destructive")),
        ],
        events: vec![CanonicalAbstractEvent {
            canonical_name: "click".to_string(),
            abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
        }],
        extracted_state_machine: Some(StateMachine {
            id: "sm".into(), initial: "idle".into(),
            states: [("idle".into(), StateNode { on: None })].into(),
        }),
        extracted_parts: vec![ExtractedPart { name: "trigger".into(), selectable: true }],
        source_repos: vec![SourceAttribution { repo_url: "local".into(), file_path: "button.rs".into(), line_start: 1 }],
        provided_context: None, consumed_contexts: vec![],
    }
}

pub fn make_prop(name: &str, abstract_type: AbstractPropType, concrete: Option<&str>) -> CanonicalAbstractProp {
    CanonicalAbstractProp {
        canonical_name: name.into(), abstract_type,
        reactivity: AbstractReactivity::Static,
        concrete_type: concrete.map(|s| s.into()),
        sources: vec![], confidence: 1.0, conflicts: vec![],
    }
}

pub fn make_empty_spec() -> SynthesisOutput {
    SynthesisOutput {
        ucp_version: "4.0.0".into(), components: vec![],
        stats: PipelineStats { files_scanned: 0, files_parsed: 0, components_found: 0, conflicts_detected: 0, llm_enriched: false },
    }
}

pub fn make_package_manifest(name: &str, comps: Vec<CanonicalAbstractComponent>) -> PackageManifest {
    PackageManifest {
        name: name.into(), version: "0.1.0".into(), frameworks: vec!["dioxus".into()],
        components: comps, global_styles: None, generated_by: "test".into(), generated_at: "now".into(),
    }
}
