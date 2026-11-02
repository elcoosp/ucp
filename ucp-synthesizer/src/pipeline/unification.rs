//! Unification of raw framework extractions into CAM components.
//!
//! All framework-specific extractors converge on
//! [`build_component`]; only the prop-conversion step differs
//! between Rust-style (struct fields / fn parameters) and TSX-style
//! (interface members) raw extractions.

use super::output::{ANY_PENALTY_PER_PROP, BASE_CONFIDENCE_RUST, BASE_CONFIDENCE_TSX};
use crate::extract::rust_ast;
use crate::extract::tsx_ast;
use crate::unify::map_raw_type_with_concrete;
use ucp_core::cam::{
    AbstractPropType, AbstractReactivity, CanonicalAbstractComponent, CanonicalAbstractEvent,
    CanonicalAbstractProp, ExtractedPart, PropSourceMapping, SemanticFingerprint,
    SourceAttribution,
};
use ucp_core::Result;

/// Unify a Leptos `#[component]` function extraction into CAM.
///
/// # Errors
///
/// Propagates errors from raw type mapping (in practice type mapping is
/// total and never fails).
pub fn unify_rust_component(
    raw: &rust_ast::RawComponentExtraction,
    file_path: &str,
) -> Result<CanonicalAbstractComponent> {
    let props = raw
        .props
        .iter()
        .map(|rp| rust_prop_to_cam(rp, file_path))
        .collect();
    Ok(build_component(
        ComponentShape {
            id_prefix: "rust",
            base_confidence: BASE_CONFIDENCE_RUST,
            provided_context: raw.provided_context.clone(),
            consumed_contexts: raw.consumed_contexts.clone(),
        },
        &raw.name,
        raw.line_start,
        props,
        file_path,
    ))
}

/// Unify a Leptos struct-props (`ButtonProps` + `impl Button`) extraction.
///
/// # Errors
///
/// See [`unify_rust_component`].
pub fn unify_rust_component_struct(
    raw: &rust_ast::RawComponentExtraction,
    file_path: &str,
) -> Result<CanonicalAbstractComponent> {
    let props = raw
        .props
        .iter()
        .map(|rp| rust_prop_to_cam(rp, file_path))
        .collect();
    Ok(build_component(
        ComponentShape {
            id_prefix: "rust-struct",
            base_confidence: BASE_CONFIDENCE_RUST,
            provided_context: raw.provided_context.clone(),
            consumed_contexts: raw.consumed_contexts.clone(),
        },
        &raw.name,
        raw.line_start,
        props,
        file_path,
    ))
}

/// Unify a TSX component extraction into CAM.
///
/// # Errors
///
/// See [`unify_rust_component`].
pub fn unify_tsx_component(
    raw: &tsx_ast::RawTsxExtraction,
    file_path: &str,
) -> Result<CanonicalAbstractComponent> {
    let props = raw
        .props
        .iter()
        .map(|rp| tsx_prop_to_cam(rp, file_path))
        .collect();
    Ok(build_component(
        ComponentShape {
            id_prefix: "tsx",
            base_confidence: BASE_CONFIDENCE_TSX,
            provided_context: raw.provided_context.clone(),
            consumed_contexts: raw.consumed_contexts.clone(),
        },
        &raw.name,
        raw.line_start,
        props,
        file_path,
    ))
}

/// The per-kind facts that differ between raw extraction flavors.
struct ComponentShape {
    id_prefix: &'static str,
    base_confidence: f32,
    provided_context: Option<String>,
    consumed_contexts: Vec<String>,
}

/// Convert one Rust-style raw prop to CAM, honoring the spread-attributes
/// and explicit event flags produced by the Rust extractors.
fn rust_prop_to_cam(rp: &rust_ast::RawPropExtraction, file_path: &str) -> CanonicalAbstractProp {
    let (cam_type, concrete_type) = if rp.is_spread_attributes {
        (
            AbstractPropType::SpreadAttributes,
            Some("Attributes".to_string()),
        )
    } else {
        map_raw_type_with_concrete(&rp.raw_type).unwrap_or((AbstractPropType::Any, None))
    };
    let cam_type = if rp.is_event {
        AbstractPropType::AsyncEventHandler(vec![])
    } else {
        cam_type
    };
    make_prop(
        rp.name.clone(),
        cam_type,
        concrete_type,
        &rp.raw_type,
        file_path,
        rp.has_default,
    )
}

