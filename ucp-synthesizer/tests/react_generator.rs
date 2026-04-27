use std::fs;
use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::generate::react::generate_react;

#[test]
fn generate_react_button_stub() {
    let tmp = TempDir::new().unwrap();
    let comp = CanonicalAbstractComponent {
        id: "tsx:button.tsx:Button".to_string(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "abc".into(),
            normalized_prop_names: vec![
                "disabled".into(),
                "label".into(),
                "attributes".into(),
                "children".into(),
            ],
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
        name: "test-react-lib".into(),
        version: "0.1.0".into(),
        frameworks: vec!["react".into()],
        components: vec![comp],
        global_styles: None,
        generated_by: "test".into(),
        generated_at: "now".into(),
    };
    generate_react(&manifest, &tmp.path().to_string_lossy()).unwrap();
    let file = tmp.path().join("src/button.tsx");
    assert!(file.exists(), "Expected generated file");
    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("interface ButtonProps"));
    assert!(content.contains("export function Button"));
    assert!(content.contains("disabled?: boolean"));
    assert!(content.contains("label?: string"));
    assert!(content.contains("...rest"));
    assert!(content.contains("React.ReactNode"));
}

#[test]
fn snapshot_react_button() {
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
        frameworks: vec!["react".into()],
        components: vec![comp],
        global_styles: None, generated_by: "test".into(), generated_at: "now".into(),
    };
    ucp_synthesizer::generate::react::generate_react(&manifest, &tmp.path().to_string_lossy()).unwrap();
    let content = std::fs::read_to_string(tmp.path().join("src/button.tsx")).unwrap();
    assert_snapshot!("react_button", content);
}
