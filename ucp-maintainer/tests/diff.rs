//! Integration tests for the spec diff engine.
use ucp_maintainer::diff::{diff_specs, diff_specs_text};
use ucp_synthesizer::pipeline::{PipelineStats, SynthesisOutput};

fn empty_spec() -> SynthesisOutput {
    SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![],
        stats: PipelineStats {
            files_scanned: 0,
            files_parsed: 0,
            components_found: 0,
            conflicts_detected: 0,
            cache_hits: 0,
        },
        provenance: None,
        curation_log: None,
    }
}

#[test]
fn diff_integration_no_differences() {
    let spec = empty_spec();
    let report = diff_specs(&spec, &spec).unwrap();
    assert_eq!(report.added_components, [] as [std::string::String; 0]);
    assert_eq!(report.removed_components, [] as [std::string::String; 0]);
    assert!(report.changed_components.is_empty());
}

#[test]
fn diff_integration_text_output() {
    let spec = empty_spec();
    let text = diff_specs_text(&spec, &spec).unwrap();
    assert_ne!(text, "");
}