/// Convert one TSX-style raw prop to CAM. TSX has no explicit event flag:
/// arrow-function and `void`-returning types are treated as handlers.
fn tsx_prop_to_cam(rp: &tsx_ast::RawTsxPropExtraction, file_path: &str) -> CanonicalAbstractProp {
    let (cam_type, concrete_type) = if rp.raw_type.contains("=>") || rp.raw_type.contains("void") {
        (
            AbstractPropType::AsyncEventHandler(vec![]),
            Some(rp.raw_type.clone()),
        )
    } else {
        map_raw_type_with_concrete(&rp.raw_type).unwrap_or((AbstractPropType::Any, None))
    };
    make_prop(
        rp.name.clone(),
        cam_type,
        concrete_type,
        &rp.raw_type,
        file_path,
        rp.is_optional,
    )
}

fn make_prop(
    name: String,
    abstract_type: AbstractPropType,
    concrete_type: Option<String>,
    raw_type: &str,
    file_path: &str,
    has_default: bool,
) -> CanonicalAbstractProp {
    CanonicalAbstractProp {
        reactivity: derive_reactivity(&abstract_type, has_default),
        canonical_name: name.clone(),
        abstract_type,
        concrete_type,
        sources: vec![PropSourceMapping {
            repo_id: file_path.to_string(),
            original_name: name,
            original_type: raw_type.to_string(),
        }],
        confidence: 0.0,
        conflicts: vec![],
    }
}

/// Assemble the final CAM component from converted props (shared by all
/// extractor flavors).
fn build_component(
    shape: ComponentShape,
    name: &str,
    line_start: usize,
    props: Vec<CanonicalAbstractProp>,
    file_path: &str,
) -> CanonicalAbstractComponent {
    let confidence = compute_confidence(&props, shape.base_confidence);
    let events = extract_events_from_props(&props);
    let extracted_parts = populate_extracted_parts(&props);
    let props: Vec<_> = props
        .into_iter()
        .map(|mut p| {
            p.confidence = confidence;
            p
        })
        .collect();

    CanonicalAbstractComponent {
        id: format!("{}:{}:{}", shape.id_prefix, file_path, name),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: compute_purpose_hash(name, &props),
            normalized_prop_names: props.iter().map(|p| p.canonical_name.clone()).collect(),
        },
        props,
        events,
        extracted_state_machine: None,
        extracted_parts,
        source_repos: vec![SourceAttribution {
            repo_url: "local".to_string(),
            file_path: file_path.to_string(),
            line_start,
        }],
        provided_context: shape.provided_context,
        consumed_contexts: shape.consumed_contexts,
    }
}

/// Convert a small element count to `f32` without a lossy `as` cast.
///
/// Prop/component counts far exceed `u16` range only in adversarial
/// inputs; capping there is harmless for confidence math.
#[must_use]
fn count_as_f32(n: usize) -> f32 {
    f32::from(u16::try_from(n).unwrap_or(u16::MAX))
}

/// Derive the reactivity model from an abstract type and default presence.
#[must_use]
pub fn derive_reactivity(cam_type: &AbstractPropType, has_default: bool) -> AbstractReactivity {
    match (cam_type, has_default) {
        (AbstractPropType::ControlledValue(_), _) => AbstractReactivity::Controlled,
        (AbstractPropType::UncontrolledValue(_), _) | (AbstractPropType::ControlFlag, false) => {
            AbstractReactivity::Uncontrolled
        }
        _ => AbstractReactivity::Static,
    }
}

/// Compute extraction confidence: the base score penalized by the share of
/// props whose type could not be classified (`Any`).
#[must_use]
pub fn compute_confidence(props: &[CanonicalAbstractProp], base: f32) -> f32 {
    let any_count = props
        .iter()
        .filter(|p| p.abstract_type == AbstractPropType::Any)
        .count();
    let total = props.len().max(1);
    let any_f = count_as_f32(any_count);
    let total_f = count_as_f32(total);
    let any_ratio = any_f / total_f;
    (base - any_ratio * ANY_PENALTY_PER_PROP * any_f).max(0.1)
}

/// Derive semantic events from callback-like props: `on_click` becomes
/// event `click`.
#[must_use]
pub fn extract_events_from_props(props: &[CanonicalAbstractProp]) -> Vec<CanonicalAbstractEvent> {
    props
        .iter()
        .filter_map(|p| {
            let AbstractPropType::AsyncEventHandler(payload_types) = &p.abstract_type else {
                return None;
            };
            let stripped = p
                .canonical_name
                .strip_prefix("on_")
                .or_else(|| p.canonical_name.strip_prefix("on"))
                .unwrap_or(&p.canonical_name);
            let event_name = if stripped.is_empty() {
                p.canonical_name.clone()
            } else {
                lower_first(stripped)
            };
            Some(CanonicalAbstractEvent {
                canonical_name: event_name,
                abstract_payload: AbstractPropType::AsyncEventHandler(payload_types.clone()),
            })
        })
        .collect()
}

