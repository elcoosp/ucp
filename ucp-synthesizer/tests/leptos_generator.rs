use std::fs;
use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::generate::leptos::generate_leptos;

#[test]
fn generate_leptos_button_stub() {
    let tmp = TempDir::new().unwrap();
    let comp = CanonicalAbstractComponent {
        id: "rust:button.rs:Button".to_string(),
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
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    };
    let manifest = PackageManifest {
        name: "test-leptos".into(),
        version: "0.1.0".into(),
        frameworks: vec!["leptos".into()],
        components: vec![comp],
        global_styles: None,
        generated_by: "test".into(),
        generated_at: "now".into(),
    };
    generate_leptos(&manifest, &tmp.path().to_string_lossy()).unwrap();
    let file = tmp.path().join("src").join("button.rs");
    assert!(file.exists(), "Expected generated file to exist");
    let content = fs::read_to_string(&file).unwrap();
    assert!(
        content.contains("pub fn Button"),
        "Should contain component function"
    );
    assert!(content.contains("view! {"), "Should contain view! macro");
    assert!(content.contains("disabled"), "Should contain disabled prop");
    assert!(content.contains("label"), "Should contain label prop");
}

#[test]
fn generate_leptos_with_spread_attributes() {
    let tmp = TempDir::new().unwrap();
    let comp = CanonicalAbstractComponent {
        id: "rust:card.rs:Card".to_string(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "def".into(),
            normalized_prop_names: vec!["attributes".into()],
        },
        props: vec![
            CanonicalAbstractProp {
                canonical_name: "attributes".into(),
                abstract_type: AbstractPropType::SpreadAttributes,
                reactivity: AbstractReactivity::Static,
                concrete_type: None,
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            },
            CanonicalAbstractProp {
                canonical_name: "children".into(),
                abstract_type: AbstractPropType::Renderable,
                reactivity: AbstractReactivity::Static,
                concrete_type: None,
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
        name: "test-leptos2".into(),
        version: "0.1.0".into(),
        frameworks: vec!["leptos".into()],
        components: vec![comp],
        global_styles: None,
        generated_by: "test".into(),
        generated_at: "now".into(),
    };
    generate_leptos(&manifest, &tmp.path().to_string_lossy()).unwrap();
    let file = tmp.path().join("src").join("card.rs");
    let content = fs::read_to_string(&file).unwrap();
    assert!(
        content.contains("#[prop(attrs)]"),
        "Should have attrs on spread"
    );
    assert!(
        content.contains("Children"),
        "Should use Children for renderable"
    );
}

#[test]
fn snapshot_leptos_button() {
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
        frameworks: vec!["leptos".into()],
        components: vec![comp],
        global_styles: None, generated_by: "test".into(), generated_at: "now".into(),
    };
    ucp_synthesizer::generate::leptos::generate_leptos(&manifest, &tmp.path().to_string_lossy()).unwrap();
    let content = std::fs::read_to_string(tmp.path().join("src/button.rs")).unwrap();
    assert_snapshot!("leptos_button", content);
}
