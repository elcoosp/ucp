use insta::{assert_json_snapshot, assert_snapshot};
use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};

fn make_button_output() -> SynthesisOutput {
    SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![CanonicalAbstractComponent {
            id: "rust:button.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint { purpose_hash: "abc".into(), normalized_prop_names: vec!["disabled".into()] },
            props: vec![CanonicalAbstractProp {
                canonical_name: "disabled".into(), abstract_type: AbstractPropType::ControlFlag,
                reactivity: AbstractReactivity::Static, concrete_type: Some("bool".into()),
                sources: vec![], confidence: 1.0, conflicts: vec![],
            }],
            events: vec![CanonicalAbstractEvent { canonical_name: "click".into(), abstract_payload: AbstractPropType::AsyncEventHandler(vec![]) }],
            extracted_state_machine: None, extracted_parts: vec![], source_repos: vec![],
            provided_context: None, consumed_contexts: vec![],
        }],
        stats: PipelineStats { files_scanned: 1, files_parsed: 1, components_found: 1, conflicts_detected: 0, llm_enriched: false },
        provenance: None,
        curation_log: None,
    }
}

#[test]
fn snapshot_a2ui_export() {
    let tmp = TempDir::new().unwrap();
    ucp_synthesizer::export::a2ui::export_a2ui(&make_button_output(), "test-lib", "1.0.0", &tmp.path().to_string_lossy()).unwrap();
    let content = std::fs::read_to_string(tmp.path().join("a2ui-catalog.json")).unwrap();
    assert_json_snapshot!("a2ui_catalog", &serde_json::from_str::<serde_json::Value>(&content).unwrap());
}

#[test]
fn snapshot_ag_ui_export() {
    let tmp = TempDir::new().unwrap();
    ucp_synthesizer::export::ag_ui::export_ag_ui(&make_button_output(), &tmp.path().to_string_lossy()).unwrap();
    let content = std::fs::read_to_string(tmp.path().join("ag-ui-events.json")).unwrap();
    assert_json_snapshot!("ag_ui_events", &serde_json::from_str::<serde_json::Value>(&content).unwrap());
}

#[test]
fn snapshot_design_md_export() {
    let tmp = TempDir::new().unwrap();
    ucp_synthesizer::export::design_md::export_design_md(&make_button_output(), None, "test-lib", "1.0.0", &tmp.path().to_string_lossy()).unwrap();
    let content = std::fs::read_to_string(tmp.path().join("DESIGN.md")).unwrap();
    assert_snapshot!("design_md", content);
}

#[test]
fn snapshot_llms_txt_export() {
    let tmp = TempDir::new().unwrap();
    ucp_synthesizer::export::llms_txt::export_llms_txt(&make_button_output(), &tmp.path().to_string_lossy()).unwrap();
    let content = std::fs::read_to_string(tmp.path().join("llms.txt")).unwrap();
    assert_snapshot!("llms_txt", content);
}

#[test]
fn snapshot_ai_contract() {
    let tmp = TempDir::new().unwrap();
    ucp_synthesizer::contract::ai_contract::export_ai_contract(&make_button_output(), &tmp.path().join("contract.json").to_string_lossy()).unwrap();
    let mut content = std::fs::read_to_string(tmp.path().join("contract.json")).unwrap();
    // Replace variable timestamp for deterministic snapshots
    if let Some(start) = content.find("\"generatedAt\": \"") {
        let val_start = start + "\"generatedAt\": \"".len();
        if let Some(end) = content[val_start..].find('"') {
            content.replace_range(val_start..val_start + end, "FIXED_TIMESTAMP");
        }
    }
    assert_json_snapshot!("ai_contract", &serde_json::from_str::<serde_json::Value>(&content).unwrap());
}
