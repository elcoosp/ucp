use super::output::{ANY_PENALTY_PER_PROP, BASE_CONFIDENCE_RUST, BASE_CONFIDENCE_TSX};
use crate::extract::rust_ast;
use crate::extract::tsx_ast;
use crate::unify::map_raw_type_with_concrete;
use std::hash::{Hash, Hasher};
use ucp_core::cam::*;
use ucp_core::Result;

pub fn unify_rust_component(
    raw: &rust_ast::RawComponentExtraction,
    file_path: &str,
) -> Result<CanonicalAbstractComponent> {
    let props: Vec<CanonicalAbstractProp> = raw
        .props
        .iter()
        .map(|rp| {
            let (cam_type, concrete_type_opt) = if rp.is_spread_attributes {
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
            let reactivity = derive_reactivity(&cam_type, rp.has_default);
            CanonicalAbstractProp {
                canonical_name: rp.name.clone(),
                abstract_type: cam_type,
                reactivity,
                concrete_type: concrete_type_opt,
                sources: vec![PropSourceMapping {
                    repo_id: file_path.to_string(),
                    original_name: rp.name.clone(),
                    original_type: rp.raw_type.clone(),
                }],
                confidence: 0.0,
                conflicts: vec![],
            }
        })
        .collect();

    let confidence = compute_confidence(&props, BASE_CONFIDENCE_RUST);
    let events = extract_events_from_props(&props);
    let extracted_parts = populate_extracted_parts(&props);
    let props_with_conf: Vec<_> = props
        .into_iter()
        .map(|mut p| {
            p.confidence = confidence;
            p
        })
        .collect();

    Ok(CanonicalAbstractComponent {
        id: format!("rust:{}:{}", file_path, raw.name),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: compute_purpose_hash(&raw.name, &props_with_conf),
            normalized_prop_names: props_with_conf
                .iter()
                .map(|p| p.canonical_name.clone())
                .collect(),
        },
        props: props_with_conf,
        events,
        extracted_state_machine: None,
        extracted_parts,
        source_repos: vec![SourceAttribution {
            repo_url: "local".to_string(),
            file_path: file_path.to_string(),
            line_start: raw.line_start,
        }],
        provided_context: raw.provided_context.clone(),
        consumed_contexts: raw.consumed_contexts.clone(),
    })
}

pub fn unify_tsx_component(
    raw: &tsx_ast::RawTsxExtraction,
    file_path: &str,
) -> Result<CanonicalAbstractComponent> {
    let props: Vec<CanonicalAbstractProp> = raw
        .props
        .iter()
        .map(|rp| {
            let (cam_type, concrete_type_opt) = if rp.raw_type.contains("=>")
                || rp.raw_type.contains("void")
            {
                (
                    AbstractPropType::AsyncEventHandler(vec![]),
                    Some(rp.raw_type.clone()),
                )
            } else {
                map_raw_type_with_concrete(&rp.raw_type).unwrap_or((AbstractPropType::Any, None))
            };
            let reactivity = derive_reactivity(&cam_type, false);
            CanonicalAbstractProp {
                canonical_name: rp.name.clone(),
                abstract_type: cam_type,
                reactivity,
                concrete_type: concrete_type_opt,
                sources: vec![PropSourceMapping {
                    repo_id: file_path.to_string(),
                    original_name: rp.name.clone(),
                    original_type: rp.raw_type.clone(),
                }],
                confidence: 0.0,
                conflicts: vec![],
            }
        })
        .collect();

    let confidence = compute_confidence(&props, BASE_CONFIDENCE_TSX);
    let events = extract_events_from_props(&props);
    let extracted_parts = populate_extracted_parts(&props);
    let props_with_conf: Vec<_> = props
        .into_iter()
        .map(|mut p| {
            p.confidence = confidence;
            p
        })
        .collect();

    Ok(CanonicalAbstractComponent {
        id: format!("tsx:{}:{}", file_path, raw.name),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: compute_purpose_hash(&raw.name, &props_with_conf),
            normalized_prop_names: props_with_conf
                .iter()
                .map(|p| p.canonical_name.clone())
                .collect(),
        },
        props: props_with_conf,
        events,
        extracted_state_machine: None,
        extracted_parts,
        source_repos: vec![SourceAttribution {
            repo_url: "local".to_string(),
            file_path: file_path.to_string(),
            line_start: raw.line_start,
        }],
        provided_context: None,
        consumed_contexts: vec![],
    })
}

