use proptest::prelude::*;
use ucp_core::cam::*;
use ucp_synthesizer::generate::dioxus::DioxusGenerator;
use ucp_synthesizer::generate::traits::CodeGenerator;

fn make_component(name: String) -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: format!("rust:test.rs:{}", name),
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
        extracted_state_machine: None, extracted_parts: vec![], source_repos: vec![],
        provided_context: None, consumed_contexts: vec![],
    }
}

proptest! {
    #[test]
    fn generator_output_contains_component_name(name in r"[A-Z][a-zA-Z]{2,10}") {
        let comp = make_component(name.clone());
        let gen = DioxusGenerator;
        let output = gen.generate_component_code(&comp);
        prop_assert!(output.contains(&name));
        prop_assert!(output.contains("fn"));
        prop_assert!(output.contains("disabled"));
        prop_assert!(output.contains("label"));
    }

    #[test]
    fn generator_output_is_syntactically_reasonable(name in r"[A-Z][a-zA-Z]{2,10}") {
        let comp = make_component(name.clone());
        let gen = DioxusGenerator;
        let output = gen.generate_component_code(&comp);
        prop_assert!(output.contains("use dioxus::prelude::*"));
        prop_assert!(output.contains("#[component]"));
        prop_assert!(output.contains("Element"));
    }
}
