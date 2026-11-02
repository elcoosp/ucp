//! Conflict detection shared by the extraction pipeline and the merge step.
//!
//! The engine groups components by [`SemanticFingerprint::purpose_hash`],
//! then for every prop name that appears with more than one distinct
//! abstract type inside a group, attaches a [`Conflict`] to every
//! participating prop.

use std::collections::{BTreeMap, HashSet};

use ucp_core::cam::{CanonicalAbstractComponent, Conflict, ResolutionStrategy};

/// A conflict detected across `member` components for one prop.
pub struct DetectedConflict {
    /// Canonical prop name the conflict is about.
    pub prop_name: String,
    /// Deterministic conflict id (`<prop>:<hash>`).
    pub conflict_id: String,
    /// Component ids where the prop is declared.
    pub present_in: Vec<String>,
    /// Component ids of the group where the prop is missing.
    pub absent_in: Vec<String>,
    /// Indices into the scanned component slice.
    pub member_indices: Vec<usize>,
    /// Mean confidence of the participating props.
    pub confidence: f32,
    /// Suggested resolution for the type disagreement.
    pub resolution: ResolutionStrategy,
}

/// Outcome of running the conflict engine over a component list.
pub struct ConflictScan {
    /// Conflicts ready to be attached, in deterministic order.
    pub detected: Vec<DetectedConflict>,
}

/// Scan components for conflicting prop types and return the conflicts.
///
/// The scan is deterministic: components are grouped by fingerprint hash in
/// a `BTreeMap`, groups with a single member are skipped, and conflict ids
/// are assigned in sorted prop-name order.
#[must_use]
pub fn scan_conflicts(components: &[CanonicalAbstractComponent]) -> ConflictScan {
    let mut hash_groups: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (idx, comp) in components.iter().enumerate() {
        hash_groups
            .entry(comp.semantic_fingerprint.purpose_hash.as_str())
            .or_default()
            .push(idx);
    }

    let mut detected = Vec::new();
    let mut counter = 0u32;

    for group_indices in hash_groups.values() {
        if group_indices.len() <= 1 {
            continue;
        }

        // prop name -> component indices in this group declaring the prop
        let mut prop_entries: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        for &idx in group_indices {
            for prop in &components[idx].props {
                prop_entries
                    .entry(prop.canonical_name.as_str())
                    .or_default()
                    .push(idx);
            }
        }

        for (prop_name, member_indices) in prop_entries {
            if member_indices.len() <= 1 {
                continue;
            }
            let type_variants = distinct_type_variants(components, &member_indices, prop_name);
            if type_variants.len() <= 1 {
                continue;
            }

            counter += 1;
            let present_in: Vec<String> = member_indices
                .iter()
                .map(|&idx| source_label(components, idx))
                .collect();
            let member_set: HashSet<usize> = member_indices.iter().copied().collect();
            let absent_in: Vec<String> = (0..components.len())
                .filter(|i| !member_set.contains(i))
                .map(|idx| source_label(components, idx))
                .filter(|s| !present_in.contains(s))
                .collect();

            // Bigger groups are noisier and less trustworthy.
            let (confidence, resolution) = if member_indices.len() > 2 {
                (0.4, ResolutionStrategy::FlagForHumanReview)
            } else {
                (0.7, ResolutionStrategy::IncludeMajority)
            };

            detected.push(DetectedConflict {
                prop_name: prop_name.to_string(),
                conflict_id: format!("conf_{counter:03}"),
                present_in,
                absent_in,
                member_indices,
                confidence,
                resolution,
            });
        }
    }

    ConflictScan { detected }
}

/// Total number of conflicts detected across `components` (pipeline flavor).
/// Count conflicts currently attached to any prop in the component list.
#[must_use]
pub fn count_conflicts(components: &[CanonicalAbstractComponent]) -> usize {
    components
        .iter()
        .map(|c| c.props.iter().filter(|p| !p.conflicts.is_empty()).count())
        .sum()
}

/// Attach scanned conflicts to the components (pipeline flavor).
/// Scan and attach conflicts to the participating props in place.
pub fn attach_conflicts(components: &mut [CanonicalAbstractComponent]) {
    let scan = scan_conflicts(components);
    for detected in scan.detected {
        for &idx in &detected.member_indices {
            if let Some(prop) = components[idx]
                .props
                .iter_mut()
                .find(|p| p.canonical_name == detected.prop_name)
            {
                prop.conflicts.push(Conflict {
                    id: detected.conflict_id.clone(),
                    field: format!("props.{}", detected.prop_name),
                    present_in: detected.present_in.clone(),
                    absent_in: detected.absent_in.clone(),
                    confidence: detected.confidence,
                    resolution_suggestion: detected.resolution.clone(),
                });
            }
        }
    }
}

