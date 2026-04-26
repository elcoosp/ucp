use std::fs;
use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::export::a2ui::export_a2ui;
use ucp_synthesizer::export::ag_ui::export_ag_ui;
use ucp_synthesizer::pipeline::{PipelineStats, SynthesisOutput};

fn make_button_component() -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: "rust:button.rs:Button".into(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "abc".into(),
            normalized_prop_names: vec!["disabled".into(), "variant".into()],
        },
        props: vec![
            CanonicalAbstractProp {
                canonical_name: "disabled".into(),
                abstract_type: AbstractPropType::ControlFlag,
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("bool".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            },
            CanonicalAbstractProp {
                canonical_name: "variant".into(),
                abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("enum: Default, Destructive, Outline".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            },
        ],
        events: vec![CanonicalAbstractEvent {
            canonical_name: "click".to_string(),
            abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
        }],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    }
}

#[test]
fn a2ui_export_contains_component_with_variants() {
    let tmp = TempDir::new().unwrap();
    let output = SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![make_button_component()],
        stats: PipelineStats {
            files_scanned: 1,
            files_parsed: 1,
            components_found: 1,
            conflicts_detected: 0,
            llm_enriched: false,
        },
    };
    export_a2ui(&output, "test-lib", "1.0.0", &tmp.path().to_string_lossy()).unwrap();
    let content = fs::read_to_string(tmp.path().join("a2ui-catalog.json")).unwrap();
    assert!(content.contains("Button"));
    assert!(content.contains("Default"));
    assert!(content.contains("Destructive"));
    assert!(content.contains("Outline"));
}

#[test]
fn ag_ui_export_contains_events() {
    let tmp = TempDir::new().unwrap();
    let output = SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![make_button_component()],
        stats: PipelineStats {
            files_scanned: 1,
            files_parsed: 1,
            components_found: 1,
            conflicts_detected: 0,
            llm_enriched: false,
        },
    };
    export_ag_ui(&output, &tmp.path().to_string_lossy()).unwrap();
    let content = fs::read_to_string(tmp.path().join("ag-ui-events.json")).unwrap();
    assert!(content.contains("component.click"));
    assert!(content.contains("Button"));
}
