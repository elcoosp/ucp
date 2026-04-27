use ucp_maintainer::curate::{curate_spec, count_unresolved_conflicts};
use ucp_core::cam::*;
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};

fn make_conflict_spec() -> SynthesisOutput {
    let conflict_id = "conf_001".to_string();
    SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![CanonicalAbstractComponent {
            id: "rust:test.rs:Button".into(),
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
                conflicts: vec![Conflict {
                    id: conflict_id.clone(),
                    field: "props.disabled".into(),
                    present_in: vec!["src/a.rs".into()],
                    absent_in: vec!["src/b.rs".into()],
                    confidence: 0.8,
                    resolution_suggestion: ResolutionStrategy::IncludeMajority,
                }],
            }],
            events: vec![],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![],
            provided_context: None,
            consumed_contexts: vec![],
        }],
        stats: PipelineStats {
            files_scanned: 0, files_parsed: 0, components_found: 0,
            conflicts_detected: 0, llm_enriched: false,
        },
        provenance: None,
        curation_log: None,
    }
}

#[test]
fn integration_curation_resolves_all_conflicts() {
    let spec = make_conflict_spec();
    let resolutions = vec![ucp_maintainer::curate::Resolution {
        conflict_id: "conf_001".into(),
        chosen_resolution: ResolutionStrategy::IncludeMajority,
        custom_rationale: None,
    }];
    let curated = curate_spec(&spec, &resolutions).unwrap();
    assert_eq!(count_unresolved_conflicts(&curated), 0);
    assert!(curated.curation_log.is_some());
    assert_eq!(curated.curation_log.unwrap().len(), 1);
}

#[test]
fn integration_unresolved_conflicts_remain() {
    let spec = make_conflict_spec();
    let curated = curate_spec(&spec, &[]).unwrap();
    assert_eq!(count_unresolved_conflicts(&curated), 1);
    assert!(curated.curation_log.is_none());
}
