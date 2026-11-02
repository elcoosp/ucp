//! Property-based tests for the raw-type unification ontology.
use proptest::prelude::*;
use ucp_synthesizer::unify::map_raw_type_with_concrete;

proptest! {
    #[test]
    fn type_unification_is_consistent(s in r"[a-zA-Z<>]+") {
        if let Ok((t, _)) = map_raw_type_with_concrete(&s) {
            // The abstract type should always be one of the known variants
            let type_str = format!("{t:?}");
            prop_assert!(type_str.contains("ControlFlag")
                || type_str.contains("StaticValue")
                || type_str.contains("ControlledValue")
                || type_str.contains("UncontrolledValue")
                || type_str.contains("AsyncEventHandler")
                || type_str.contains("Renderable")
                || type_str.contains("SpreadAttributes")
                || type_str.contains("Any"));
        }
    }

    #[test]
    fn concrete_types_are_non_empty(s in r"[a-zA-Z<>]+") {
        if let Ok((_, Some(c))) = map_raw_type_with_concrete(&s) {
            prop_assert!(!c.is_empty());
        }
    }
}
