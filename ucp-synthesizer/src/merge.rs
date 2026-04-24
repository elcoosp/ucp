use std::collections::HashMap;
use ucp_core::cam::*;
use ucp_core::Result;

use crate::pipeline::{PipelineStats, SynthesisOutput};

pub fn merge_specs(specs: &[SynthesisOutput]) -> Result<SynthesisOutput> {
    if specs.is_empty() {
        return Ok(SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![],
            stats: PipelineStats {
                files_scanned: 0,
                files_parsed: 0,
                components_found: 0,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        });
    }

    let mut all_components: Vec<(usize, CanonicalAbstractComponent)> = Vec::new();
    let mut total_scanned = 0usize;
    let mut total_parsed = 0usize;
    let mut any_llm = false;

    for (spec_idx, spec) in specs.iter().enumerate() {
        total_scanned += spec.stats.files_scanned;
        total_parsed += spec.stats.files_parsed;
        if spec.stats.llm_enriched {
            any_llm = true;
        }
        for comp in &spec.components {
            all_components.push((spec_idx, comp.clone()));
        }
    }

    let mut deduped = deduplicate_components(&mut all_components);
    let components_found = deduped.len();
    let conflicts_detected = detect_cross_spec_conflicts(&mut deduped);

    Ok(SynthesisOutput {
        ucp_version: "4.0.0".to_string(),
        components: deduped,
        stats: PipelineStats {
            files_scanned: total_scanned,
            files_parsed: total_parsed,
            components_found,
            conflicts_detected,
            llm_enriched: any_llm,
        },
    })
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
        let mut fallback_parts: Vec<ExtractedPart> = Vec::new();

        for &idx in &indices[1..] {
            let other = &components[idx].1;
            other_props_list.push(other.props.clone());
            other_events_list.push(other.events.clone());
            other_source_repos.extend(other.source_repos.clone());

            if fallback_state_machine.is_none() && other.extracted_state_machine.is_some() {
                fallback_state_machine = other.extracted_state_machine.clone();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn empty_spec() -> SynthesisOutput {
        SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![],
            stats: PipelineStats {
                files_scanned: 5,
                files_parsed: 3,
                components_found: 0,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        }
    }

    fn make_component(
        id: &str,
        prop_name: &str,
        prop_type: AbstractPropType,
    ) -> CanonicalAbstractComponent {
        let mut hasher = DefaultHasher::new();
        id.to_lowercase().hash(&mut hasher);
        prop_name.hash(&mut hasher);
        let purpose_hash = format!("{:016x}", hasher.finish());
        CanonicalAbstractComponent {
            id: id.to_string(),
            semantic_fingerprint: ucp_core::cam::SemanticFingerprint {
                purpose_hash,
                normalized_prop_names: vec![prop_name.to_string()],
            },
            props: vec![CanonicalAbstractProp {
                canonical_name: prop_name.to_string(),
                abstract_type: prop_type,
                reactivity: ucp_core::cam::AbstractReactivity::Static,
                sources: vec![],
                confidence: 0.9,
                conflicts: vec![],
            }],
            events: vec![],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![SourceAttribution {
                repo_url: "local".to_string(),
                file_path: format!("{}.rs", id),
                line_start: 1,
            }],
        }
    }

    fn make_component_with_source(
        id: &str,
        prop_name: &str,
        prop_type: AbstractPropType,
        file_path: &str,
    ) -> CanonicalAbstractComponent {
        let mut comp = make_component(id, prop_name, prop_type);
        comp.source_repos = vec![SourceAttribution {
            repo_url: "local".to_string(),
            file_path: file_path.to_string(),
            line_start: 1,
        }];
        comp
    }

    #[test]
    fn merge_empty_specs() {
        assert!(merge_specs(&[]).unwrap().components.is_empty());
    }
    #[test]
    fn merge_single_spec_passthrough() {
        assert_eq!(merge_specs(&[empty_spec()]).unwrap().stats.files_scanned, 5);
    }
    #[test]
    fn merge_two_specs_accumulates_stats() {
        let spec2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![],
            stats: PipelineStats {
                files_scanned: 10,
                files_parsed: 7,
                components_found: 0,
                conflicts_detected: 0,
                llm_enriched: true,
            },
        };
        assert_eq!(
            merge_specs(&[empty_spec(), spec2])
                .unwrap()
                .stats
                .files_scanned,
            15
        );
    }

    #[test]
    fn dedup_same_components_single_output() {
        let comp = make_component_with_source(
            "Button",
            "label",
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            "src/a.rs",
        );
        let spec = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![comp],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let r = merge_specs(&[spec]).unwrap();
        assert_eq!(r.stats.components_found, 1);
        assert!(r.components[0].id.starts_with("unified:"));
    }

    #[test]
    fn dedup_same_components_different_sources() {
        let a = make_component_with_source(
            "Button",
            "label",
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            "leptos/src/button.rs",
        );
        let b = make_component_with_source(
            "Button",
            "label",
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            "react/src/Button.tsx",
        );
        let s1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![a],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let s2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![b],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let r = merge_specs(&[s1, s2]).unwrap();
        assert_eq!(r.stats.components_found, 1);
        assert_eq!(r.components[0].source_repos.len(), 2);
    }

    #[test]
    fn dedup_keeps_higher_confidence() {
        let mut a = make_component(
            "Button",
            "label",
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
        );
        a.props[0].confidence = 0.7;
        let mut b = make_component(
            "Button",
            "label",
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
        );
        b.props[0].confidence = 0.95;
        b.source_repos = vec![SourceAttribution {
            repo_url: "local".to_string(),
            file_path: "other.rs".to_string(),
            line_start: 1,
        }];
        let s1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![a],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let s2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![b],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        assert_eq!(
            merge_specs(&[s1, s2]).unwrap().components[0].props[0].confidence,
            0.95
        );
    }

    #[test]
    fn dedup_keeps_state_machine_from_other() {
        let a = make_component("Dialog", "open", AbstractPropType::ControlFlag);
        let mut b = make_component("Dialog", "open", AbstractPropType::ControlFlag);
        b.extracted_state_machine = Some(StateMachine {
            id: "sm".to_string(),
            initial: "Closed".to_string(),
            states: std::collections::BTreeMap::new(),
        });
        let s1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![a],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let s2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![b],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        assert!(merge_specs(&[s1, s2]).unwrap().components[0]
            .extracted_state_machine
            .is_some());
    }

    #[test]
    fn dedup_keeps_extracted_parts_from_other() {
        let a = make_component("Card", "children", AbstractPropType::Renderable);
        let mut b = make_component("Card", "children", AbstractPropType::Renderable);
        b.extracted_parts = vec![ExtractedPart {
            name: "children".to_string(),
            selectable: true,
        }];
        let s1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![a],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let s2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![b],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        assert!(!merge_specs(&[s1, s2]).unwrap().components[0]
            .extracted_parts
            .is_empty());
    }

    #[test]
    fn dedup_merges_events() {
        let mut a = make_component("Form", "data", AbstractPropType::Any);
        a.events = vec![CanonicalAbstractEvent {
            canonical_name: "Submit".to_string(),
            abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
        }];
        let mut b = make_component("Form", "data", AbstractPropType::Any);
        b.events = vec![CanonicalAbstractEvent {
            canonical_name: "Reset".to_string(),
            abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
        }];
        let s1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![a],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let s2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![b],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        assert_eq!(
            merge_specs(&[s1, s2]).unwrap().components[0].events.len(),
            2
        );
    }

    #[test]
    fn dedup_deduplicates_events() {
        let mut a = make_component(
            "Form",
            "on_submit",
            AbstractPropType::AsyncEventHandler(vec![]),
        );
        a.events = vec![CanonicalAbstractEvent {
            canonical_name: "Submit".to_string(),
            abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
        }];
        let mut b = make_component(
            "Form",
            "on_submit",
            AbstractPropType::AsyncEventHandler(vec![]),
        );
        b.events = vec![CanonicalAbstractEvent {
            canonical_name: "Submit".to_string(),
            abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
        }];
        let s1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![a],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let s2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![b],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        assert_eq!(
            merge_specs(&[s1, s2]).unwrap().components[0].events.len(),
            1
        );
    }

    #[test]
    fn merge_detects_cross_spec_type_conflict_after_dedup() {
        let c1 = make_component("Button", "disabled", AbstractPropType::ControlFlag);
        let c2 = make_component(
            "Button",
            "disabled",
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
        );
        let s1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![c1],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let s2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![c2],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        assert!(merge_specs(&[s1, s2]).unwrap().stats.conflicts_detected > 0);
    }

    #[test]
    fn merge_unrelated_components_no_conflicts() {
        let a = make_component(
            "Button",
            "label",
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
        );
        let b = make_component("Modal", "open", AbstractPropType::ControlFlag);
        let s1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![a],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let s2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![b],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let r = merge_specs(&[s1, s2]).unwrap();
        assert_eq!(r.stats.conflicts_detected, 0);
        assert_eq!(r.stats.components_found, 2);
    }

    #[test]
    fn merge_no_conflict_for_matching_types_after_dedup() {
        let c1 = make_component(
            "Input",
            "value",
            AbstractPropType::ControlledValue(Box::new(AbstractPropType::Any)),
        );
        let c2 = make_component(
            "Input",
            "value",
            AbstractPropType::ControlledValue(Box::new(AbstractPropType::Any)),
        );
        let s1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![c1],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let s2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![c2],
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        assert_eq!(merge_specs(&[s1, s2]).unwrap().stats.conflicts_detected, 0);
    }
}
