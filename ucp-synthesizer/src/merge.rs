//! Merging of multiple `SynthesisOutput` specs into one canonical spec.

use std::collections::HashMap;

use ucp_core::cam::{
    CanonicalAbstractComponent, CanonicalAbstractEvent, CanonicalAbstractProp, Conflict,
    ResolutionStrategy,
};
use ucp_core::Result;

use crate::pipeline::conflicts::scan_conflicts;
use crate::pipeline::output::{MergeRecord, PipelineStats, SynthesisOutput};
use crate::utils::unix_timestamp_string;

#[derive(Debug, Clone)]
/// Options for controlling merge behavior.
#[derive(Default)]
pub struct MergeOptions {
    /// Per-source confidence weights (source label → weight in `[0,1]`).
    /// When set, conflicting prop types are resolved in favor of the
    /// highest-weighted source.
    pub weights: Option<HashMap<String, f32>>,
    /// Existing spec to merge into incrementally.
    pub incremental_base: Option<SynthesisOutput>,
}

/// Merge multiple [`SynthesisOutput`] specs into one.
///
/// Components with equal [`SemanticFingerprint::purpose_hash`] are
/// deduplicated into a single `unified:<Name>` component whose props,
/// events, parts, state machine, and source attributions are the union of
/// the group.
///
/// Supports incremental merge (via [`MergeOptions::incremental_base`]) and
/// weighted conflict resolution (via [`MergeOptions::weights`]).
///
/// # Errors
///
/// Propagates serialization errors from the underlying pipeline types.
pub fn merge_specs(specs: &[SynthesisOutput], options: MergeOptions) -> Result<SynthesisOutput> {
    let (base_components, base_stats) = if let Some(base) = options.incremental_base {
        (base.components, base.stats)
    } else {
        (Vec::new(), PipelineStats::default())
    };

    if specs.is_empty() {
        return Ok(SynthesisOutput {
            ucp_version: ucp_core::UCP_SPEC_VERSION.to_string(),
            components: base_components,
            provenance: None,
            curation_log: None,
            stats: base_stats,
        });
    }

    // (origin spec index, component); base components get origin usize::MAX
    let mut all_components: Vec<(usize, CanonicalAbstractComponent)> = base_components
        .into_iter()
        .map(|c| (usize::MAX, c))
        .collect();
    let mut total_scanned = base_stats.files_scanned;
    let mut total_parsed = base_stats.files_parsed;
    let mut any_llm = base_stats.llm_enriched;

    for (spec_idx, spec) in specs.iter().enumerate() {
        total_scanned += spec.stats.files_scanned;
        total_parsed += spec.stats.files_parsed;
        any_llm = any_llm || spec.stats.llm_enriched;
        for comp in &spec.components {
            all_components.push((spec_idx, comp.clone()));
        }
    }

    let mut deduped = deduplicate_components(&mut all_components);
    let components_found = deduped.len();
    let conflicts_detected = detect_and_mark_conflicts(&mut deduped);

    if let Some(weights) = options.weights {
        apply_weighted_resolution(&mut deduped, specs, &weights);
    }

    deduped.sort_by(|a, b| a.id.cmp(&b.id));

    // Build provenance records for all input specs.
    let provenance: Vec<MergeRecord> = specs
        .iter()
        .enumerate()
        .map(|(i, spec)| MergeRecord {
            source_spec_path: format!("input-{i}"),
            source_fingerprint: format!("{:016x}", spec.components.len()),
            merged_at: unix_timestamp_string(),
        })
        .collect();

    Ok(SynthesisOutput {
        ucp_version: ucp_core::UCP_SPEC_VERSION.to_string(),
        components: deduped,
        provenance: Some(provenance),
        curation_log: None,
        stats: PipelineStats {
            files_scanned: total_scanned,
            files_parsed: total_parsed,
            components_found,
            conflicts_detected,
            llm_enriched: any_llm,
        },
    })
}

/// A conflict attachment discovered during the scan phase, applied after
/// scanning to keep immutable and mutable borrows apart.
struct PendingAttachment {
    comp_idx: usize,
    prop_indices: Vec<usize>,
    conflict_id: String,
    field: String,
    present_in: Vec<String>,
}