/// Lowercase the first character of `s` (used to normalize event names).
fn lower_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Every renderable prop doubles as a selectable part of the component.
#[must_use]
pub fn populate_extracted_parts(props: &[CanonicalAbstractProp]) -> Vec<ExtractedPart> {
    props
        .iter()
        .filter(|prop| matches!(prop.abstract_type, AbstractPropType::Renderable))
        .map(|prop| ExtractedPart {
            name: prop.canonical_name.clone(),
            selectable: true,
        })
        .collect()
}

/// Stable, order-independent semantic fingerprint: FNV-1a over the
/// lower-cased component name and its sorted prop names.
#[must_use]
pub fn compute_purpose_hash(name: &str, props: &[CanonicalAbstractProp]) -> String {
    let mut bytes = Vec::with_capacity(64);
    bytes.extend_from_slice(name.to_lowercase().as_bytes());
    let mut prop_names: Vec<&str> = props.iter().map(|p| p.canonical_name.as_str()).collect();
    prop_names.sort_unstable();
    for pn in &prop_names {
        bytes.push(0x1f); // unit separator between name parts
        bytes.extend_from_slice(pn.as_bytes());
    }
    format!("{:016x}", ucp_core::fnv1a64(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_are_derived_and_normalized() {
        let props = vec![
            make_test_prop("on_click", AbstractPropType::AsyncEventHandler(vec![])),
            make_test_prop("onChange", AbstractPropType::AsyncEventHandler(vec![])),
            make_test_prop("disabled", AbstractPropType::ControlFlag),
        ];
        let events = extract_events_from_props(&props);
        let names: Vec<_> = events.iter().map(|e| e.canonical_name.clone()).collect();
        assert_eq!(names, vec!["click", "change"]);
    }

    #[test]
    fn bare_on_prop_keeps_name() {
        let props = vec![make_test_prop(
            "on",
            AbstractPropType::AsyncEventHandler(vec![]),
        )];
        let events = extract_events_from_props(&props);
        assert_eq!(events[0].canonical_name, "on");
    }

    #[test]
    fn confidence_penalizes_untyped_props() {
        let typed = vec![make_test_prop("a", AbstractPropType::ControlFlag)];
        let untyped = vec![make_test_prop("a", AbstractPropType::Any)];
        let base = 0.95;
        assert!(compute_confidence(&typed, base) > compute_confidence(&untyped, base));
        // Floor at 0.1 even with many untyped props.
        let many: Vec<_> = (0..50)
            .map(|i| make_test_prop(&format!("p{i}"), AbstractPropType::Any))
            .collect();
        assert!((compute_confidence(&many, base) - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn purpose_hash_is_order_independent() {
        let a = vec![
            make_test_prop("alpha", AbstractPropType::ControlFlag),
            make_test_prop("beta", AbstractPropType::ControlFlag),
        ];
        let b = vec![
            make_test_prop("beta", AbstractPropType::ControlFlag),
            make_test_prop("alpha", AbstractPropType::ControlFlag),
        ];
        assert_eq!(
            compute_purpose_hash("Button", &a),
            compute_purpose_hash("Button", &b)
        );
        // Case-insensitive on the component name.
        assert_eq!(
            compute_purpose_hash("Button", &a),
            compute_purpose_hash("button", &b)
        );
        assert_ne!(
            compute_purpose_hash("Button", &a),
            compute_purpose_hash("Card", &a)
        );
    }

    #[test]
    fn reactivity_follows_type_shape() {
        assert_eq!(
            derive_reactivity(
                &AbstractPropType::ControlledValue(Box::new(AbstractPropType::Any)),
                false
            ),
            AbstractReactivity::Controlled
        );
        assert_eq!(
            derive_reactivity(&AbstractPropType::ControlFlag, true),
            AbstractReactivity::Static
        );
        assert_eq!(
            derive_reactivity(&AbstractPropType::ControlFlag, false),
            AbstractReactivity::Uncontrolled
        );
        assert_eq!(
            derive_reactivity(
                &AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                false
            ),
            AbstractReactivity::Static
        );
    }

    fn make_test_prop(name: &str, ty: AbstractPropType) -> CanonicalAbstractProp {
        CanonicalAbstractProp {
            canonical_name: name.to_string(),
            abstract_type: ty,
            reactivity: AbstractReactivity::Static,
            concrete_type: None,
            sources: vec![],
            confidence: 0.9,
            conflicts: vec![],
        }
    }
}
