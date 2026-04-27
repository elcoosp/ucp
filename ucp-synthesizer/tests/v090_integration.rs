use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::contract::mcp_server::generate_server_json;
use ucp_synthesizer::export::design_md::export_design_md;
use ucp_synthesizer::export::llms_txt::export_llms_txt;
use ucp_synthesizer::pipeline::{PipelineStats, SynthesisOutput};

fn make_button_component() -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
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
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    }
}

#[test]
fn v090_llms_txt_export() {
    let tmp = TempDir::new().unwrap();
    let output = SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![make_button_component()],
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
    export_llms_txt(&output, &tmp.path().to_string_lossy()).unwrap();
    assert!(tmp.path().join("llms.txt").exists());
}

#[test]
fn v090_design_md_v1_export() {
    let tmp = TempDir::new().unwrap();
    let output = SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![make_button_component()],
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
    export_design_md(
        &output,
        None,
        "test-lib",
        "1.0.0",
        &tmp.path().to_string_lossy(),
    )
    .unwrap();
    let content = std::fs::read_to_string(tmp.path().join("DESIGN.md")).unwrap();
    assert!(content.contains("title: test-lib"));
}

#[test]
fn v090_mcp_server_json() {
    let tmp = TempDir::new().unwrap();
    generate_server_json("test-server", "Test MCP", &tmp.path().to_string_lossy()).unwrap();
    let content = std::fs::read_to_string(tmp.path().join("server.json")).unwrap();
    assert!(content.contains("test-server"));
    assert!(content.contains("tools"));
}