/// Detect cross- and intra-spec conflicts on deduplicated components and
/// return the number of conflict attachments made.
fn detect_and_mark_conflicts(deduped: &mut [CanonicalAbstractComponent]) -> usize {
    let scan = scan_conflicts(deduped);
    let mut counter = scan.detected.len();
    let mut attachments = 0usize;

    for detected in scan.detected {
        for &idx in &detected.member_indices {
            if let Some(prop) = deduped[idx]
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
                attachments += 1;
            }
        }
    }

    // Intra-component conflicts: duplicate prop names with different types
    // inside one merged component. Conflicts are collected first (shared
    // immutable borrows) and attached afterwards (mutable borrows), which
    // keeps the two phases from overlapping.
    let mut pending: Vec<PendingAttachment> = Vec::new();
    for (comp_idx, comp) in deduped.iter().enumerate() {
        for (prop_name, prop_indices) in prop_duplicate_groups(comp) {
            if prop_indices.len() <= 1 {
                continue;
            }
            let variants: Vec<String> = prop_indices
                .iter()
                .map(|&i| format!("{:?}", comp.props[i].abstract_type))
                .collect();
            let mut distinct = variants.clone();
            distinct.sort();
            distinct.dedup();
            if distinct.len() <= 1 {
                continue;
            }
            counter += 1;
            pending.push(PendingAttachment {
                comp_idx,
                prop_indices,
                conflict_id: format!("intra_{counter:03}"),
                field: format!("props.{prop_name}"),
                present_in: comp
                    .source_repos
                    .iter()
                    .map(|s| s.file_path.clone())
                    .collect(),
            });
        }
    }

    for pending_conflict in pending {
        for &i in &pending_conflict.prop_indices {
            deduped[pending_conflict.comp_idx].props[i]
                .conflicts
                .push(Conflict {
                    id: pending_conflict.conflict_id.clone(),
                    field: pending_conflict.field.clone(),
                    present_in: pending_conflict.present_in.clone(),
                    absent_in: vec![],
                    confidence: 0.7,
                    resolution_suggestion: ResolutionStrategy::IncludeMajority,
                });
            attachments += 1;
        }
    }

    attachments
}

/// Group prop indices by name for duplicate detection.
fn prop_duplicate_groups(comp: &CanonicalAbstractComponent) -> Vec<(String, Vec<usize>)> {
    let mut groups: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, prop) in comp.props.iter().enumerate() {
        groups
            .entry(prop.canonical_name.as_str())
            .or_default()
            .push(i);
    }
    groups
        .into_iter()
        .map(|(name, idx)| (name.to_string(), idx))
        .collect()
}

