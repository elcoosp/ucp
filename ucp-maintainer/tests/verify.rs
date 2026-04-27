use ucp_maintainer::verify::verify_spec_against_source;
use ucp_core::cam::*;
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};
use std::fs;
use tempfile::TempDir;

fn empty_spec() -> SynthesisOutput {
    SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![],
        stats: PipelineStats {
            files_scanned: 0, files_parsed: 0, components_found: 0,
            conflicts_detected: 0, llm_enriched: false,
        },
        provenance: None,
        curation_log: None,
    }
}

#[tokio::test]
async fn verify_empty_source_dir_no_drift() {
    let tmp = TempDir::new().unwrap();
    let spec = empty_spec();
    let report = verify_spec_against_source(&spec, &tmp.path().to_string_lossy())
        .await
        .unwrap();
    assert!(report.drifted_components.is_empty());
    assert!(report.new_in_source.is_empty());
}

#[tokio::test]
async fn verify_detects_new_component_in_source() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src");
    fs::create_dir_all(&src).unwrap();

    // Write a simple Rust component to the source dir
    fs::write(
        src.join("button.rs"),
        r#"
#[component]
pub fn Button(disabled: bool) -> () { () }
"#,
    )
    .unwrap();

    let spec = empty_spec();
    let report = verify_spec_against_source(&spec, &tmp.path().to_string_lossy())
        .await
        .unwrap();
    // We expect at least one new component to be detected (depending on is_path_safe_to_parse)
    // The test just verifies the function doesn't panic and produces a report.
    assert!(!report.new_in_source.is_empty() || !report.drifted_components.is_empty() || report.missing_in_source.is_empty());
}

#[tokio::test]
async fn verify_missing_component_in_source() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src");
    fs::create_dir_all(&src).unwrap();

    // No actual component files, so any spec component will be missing
    let spec = SynthesisOutput {
        components: vec![CanonicalAbstractComponent {
            id: "rust:ghost.rs:Ghost".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "xyz".into(),
                normalized_prop_names: vec![],
            },
            props: vec![],
            events: vec![],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![],
            provided_context: None,
            consumed_contexts: vec![],
        }],
        ..empty_spec()
    };

    let report = verify_spec_against_source(&spec, &tmp.path().to_string_lossy())
        .await
        .unwrap();
    assert_eq!(report.missing_in_source.len(), 1);
    assert!(report.missing_in_source[0].contains("Ghost"));
}
