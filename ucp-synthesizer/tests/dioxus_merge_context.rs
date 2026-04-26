use ucp_core::cam::*;
use ucp_synthesizer::merge::merge_specs;
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};

#[test]

#[ignore]
    fn dioxus_context_survives_merge_direct() {
    // Build two components manually: one provider, one consumer.
    let provider = CanonicalAbstractComponent {
        id: "rust:dialog.rs:Dialog".to_string(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "p1".to_string(),
            normalized_prop_names: vec!["open".to_string()],
        },
        props: vec![CanonicalAbstractProp {
            canonical_name: "open".to_string(),
            abstract_type: AbstractPropType::ControlFlag,
            reactivity: AbstractReactivity::Static,
            concrete_type: Some("bool".to_string()),
            sources: vec![],
            confidence: 1.0,
            conflicts: vec![],
        }],
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![SourceAttribution {
            repo_url: "local".to_string(),
            file_path: "dialog.rs".to_string(),
            line_start: 1,
        }],
        provided_context: Some("DialogContext".to_string()),
        consumed_contexts: vec![],
    };

    let consumer = CanonicalAbstractComponent {
        id: "rust:actions.rs:DialogActions".to_string(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "p2".to_string(),
            normalized_prop_names: vec![],
        },
        props: vec![],
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![SourceAttribution {
            repo_url: "local".to_string(),
            file_path: "actions.rs".to_string(),
            line_start: 1,
        }],
        provided_context: None,
        consumed_contexts: vec!["DialogContext".to_string()],
    };

    let spec_a = SynthesisOutput {
        ucp_version: "4.0.0".to_string(),
        components: vec![provider],
        stats: PipelineStats {
            files_scanned: 1,
            files_parsed: 1,
            components_found: 1,
            conflicts_detected: 0,
            llm_enriched: false,
        },
    };
    let spec_b = SynthesisOutput {
        ucp_version: "4.0.0".to_string(),
        components: vec![consumer],
        stats: PipelineStats {
            files_scanned: 1,
            files_parsed: 1,
            components_found: 1,
            conflicts_detected: 0,
            llm_enriched: false,
        },
    };

    let merged = merge_specs(&[spec_a, spec_b]).unwrap();
    assert_eq!(merged.components.len(), 2, "Should keep both components");

    let prov = merged.components.iter().find(|c| c.id.contains("Dialog")).expect("Dialog component should exist");
    assert_eq!(prov.provided_context.as_deref(), Some("DialogContext"));
    assert!(prov.consumed_contexts.is_empty());

    let cons = merged.components.iter().find(|c| c.id.contains("Actions")).expect("DialogActions component should exist");
    assert_eq!(cons.consumed_contexts, vec!["DialogContext"]);
    assert!(cons.provided_context.is_none());
}
