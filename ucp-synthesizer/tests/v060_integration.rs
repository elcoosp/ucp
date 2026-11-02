//! Backward-compatibility tests against v0.6.0 spec fixtures.
use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::generate::dioxus::generate_dioxus;
use ucp_synthesizer::generate::gpui::generate_gpui;
use ucp_synthesizer::generate::leptos::generate_leptos;
use ucp_synthesizer::generate::react::generate_react;

fn make_button_component(framework: &str) -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: format!(
            "{}:button.{}:Button",
            framework,
            if framework == "tsx" { "tsx" } else { "rs" }
        ),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "abc".into(),
            normalized_prop_names: vec!["disabled".into(), "label".into()],
        },
        props: vec![
            CanonicalAbstractProp {
                canonical_name: "disabled".into(),
                abstract_type: AbstractPropType::ControlFlag,
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("bool".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            },
            CanonicalAbstractProp {
                canonical_name: "label".into(),
                abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("String".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            },
        ],
        events: vec![CanonicalAbstractEvent {
            canonical_name: "click".to_string(),
            abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
        }],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    }
}

#[test]
fn full_v060_pipeline_all_generators() {
    let tmp = TempDir::new().unwrap();
    let dioxus_comp = make_button_component("rust");
    let manifest = PackageManifest {
        name: "test-v060".into(),
        version: "0.1.0".into(),
        frameworks: vec![
            "dioxus".into(),
            "react".into(),
            "leptos".into(),
            "gpui".into(),
        ],
        components: vec![dioxus_comp],
        global_styles: None,
        generated_by: "test".into(),
        generated_at: "now".into(),
    };

    // All four generators should succeed
    generate_dioxus(&manifest, &tmp.path().join("dioxus").to_string_lossy()).unwrap();
    assert!(tmp.path().join("dioxus/src/button.rs").exists());

    generate_react(&manifest, &tmp.path().join("react").to_string_lossy()).unwrap();
    assert!(tmp.path().join("react/src/button.tsx").exists());

    generate_leptos(&manifest, &tmp.path().join("leptos").to_string_lossy()).unwrap();
    assert!(tmp.path().join("leptos/src/button.rs").exists());

    generate_gpui(&manifest, &tmp.path().join("gpui").to_string_lossy()).unwrap();
    assert!(tmp.path().join("gpui/src/button.rs").exists());
}
