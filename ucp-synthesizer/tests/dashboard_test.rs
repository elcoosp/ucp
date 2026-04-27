use std::fs;
use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::dashboard::generator::generate_dashboard;
use ucp_synthesizer::pipeline::{PipelineStats, SynthesisOutput};

#[test]
fn dashboard_creates_valid_html() {
    let tmp = TempDir::new().unwrap();

    let comp = CanonicalAbstractComponent {
        id: "rust:button.rs:Button".to_string(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "abc".into(),
            normalized_prop_names: vec!["disabled".into(), "label".into()],
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
                canonical_name: "label".into(),
                abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("String".into()),
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
        source_repos: vec![SourceAttribution {
            repo_url: "local".into(),
            file_path: "button.rs".into(),
            line_start: 1,
        }],
        provided_context: None,
        consumed_contexts: vec![],
    };

    let output = SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![comp],
        provenance: None,
        curation_log: None,
        stats: PipelineStats {
            files_scanned: 1,
            files_parsed: 1,
            components_found: 1,
            conflicts_detected: 0,
            llm_enriched: false,
        },
    };

    generate_dashboard(&output, &tmp.path().to_string_lossy()).unwrap();

    let html_path = tmp.path().join("index.html");
    assert!(html_path.exists(), "index.html should exist");

    let content = fs::read_to_string(&html_path).unwrap();
    assert!(content.contains("UCP Dashboard"));
    assert!(content.contains("Button"));
    assert!(content.contains("disabled"));
    assert!(content.contains("label"));
    assert!(content.contains("click"));
    assert!(content.contains("No conflicts"));
}
