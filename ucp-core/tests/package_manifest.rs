use ucp_core::cam::*;
use ucp_core::cam::{CanonicalAbstractComponent, CanonicalAbstractProp, AbstractPropType, AbstractReactivity, PackageManifest};

#[test]
fn serialize_package_manifest() {
    let comp = CanonicalAbstractComponent {
        id: "button".to_string(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "abc".to_string(),
            normalized_prop_names: vec!["disabled".to_string()],
        },
        props: vec![CanonicalAbstractProp {
            canonical_name: "disabled".to_string(),
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
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    };

    let manifest = PackageManifest {
        name: "shadcn-dioxus".to_string(),
        version: "0.1.0".to_string(),
        frameworks: vec!["dioxus".to_string()],
        components: vec![comp],
        global_styles: None,
        generated_by: "ucp 4.0.0".to_string(),
        generated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let json = serde_json::to_string_pretty(&manifest).unwrap();
    assert!(json.contains("shadcn-dioxus"));
    assert!(json.contains("dioxus"));
    assert!(json.contains("button"));
}
