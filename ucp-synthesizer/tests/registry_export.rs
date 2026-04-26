use std::fs;
use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::generate::registry::generate_registry;

fn make_test_component(name: &str) -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: format!("rust:{}.rs:{}", name.to_lowercase(), name),
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
    }
}

#[test]
fn registry_index_is_object_not_array() {
    let tmp = TempDir::new().unwrap();
    let comp = make_test_component("Button");
    let manifest = PackageManifest {
        name: "test-registry".into(),
        version: "0.1.0".into(),
        frameworks: vec!["dioxus".into()],
        components: vec![comp],
        global_styles: None,
        generated_by: "test".into(),
        generated_at: "now".into(),
    };
    generate_registry(&manifest, &tmp.path().to_string_lossy(), None, None, None).unwrap();

    let content = fs::read_to_string(tmp.path().join("registry.json")).unwrap();
    assert!(content.contains("\"$schema\""));
    assert!(content.contains("https://ui.shadcn.com/schema/registry.json"));
    assert!(content.contains("\"items\""));
    assert!(!content.starts_with('[')); // should be an object, not array
}

#[test]
fn registry_item_has_full_schema_fields() {
    let tmp = TempDir::new().unwrap();
    let comp = make_test_component("Button");
    let manifest = PackageManifest {
        name: "test-registry".into(),
        version: "0.1.0".into(),
        frameworks: vec!["dioxus".into()],
        components: vec![comp],
        global_styles: None,
        generated_by: "test".into(),
        generated_at: "now".into(),
    };
    generate_registry(
        &manifest,
        &tmp.path().to_string_lossy(),
        Some("acme"),
        Some("UCP Team <team@ucp.dev>"),
        Some("https://ucp.dev"),
    )
    .unwrap();

    let content = fs::read_to_string(tmp.path().join("registry-item-button.json")).unwrap();
    assert!(content.contains("\"$schema\""));
    assert!(content.contains("registry-item.json"));
    assert!(content.contains("\"title\""));
    assert!(content.contains("\"Button\"")); // humanised
    assert!(content.contains("\"dependencies\""));
    assert!(content.contains("\"meta\""));
}

#[test]
fn namespaced_dependency_format() {
    let tmp = TempDir::new().unwrap();
    let button = make_test_component("Button");
    let dialog = {
        let mut d = make_test_component("Dialog");
        d.props.push(CanonicalAbstractProp {
            canonical_name: "trigger".into(),
            abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            reactivity: AbstractReactivity::Static,
            concrete_type: Some("Button".into()),
            sources: vec![],
            confidence: 1.0,
            conflicts: vec![],
        });
        d
    };
    let manifest = PackageManifest {
        name: "test-registry".into(),
        version: "0.1.0".into(),
        frameworks: vec!["dioxus".into()],
        components: vec![button, dialog],
        global_styles: None,
        generated_by: "test".into(),
        generated_at: "now".into(),
    };
    generate_registry(
        &manifest,
        &tmp.path().to_string_lossy(),
        Some("acme"),
        None,
        None,
    )
    .unwrap();

    let content = fs::read_to_string(tmp.path().join("registry-item-dialog.json")).unwrap();
    assert!(
        content.contains("@acme/button"),
        "Should contain namespaced dependency"
    );
}