/// Number of distinct abstract type spellings for one prop among members.
fn distinct_type_variants(
    components: &[CanonicalAbstractComponent],
    member_indices: &[usize],
    prop_name: &str,
) -> Vec<String> {
    let mut variants: Vec<String> = member_indices
        .iter()
        .map(|&idx| {
            components[idx]
                .props
                .iter()
                .find(|p| p.canonical_name == prop_name)
                .map_or_else(
                    || "missing".to_string(),
                    |p| format!("{:?}", p.abstract_type),
                )
        })
        .collect();
    variants.sort();
    variants.dedup();
    variants
}

/// Best-effort source label for a component (first source file path).
fn source_label(components: &[CanonicalAbstractComponent], idx: usize) -> String {
    components[idx]
        .source_repos
        .first()
        .map_or_else(|| "unknown".to_string(), |s| s.file_path.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::unification::compute_purpose_hash;
    use ucp_core::cam::{
        AbstractPropType, AbstractReactivity, CanonicalAbstractProp, SemanticFingerprint,
    };

    fn prop(name: &str, ty: AbstractPropType) -> CanonicalAbstractProp {
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

    fn comp(id: &str, file: &str, props: Vec<CanonicalAbstractProp>) -> CanonicalAbstractComponent {
        CanonicalAbstractComponent {
            id: id.to_string(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: compute_purpose_hash(id.rsplit(':').next().unwrap_or(id), &props),
                normalized_prop_names: props.iter().map(|p| p.canonical_name.clone()).collect(),
            },
            props,
            events: vec![],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![ucp_core::cam::SourceAttribution {
                repo_url: "local".into(),
                file_path: file.to_string(),
                line_start: 1,
            }],
            provided_context: None,
            consumed_contexts: vec![],
        }
    }

    #[test]
    fn no_conflicts_without_type_disagreement() {
        let mut comps = vec![
            comp(
                "rust:a.rs:Btn",
                "a.rs",
                vec![prop("size", AbstractPropType::ControlFlag)],
            ),
            comp(
                "tsx:b.tsx:Btn",
                "b.tsx",
                vec![prop("size", AbstractPropType::ControlFlag)],
            ),
        ];
        attach_conflicts(&mut comps);
        assert!(comps
            .iter()
            .all(|c| c.props.iter().all(|p| p.conflicts.is_empty())));
        assert_eq!(count_conflicts(&comps), 0);
    }

    #[test]
    fn type_disagreement_attaches_conflict() {
        let mut comps = vec![
            comp(
                "rust:a.rs:Btn",
                "a.rs",
                vec![prop("size", AbstractPropType::ControlFlag)],
            ),
            comp(
                "tsx:b.tsx:Btn",
                "b.tsx",
                vec![prop(
                    "size",
                    AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                )],
            ),
        ];
        attach_conflicts(&mut comps);
        let with_conflict: usize = comps
            .iter()
            .map(|c| c.props.iter().filter(|p| !p.conflicts.is_empty()).count())
            .sum();
        assert_eq!(with_conflict, 2);
        assert_eq!(count_conflicts(&comps), 2);
        let conflict = &comps[0].props[0].conflicts[0];
        assert!(conflict.present_in.contains(&"a.rs".to_string()));
        assert!(conflict.absent_in.is_empty() || !conflict.absent_in.contains(&"a.rs".to_string()));
    }

    #[test]
    fn large_groups_flag_for_human_review() {
        let mut comps = vec![
            comp(
                "rust:a.rs:Btn",
                "a.rs",
                vec![prop("size", AbstractPropType::ControlFlag)],
            ),
            comp(
                "tsx:b.tsx:Btn",
                "b.tsx",
                vec![prop(
                    "size",
                    AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                )],
            ),
            comp(
                "rust:c.rs:Btn",
                "c.rs",
                vec![prop("size", AbstractPropType::ControlFlag)],
            ),
            comp(
                "dioxus:d.rs:Btn",
                "d.rs",
                vec![prop("size", AbstractPropType::ControlFlag)],
            ),
        ];
        attach_conflicts(&mut comps);
        let conflict = &comps[0].props[0].conflicts[0];
        assert!(matches!(
            conflict.resolution_suggestion,
            ResolutionStrategy::FlagForHumanReview
        ));
        assert!((conflict.confidence - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn different_fingerprints_never_conflict() {
        let mut comps = vec![
            comp(
                "rust:a.rs:Btn",
                "a.rs",
                vec![prop("size", AbstractPropType::ControlFlag)],
            ),
            comp(
                "tsx:b.tsx:Dialog",
                "b.tsx",
                vec![prop(
                    "size",
                    AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                )],
            ),
        ];
        attach_conflicts(&mut comps);
        assert_eq!(count_conflicts(&comps), 0);
    }
}