/// Resolve conflicts in favor of the highest-weighted source, when weights
/// are provided for the sources involved.
fn apply_weighted_resolution(
    components: &mut [CanonicalAbstractComponent],
    specs: &[SynthesisOutput],
    weights: &HashMap<String, f32>,
) {
    for comp in components.iter_mut() {
        for prop in &mut comp.props {
            if prop.conflicts.is_empty() {
                continue;
            }
            let mut best_weight = -1.0f32;
            let mut best_type = prop.abstract_type.clone();
            for conflict in &prop.conflicts {
                for present in &conflict.present_in {
                    if let Some(&w) = weights.get(present) {
                        if w > best_weight {
                            best_weight = w;
                            for spec in specs {
                                for sc in &spec.components {
                                    if sc.id != comp.id {
                                        continue;
                                    }
                                    if let Some(p) = sc
                                        .props
                                        .iter()
                                        .find(|p| p.canonical_name == prop.canonical_name)
                                    {
                                        best_type = p.abstract_type.clone();
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if best_weight >= 0.0 {
                prop.abstract_type = best_type;
                prop.conflicts.clear();
            }
        }
    }
}

/// Group components by semantic fingerprint and merge each group into one
/// canonical component. Deterministic: groups are keyed in sorted order and
/// the result is sorted by best display name.
fn deduplicate_components(
    components: &mut [(usize, CanonicalAbstractComponent)],
) -> Vec<CanonicalAbstractComponent> {
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, (_, comp)) in components.iter().enumerate() {
        groups
            .entry(comp.semantic_fingerprint.purpose_hash.clone())
            .or_default()
            .push(idx);
    }
    let mut group_keys: Vec<&String> = groups.keys().collect();
    group_keys.sort();

    let mut result = Vec::new();

    for key in group_keys {
        let indices = &groups[key];
        let first_idx = indices[0];
        let mut base = components[first_idx].1.clone();

        for &idx in &indices[1..] {
            let other = &components[idx].1;
            merge_props_into(&mut base.props, &other.props);
            merge_events_into(&mut base.events, &other.events);
            merge_state_machine_into(&mut base, other);
            merge_contexts_into(&mut base, other);
            merge_parts_into(&mut base, other);
            base.source_repos.extend(other.source_repos.iter().cloned());
        }

        let best_name = indices
            .iter()
            .map(|&idx| components[idx].1.id.rsplit(':').next().unwrap_or_default())
            .min_by_key(|n| n.len())
            .unwrap_or_default()
            .to_string();

        base.id = format!("unified:{best_name}");
        result.push(base);
    }

    result.sort_by(|a, b| a.id.cmp(&b.id));
    result
}

/// Fill a missing state machine from `other`.
fn merge_state_machine_into(
    base: &mut CanonicalAbstractComponent,
    other: &CanonicalAbstractComponent,
) {
    if base.extracted_state_machine.is_none() && other.extracted_state_machine.is_some() {
        base.extracted_state_machine
            .clone_from(&other.extracted_state_machine);
    }
}

/// Fill missing provided/consumed context information from `other`.
fn merge_contexts_into(base: &mut CanonicalAbstractComponent, other: &CanonicalAbstractComponent) {
    if base.provided_context.is_none() && other.provided_context.is_some() {
        base.provided_context.clone_from(&other.provided_context);
    }
    if base.consumed_contexts.is_empty() && !other.consumed_contexts.is_empty() {
        base.consumed_contexts.clone_from(&other.consumed_contexts);
    }
}

/// Fill missing extracted parts from `other`.
fn merge_parts_into(base: &mut CanonicalAbstractComponent, other: &CanonicalAbstractComponent) {
    if base.extracted_parts.is_empty() && !other.extracted_parts.is_empty() {
        base.extracted_parts.clone_from(&other.extracted_parts);
    }
}

/// Merge `other_props` into `base_props`:
///
/// - same name + same abstract type → keep the higher confidence, union sources
/// - same name + different abstract type → keep both variants (a conflict)
/// - new name → append
fn merge_props_into(
    base_props: &mut Vec<CanonicalAbstractProp>,
    other_props: &[CanonicalAbstractProp],
) {
    let mut existing_names: HashMap<String, usize> = HashMap::with_capacity(base_props.len());
    for (i, prop) in base_props.iter().enumerate() {
        existing_names.insert(prop.canonical_name.clone(), i);
    }

    for other_prop in other_props {
        if let Some(&base_idx) = existing_names.get(&other_prop.canonical_name) {
            let base_prop = &base_props[base_idx];
            if base_prop.abstract_type == other_prop.abstract_type {
                if other_prop.confidence > base_prop.confidence {
                    base_props[base_idx].confidence = other_prop.confidence;
                }
                base_props[base_idx]
                    .sources
                    .extend(other_prop.sources.iter().cloned());
            } else {
                base_props.push(other_prop.clone());
            }
        } else {
            existing_names.insert(other_prop.canonical_name.clone(), base_props.len());
            base_props.push(other_prop.clone());
        }
    }
}

/// Append events from `other_events` whose name is not yet present.
fn merge_events_into(
    base_events: &mut Vec<CanonicalAbstractEvent>,
    other_events: &[CanonicalAbstractEvent],
) {
    let existing: std::collections::HashSet<String> = base_events
        .iter()
        .map(|ev| ev.canonical_name.clone())
        .collect();
    for ev in other_events {
        if !existing.contains(&ev.canonical_name) {
            base_events.push(ev.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::output::PipelineStats;

    fn spec(components: Vec<CanonicalAbstractComponent>) -> SynthesisOutput {
        let components_found = components.len();
        SynthesisOutput {
            ucp_version: ucp_core::UCP_SPEC_VERSION.to_string(),
            components,
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found,
                conflicts_detected: 0,
                llm_enriched: false,
            },
            provenance: None,
            curation_log: None,
        }
    }

    #[test]
    fn merging_empty_spec_list_returns_base() {
        let base = spec(vec![]);
        let merged = merge_specs(
            &[],
            MergeOptions {
                incremental_base: Some(base),
                ..MergeOptions::default()
            },
        )
        .unwrap();
        assert_eq!(merged.ucp_version, ucp_core::UCP_SPEC_VERSION);
        assert!(merged.provenance.is_none());
    }

    #[test]
    fn merge_is_deterministic_across_runs() {
        let a = spec(vec![]);
        let out1 = merge_specs(std::slice::from_ref(&a), MergeOptions::default()).unwrap();
        let out2 = merge_specs(&[a], MergeOptions::default()).unwrap();
        let ids1: Vec<_> = out1.components.iter().map(|c| c.id.clone()).collect();
        let ids2: Vec<_> = out2.components.iter().map(|c| c.id.clone()).collect();
        assert_eq!(ids1, ids2);
    }
}
