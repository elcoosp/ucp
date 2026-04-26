use ucp_core::cam::*;
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};

pub fn make_minimal_component(name: &str, framework: &str) -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: format!("{}:test.rs:{}", framework, name),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "test".into(),
            normalized_prop_names: vec!["disabled".into(), "label".into()],
        },
        props: vec![
            CanonicalAbstractProp {
                canonical_name: "disabled".into(),
                abstract_type: AbstractPropType::ControlFlag,
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("bool".into()),
                sources: vec![], confidence: 1.0, conflicts: vec![],
            },
            CanonicalAbstractProp {
                canonical_name: "label".into(),
                abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("String".into()),
                sources: vec![], confidence: 1.0, conflicts: vec![],
            },
        ],
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    }
}

pub fn make_empty_spec() -> SynthesisOutput {
    SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![],
        stats: PipelineStats {
            files_scanned: 0, files_parsed: 0, components_found: 0,
            conflicts_detected: 0, llm_enriched: false,
        },
    }
}

pub fn make_package_manifest(name: &str, comps: Vec<CanonicalAbstractComponent>) -> PackageManifest {
    PackageManifest {
        name: name.into(),
        version: "0.1.0".into(),
        frameworks: vec!["dioxus".into()],
        components: comps,
        global_styles: None,
        generated_by: "test".into(),
        generated_at: "now".into(),
    }
}
