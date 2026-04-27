use ucp_maintainer::diff::{diff_specs, diff_specs_text};
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};

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

#[test]
fn diff_integration_no_differences() {
    let spec = empty_spec();
    let report = diff_specs(&spec, &spec).unwrap();
    assert!(report.added_components.is_empty());
    assert!(report.removed_components.is_empty());
    assert!(report.changed_components.is_empty());
}

#[test]
fn diff_integration_text_output() {
    let spec = empty_spec();
    let text = diff_specs_text(&spec, &spec).unwrap();
    assert!(!text.is_empty());
}
