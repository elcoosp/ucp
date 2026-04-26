use std::fs;
use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::export::w3c::export_w3c;
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};

#[test]
fn export_full_w3c_with_anatomy_states_variants() {
    let tmp = TempDir::new().unwrap();

    let comp = CanonicalAbstractComponent {
        id: "rust:accordion.rs:Accordion".to_string(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "acc".into(),
            normalized_prop_names: vec!["multiple".into(), "variant".into()],
        },
        props: vec![
            CanonicalAbstractProp {
                canonical_name: "multiple".into(),
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
        events: vec![
            CanonicalAbstractEvent {
                canonical_name: "toggle".to_string(),
                abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
            },
        ],
        extracted_state_machine: Some(StateMachine {
            id: "sm1".into(),
            initial: "Closed".into(),
            states: [
                ("Closed".into(), StateNode {
                    on: Some([("OPEN".into(), Transition { target: "Open".into(), side_effects: vec![] })].into()),
                }),
                ("Open".into(), StateNode {
                    on: Some([("CLOSE".into(), Transition { target: "Closed".into(), side_effects: vec!["focus: move_to".into()] })].into()),
                }),
            ].into(),
        }),
        extracted_parts: vec![
            ExtractedPart { name: "trigger".into(), selectable: true },
            ExtractedPart { name: "content".into(), selectable: false },
        ],
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    };

    let output = SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![comp],
        stats: PipelineStats {
            files_scanned: 1,
            files_parsed: 1,
            components_found: 1,
            conflicts_detected: 0,
            llm_enriched: false,
        },
    };

    export_w3c(&output, &tmp.path().to_string_lossy()).unwrap();
    let content = fs::read_to_string(tmp.path().join("ucp-spec.w3c.json")).unwrap();

    assert!(content.contains("Accordion"));
    assert!(content.contains("anatomy"));
    assert!(content.contains("trigger"));
    assert!(content.contains("content"));
    assert!(content.contains("states"));
    assert!(content.contains("Closed"));
    assert!(content.contains("Open"));
    assert!(content.contains("variants"));
    assert!(content.contains("Default"));
    assert!(content.contains("Destructive"));
    assert!(content.contains("Outline"));
    assert!(content.contains("toggle"));
}
