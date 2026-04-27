use insta::assert_json_snapshot;
use ucp_core::cam::*;

fn make_full_component() -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: "rust:button.rs:Button".into(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "abc".into(),
            normalized_prop_names: vec!["disabled".into(), "label".into(), "variant".into()],
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
                canonical_name: "variant".into(),
                abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("enum: Default, Destructive".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            },
        ],
        events: vec![CanonicalAbstractEvent {
            canonical_name: "click".to_string(),
            abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
        }],
        extracted_state_machine: Some(StateMachine {
            id: "sm".into(),
            initial: "idle".into(),
            states: [("idle".into(), StateNode { on: None })].into(),
        }),
        extracted_parts: vec![ExtractedPart { name: "trigger".into(), selectable: true }],
        source_repos: vec![SourceAttribution {
            repo_url: "local".into(),
            file_path: "button.rs".into(),
            line_start: 1,
        }],
        provided_context: None,
        consumed_contexts: vec![],
    }
}

#[test]
fn snapshot_canonical_abstract_component() {
    assert_json_snapshot!(make_full_component());
}

#[test]
fn snapshot_canonical_abstract_prop() {
    let prop = CanonicalAbstractProp {
        canonical_name: "disabled".to_string(),
        abstract_type: AbstractPropType::ControlFlag,
        reactivity: AbstractReactivity::Static,
        concrete_type: Some("bool".to_string()),
        sources: vec![],
        confidence: 1.0,
        conflicts: vec![],
    };
    assert_json_snapshot!(prop);
}

#[test]
fn snapshot_spread_attributes_variant() {
    let prop = CanonicalAbstractProp {
        canonical_name: "attributes".to_string(),
        abstract_type: AbstractPropType::SpreadAttributes,
        reactivity: AbstractReactivity::Static,
        concrete_type: None,
        sources: vec![],
        confidence: 1.0,
        conflicts: vec![],
    };
    assert_json_snapshot!(prop);
}