pub fn unify_rust_component_struct(
    raw: &rust_ast::RawComponentExtraction,
    file_path: &str,
) -> Result<CanonicalAbstractComponent> {
    let props: Vec<CanonicalAbstractProp> = raw
        .props
        .iter()
        .map(|rp| {
            let (cam_type, concrete_type_opt) = if rp.is_spread_attributes {
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
            let reactivity = derive_reactivity(&cam_type, rp.has_default);
            CanonicalAbstractProp {
                canonical_name: rp.name.clone(),
                abstract_type: cam_type,
                reactivity,
                concrete_type: concrete_type_opt,
                sources: vec![PropSourceMapping {
                    repo_id: file_path.to_string(),
                    original_name: rp.name.clone(),
                    original_type: rp.raw_type.clone(),
                }],
                confidence: 0.0,
                conflicts: vec![],
            }
        })
        .collect();

    let confidence = compute_confidence(&props, BASE_CONFIDENCE_RUST);
    let events = extract_events_from_props(&props);
    let extracted_parts = populate_extracted_parts(&props);
    let props_with_conf: Vec<_> = props
        .into_iter()
        .map(|mut p| {
            p.confidence = confidence;
            p
        })
        .collect();

    Ok(CanonicalAbstractComponent {
        id: format!("rust-struct:{}:{}", file_path, raw.name),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: compute_purpose_hash(&raw.name, &props_with_conf),
            normalized_prop_names: props_with_conf
                .iter()
                .map(|p| p.canonical_name.clone())
                .collect(),
        },
        props: props_with_conf,
        events,
        extracted_state_machine: None,
        extracted_parts,
        source_repos: vec![SourceAttribution {
            repo_url: "local".to_string(),
            file_path: file_path.to_string(),
            line_start: raw.line_start,
        }],
        provided_context: raw.provided_context.clone(),
        consumed_contexts: raw.consumed_contexts.clone(),
    })
}

pub fn derive_reactivity(cam_type: &AbstractPropType, has_default: bool) -> AbstractReactivity {
    match cam_type {
        AbstractPropType::ControlledValue(_) => AbstractReactivity::Controlled,
        AbstractPropType::UncontrolledValue(_) => AbstractReactivity::Uncontrolled,
        AbstractPropType::ControlFlag if has_default => AbstractReactivity::Static,
        AbstractPropType::ControlFlag => AbstractReactivity::Uncontrolled,
        _ => AbstractReactivity::Static,
    }
}

pub fn compute_confidence(props: &[CanonicalAbstractProp], base: f32) -> f32 {
    let any_count = props
        .iter()
        .filter(|p| p.abstract_type == AbstractPropType::Any)
        .count();
    let total = props.len().max(1);
    let any_ratio = any_count as f32 / total as f32;
    (base - any_ratio * ANY_PENALTY_PER_PROP * any_count as f32).max(0.1)
}

pub fn extract_events_from_props(props: &[CanonicalAbstractProp]) -> Vec<CanonicalAbstractEvent> {
    props
        .iter()
        .filter_map(|p| {
            if let AbstractPropType::AsyncEventHandler(payload_types) = &p.abstract_type {
                let event_name = p
                    .canonical_name
                    .strip_prefix("on_")
                    .or_else(|| p.canonical_name.strip_prefix("on"))
                    .unwrap_or(&p.canonical_name)
                    .to_string();
                let event_name = if event_name.is_empty() {
                    p.canonical_name.clone()
                } else {
                    let mut chars = event_name.chars();
                    match chars.next() {
                        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
                        None => String::new(),
                    }
                };
                Some(CanonicalAbstractEvent {
                    canonical_name: event_name,
                    abstract_payload: AbstractPropType::AsyncEventHandler(payload_types.clone()),
                })
            } else {
                None
            }
        })
        .collect()
}

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

pub fn compute_purpose_hash(name: &str, props: &[CanonicalAbstractProp]) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    name.to_lowercase().hash(&mut hasher);
    let mut prop_names: Vec<&str> = props.iter().map(|p| p.canonical_name.as_str()).collect();
    prop_names.sort();
    for pn in &prop_names {
        pn.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}
