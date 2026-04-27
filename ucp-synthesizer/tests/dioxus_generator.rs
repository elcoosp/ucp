use std::fs;
use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::generate::dioxus::generate_dioxus;

#[test]
fn generate_dioxus_from_package_manifest() {
    let tmp = TempDir::new().unwrap();
    let output = tmp.path();

    let comp = CanonicalAbstractComponent {
        id: "rust:button.rs:Button".to_string(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "abc".to_string(),
            normalized_prop_names: vec!["disabled".to_string(), "label".to_string()],
        },
        props: vec![
            CanonicalAbstractProp {
                canonical_name: "disabled".to_string(),
                abstract_type: AbstractPropType::ControlFlag,
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("bool".to_string()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            },
            CanonicalAbstractProp {
                canonical_name: "label".to_string(),
                abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("String".to_string()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            },
        ],
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    };

    let manifest = PackageManifest {
        name: "test-lib".to_string(),
        version: "0.1.0".to_string(),
        frameworks: vec!["dioxus".to_string()],
        components: vec![comp],
        global_styles: None,
        generated_by: "ucp-test".to_string(),
        generated_at: "now".to_string(),
    };

    generate_dioxus(&manifest, &output.to_string_lossy()).unwrap();

    let generated_file = output.join("src").join("button.rs");
    assert!(generated_file.exists(), "Button source should be generated");

    let content = fs::read_to_string(&generated_file).unwrap();
    assert!(
        content.contains("pub struct ButtonProps"),
        "Should define ButtonProps"
    );
    assert!(
        content.contains("fn Button("),
        "Should define Button component"
    );
    assert!(
        content.contains("disabled: bool"),
        "Should include disabled prop"
    );
    assert!(
        content.contains("label: String"),
        "Should include label prop"
    );
}

#[test]
fn snapshot_dioxus_button() {
    use insta::assert_snapshot;
    let tmp = tempfile::TempDir::new().unwrap();
    let comp = ucp_core::cam::CanonicalAbstractComponent {
        id: "rust:button.rs:Button".into(),
        semantic_fingerprint: ucp_core::cam::SemanticFingerprint {
            purpose_hash: "abc".into(),
            normalized_prop_names: vec!["disabled".into(), "label".into()],
        },
        props: vec![
            ucp_core::cam::CanonicalAbstractProp {
                canonical_name: "disabled".into(),
                abstract_type: ucp_core::cam::AbstractPropType::ControlFlag,
                reactivity: ucp_core::cam::AbstractReactivity::Static,
                concrete_type: Some("bool".into()),
                sources: vec![], confidence: 1.0, conflicts: vec![],
            },
            ucp_core::cam::CanonicalAbstractProp {
                canonical_name: "label".into(),
                abstract_type: ucp_core::cam::AbstractPropType::StaticValue(Box::new(ucp_core::cam::AbstractPropType::Any)),
                reactivity: ucp_core::cam::AbstractReactivity::Static,
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
    };
    let manifest = ucp_core::cam::PackageManifest {
        name: "test-lib".into(), version: "0.1.0".into(),
        frameworks: vec!["dioxus".into()],
        components: vec![comp],
        global_styles: None, generated_by: "test".into(), generated_at: "now".into(),
    };
    ucp_synthesizer::generate::dioxus::generate_dioxus(&manifest, &tmp.path().to_string_lossy()).unwrap();
    let content = std::fs::read_to_string(tmp.path().join("src/button.rs")).unwrap();
    assert_snapshot!("dioxus_button", content);
}
