use std::collections::HashMap;
use ucp_core::cam::*;
use ucp_core::Result;
use crate::pipeline::{PipelineStats, SynthesisOutput};

#[derive(Debug, Clone)]
pub struct MergeOptions {
    pub weights: Option<HashMap<String, f32>>,
    pub incremental_base: Option<SynthesisOutput>,
}

impl Default for MergeOptions {
    fn default() -> Self {
        Self { weights: None, incremental_base: None }
    }
}

pub fn merge_specs(specs: &[SynthesisOutput], options: MergeOptions) -> Result<SynthesisOutput> {
    let (base_components, base_stats) = if let Some(base) = options.incremental_base {
        (base.components, base.stats)
    } else {
        (Vec::new(), PipelineStats {
            files_scanned: 0, files_parsed: 0, components_found: 0,
            conflicts_detected: 0, llm_enriched: false,
        })
    };

    if specs.is_empty() {
        return Ok(SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: base_components,
            provenance: None,
            curation_log: None,
            stats: base_stats,
        });
    }

    let mut all_components: Vec<(usize, CanonicalAbstractComponent)> =
        base_components.into_iter().enumerate().map(|(i, c)| (i, c)).collect();
    let mut total_scanned = base_stats.files_scanned;
    let mut total_parsed = base_stats.files_parsed;
    let mut any_llm = base_stats.llm_enriched;

    for (spec_idx, spec) in specs.iter().enumerate() {
        total_scanned += spec.stats.files_scanned;
        total_parsed += spec.stats.files_parsed;
        if spec.stats.llm_enriched { any_llm = true; }
        for comp in &spec.components {
            all_components.push((spec_idx, comp.clone()));
        }
    }

    let mut deduped = deduplicate_components(&mut all_components);
    let components_found = deduped.len();
    let conflicts_detected = detect_cross_spec_conflicts(&mut deduped);

    if let Some(weights) = options.weights {
        apply_weighted_resolution(&mut deduped, specs, &weights);
    }

    Ok(SynthesisOutput {
        ucp_version: "4.0.0".to_string(),
        components: deduped,
        provenance: None,
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

fn apply_weighted_resolution(
    components: &mut [CanonicalAbstractComponent],
    specs: &[SynthesisOutput],
    weights: &HashMap<String, f32>,
) {
    for comp in components.iter_mut() {
        for prop in &mut comp.props {
            if prop.conflicts.is_empty() { continue; }
            let mut best_weight = -1.0f32;
            let mut best_type = prop.abstract_type.clone();
            for conflict in &prop.conflicts {
                for present in &conflict.present_in {
                    if let Some(&w) = weights.get(present) {
                        if w > best_weight {
                            best_weight = w;
                            for spec in specs {
                                for sc in &spec.components {
                                    if sc.id == comp.id {
                                        if let Some(p) = sc.props.iter().find(|p| p.canonical_name == prop.canonical_name) {
                                            best_type = p.abstract_type.clone();
                                        }
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

    let mut result = Vec::new();

    for indices in groups.values() {
        let first_idx = indices[0];
        let mut base = components[first_idx].1.clone();

        let mut other_props_list: Vec<Vec<CanonicalAbstractProp>> = Vec::new();
        let mut other_events_list: Vec<Vec<CanonicalAbstractEvent>> = Vec::new();
        let mut other_source_repos: Vec<SourceAttribution> = Vec::new();
        let mut fallback_state_machine: Option<StateMachine> = None;
        let mut fallback_context: Option<String> = None;
        let mut fallback_consumed: Option<Vec<String>> = None;
        let mut fallback_parts: Vec<ExtractedPart> = Vec::new();

        for &idx in &indices[1..] {
            let other = &components[idx].1;
            other_props_list.push(other.props.clone());
            other_events_list.push(other.events.clone());
            other_source_repos.extend(other.source_repos.clone());

            if fallback_state_machine.is_none() && other.extracted_state_machine.is_some() {
                fallback_state_machine = other.extracted_state_machine.clone();
            }
            if fallback_context.is_none() && other.provided_context.is_some() {
                fallback_context = other.provided_context.clone();
            }
            if fallback_consumed.is_none() && !other.consumed_contexts.is_empty() {
                fallback_consumed = Some(other.consumed_contexts.clone());
            }
            if fallback_parts.is_empty() && !other.extracted_parts.is_empty() {
                fallback_parts = other.extracted_parts.clone();
            }
        }

        for other_props in &other_props_list {
            merge_props_into(&mut base.props, other_props);
        }
        for other_events in &other_events_list {
            merge_events_into(&mut base.events, other_events);
        }
        base.source_repos.extend(other_source_repos);

        if base.extracted_state_machine.is_none() {
            base.extracted_state_machine = fallback_state_machine;
        }
        if base.provided_context.is_none() {
            if let Some(ctx) = fallback_context {
                base.provided_context = Some(ctx);
            }
        }
        if base.consumed_contexts.is_empty() {
            if let Some(consumed) = fallback_consumed {
                base.consumed_contexts = consumed;
            }
        }
        if base.extracted_parts.is_empty() {
            base.extracted_parts = fallback_parts;
        }

        let best_name = indices
            .iter()
            .map(|&idx| components[idx].1.id.rsplit(':').next().unwrap_or(""))
            .min_by_key(|n| n.len())
            .unwrap_or("")
            .to_string();

        base.id = format!("unified:{}", best_name);
        result.push(base);
    }

    result
}

fn merge_props_into(
    base_props: &mut Vec<CanonicalAbstractProp>,
    other_props: &[CanonicalAbstractProp],
) {
    let mut existing_names: HashMap<String, usize> = HashMap::new();
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
            base_props.push(other_prop.clone());
        }
    }
}

fn merge_events_into(
    base_events: &mut Vec<CanonicalAbstractEvent>,
    other_events: &[CanonicalAbstractEvent],
) {
    let mut existing: HashMap<String, bool> = HashMap::new();
    for ev in base_events.iter() {
        existing.insert(ev.canonical_name.clone(), true);
    }
    for ev in other_events {
        if !existing.contains_key(&ev.canonical_name) {
            base_events.push(ev.clone());
        }
    }
}

fn detect_cross_spec_conflicts(components: &mut [CanonicalAbstractComponent]) -> usize {
    let mut hash_groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, comp) in components.iter().enumerate() {
        hash_groups
            .entry(comp.semantic_fingerprint.purpose_hash.clone())
            .or_default()
            .push(idx);
    }

    let mut conflict_id_counter = 0u32;
    let mut total_conflicts = 0usize;

    for indices in hash_groups.values() {
        if indices.len() <= 1 {
            continue;
        }

        let mut prop_entries: HashMap<String, Vec<usize>> = HashMap::new();
        for &idx in indices {
            for prop in &components[idx].props {
                prop_entries
                    .entry(prop.canonical_name.clone())
                    .or_default()
                    .push(idx);
            }
        }

        for (prop_name, member_indices) in &prop_entries {
            if member_indices.len() <= 1 {
                continue;
            }

            let mut type_variants: Vec<String> = member_indices
                .iter()
                .map(|&idx| {
                    components[idx]
                        .props
                        .iter()
                        .find(|p| p.canonical_name == *prop_name)
                        .map(|p| format!("{:?}", p.abstract_type))
                        .unwrap_or_else(|| "missing".to_string())
                })
                .collect();
            type_variants.sort();
            type_variants.dedup();

            if type_variants.len() <= 1 {
                continue;
            }

            conflict_id_counter += 1;
            let conflict_id = format!("cross_{:03}", conflict_id_counter);

            let present_in: Vec<String> = member_indices
                .iter()
                .map(|&idx| {
                    components[idx]
                        .source_repos
                        .first()
                        .map(|s| s.file_path.clone())
                        .unwrap_or_else(|| "unknown".to_string())
                })
                .collect();

            let has_count = member_indices.len();
            let confidence = if has_count > 2 { 0.4 } else { 0.7 };
            let resolution = if has_count > 2 {
                ResolutionStrategy::FlagForHumanReview
            } else {
                ResolutionStrategy::IncludeMajority
            };

            for &idx in member_indices {
                if let Some(prop) = components[idx]
                    .props
                    .iter_mut()
                    .find(|p| p.canonical_name == *prop_name)
                {
                    prop.conflicts.push(Conflict {
                        id: conflict_id.clone(),
                        field: format!("props.{}", prop_name),
                        present_in: present_in.clone(),
                        absent_in: vec![],
                        confidence,
                        resolution_suggestion: resolution.clone(),
                    });
                    total_conflicts += 1;
                }
            }
        }
    }

    for comp in components.iter_mut() {
        let mut prop_groups: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, prop) in comp.props.iter().enumerate() {
            prop_groups
                .entry(prop.canonical_name.clone())
                .or_default()
                .push(i);
        }

        for (prop_name, prop_indices) in &prop_groups {
            if prop_indices.len() <= 1 {
                continue;
            }

            let mut type_variants: Vec<String> = prop_indices
                .iter()
                .map(|&i| format!("{:?}", comp.props[i].abstract_type))
                .collect();
            type_variants.sort();
            type_variants.dedup();

            if type_variants.len() <= 1 {
                continue;
            }

            conflict_id_counter += 1;
            let conflict_id = format!("intra_{:03}", conflict_id_counter);

            let present_in: Vec<String> = comp
                .source_repos
                .iter()
                .map(|s| s.file_path.clone())
                .collect();

            for &i in prop_indices {
                comp.props[i].conflicts.push(Conflict {
                    id: conflict_id.clone(),
                    field: format!("props.{}", prop_name),
                    present_in: present_in.clone(),
                    absent_in: vec![],
                    confidence: 0.7,
                    resolution_suggestion: ResolutionStrategy::IncludeMajority,
                });
                total_conflicts += 1;
            }
        }
    }

    total_conflicts
}

