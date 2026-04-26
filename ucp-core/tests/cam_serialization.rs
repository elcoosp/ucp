use ucp_core::cam::*;

#[test]
fn serialize_canonical_abstract_prop_controlled_value() {
    let prop = CanonicalAbstractProp {
        canonical_name: "value".to_string(),
        abstract_type: AbstractPropType::ControlledValue(Box::new(AbstractPropType::StaticValue(
            Box::new(AbstractPropType::Any),
        ))),
        reactivity: AbstractReactivity::Controlled,
        sources: vec![],
        confidence: 0.95,
        conflicts: vec![],
    };
    let json = serde_json::to_string(&prop).unwrap();
    assert!(json.contains("\"controlledValue\""));
}

#[test]
fn serialize_conflict_with_profile_scoping() {
    let conflict = Conflict {
        id: "conf_001".to_string(),
        field: "props.loading".to_string(),
        present_in: vec!["react".to_string()],
        absent_in: vec!["gpui".to_string()],
        confidence: 0.5,
        resolution_suggestion: ResolutionStrategy::ScopeToProfile("web".to_string()),
    };
    let json = serde_json::to_string(&conflict).unwrap();
    assert!(json.contains("\"scopeToProfile\":\"web\""));
}

#[test]
fn serialize_spread_attributes_variant() {
    let prop = CanonicalAbstractProp {
        canonical_name: "attributes".to_string(),
        abstract_type: AbstractPropType::SpreadAttributes,
        reactivity: AbstractReactivity::Static,
        sources: vec![],
        confidence: 1.0,
        conflicts: vec![],
    };
    let json = serde_json::to_string(&prop).unwrap();
    assert!(json.contains("\"spreadAttributes\""));
}
