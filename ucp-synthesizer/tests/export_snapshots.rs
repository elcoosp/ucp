//! Snapshot tests for the W3C/DESIGN.md/llms.txt exporters.
use insta::assert_snapshot;
use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::pipeline::{PipelineStats, SynthesisOutput};

fn make_button_output() -> SynthesisOutput {
    SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![CanonicalAbstractComponent {
            id: "rust:button.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "abc".into(),
                normalized_prop_names: vec!["disabled".into()],
            },
            props: vec![CanonicalAbstractProp {
                canonical_name: "disabled".into(),
                abstract_type: AbstractPropType::ControlFlag,
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("bool".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            }],
            events: vec![CanonicalAbstractEvent {
                canonical_name: "click".into(),
                abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
            }],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![],
            provided_context: None,
            consumed_contexts: vec![],
        }],
        stats: PipelineStats {
            files_scanned: 1,
            files_parsed: 1,
            components_found: 1,
            conflicts_detected: 0,
            cache_hits: 0,
        },
        provenance: None,
        curation_log: None,
    }
}

#[test]
fn snapshot_design_md_export() {
    let tmp = TempDir::new().unwrap();
    ucp_synthesizer::export::design_md::export_design_md(
        &make_button_output(),
        None,
        "test-lib",
        "1.0.0",
        &tmp.path().to_string_lossy(),
    )
    .unwrap();
    let content = std::fs::read_to_string(tmp.path().join("DESIGN.md")).unwrap();
    assert_snapshot!("design_md", content);
}

#[test]
fn snapshot_llms_txt_export() {
    let tmp = TempDir::new().unwrap();
    ucp_synthesizer::export::llms_txt::export_llms_txt(
        &make_button_output(),
        &tmp.path().to_string_lossy(),
    )
    .unwrap();
    let content = std::fs::read_to_string(tmp.path().join("llms.txt")).unwrap();
    assert_snapshot!("llms_txt", content);
}
