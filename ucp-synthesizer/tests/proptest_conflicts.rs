use proptest::prelude::*;
use ucp_core::cam::*;
use ucp_synthesizer::merge::{merge_specs, MergeOptions};
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};

fn make_spec(id: &str, prop_type: AbstractPropType) -> SynthesisOutput {
    SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![CanonicalAbstractComponent {
            id: format!("rust:test.rs:{}", id),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: format!("{:016x}", id.len()),
                normalized_prop_names: vec!["test_prop".into()],
            },
            props: vec![CanonicalAbstractProp {
                canonical_name: "test_prop".into(),
                abstract_type: prop_type,
                reactivity: AbstractReactivity::Static,
                concrete_type: None,
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            }],
            events: vec![],
            extracted_state_machine: None, extracted_parts: vec![], source_repos: vec![],
            provided_context: None, consumed_contexts: vec![],
        }],
        stats: PipelineStats {
            files_scanned: 1, files_parsed: 1, components_found: 1,
            conflicts_detected: 0, llm_enriched: false,
        },
        provenance: None,
        curation_log: None,
    }
}

proptest! {
    #[test]
    fn identical_specs_produce_no_conflicts(name in r"[A-Z][a-zA-Z]{2,10}") {
        let spec = make_spec(&name, AbstractPropType::ControlFlag);
        let merged = merge_specs(&[spec.clone(), spec], MergeOptions::default()).unwrap();
        prop_assert_eq!(merged.stats.conflicts_detected, 0);
    }

    #[test]
    fn type_mismatch_produces_conflicts(name in r"[A-Z][a-zA-Z]{2,10}") {
        let spec_a = make_spec(&name, AbstractPropType::ControlFlag);
        let spec_b = make_spec(&name, AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)));
        let merged = merge_specs(&[spec_a, spec_b], MergeOptions::default()).unwrap();
        prop_assert!(merged.stats.conflicts_detected >= 1);
    }
}
