use std::collections::HashMap;
use ucp_core::cam::*;
use ucp_core::Result;

use crate::pipeline::{PipelineStats, SynthesisOutput};

/// Merge multiple synthesis outputs into a single unified spec.
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

    let conflicts_detected = detect_cross_spec_conflicts(&mut all_components);

    let components: Vec<CanonicalAbstractComponent> =
        all_components.into_iter().map(|(_, c)| c).collect();
    let components_found = components.len();

    Ok(SynthesisOutput {
        ucp_version: "4.0.0".to_string(),
        components,
        stats: PipelineStats {
            files_scanned: total_scanned,
            files_parsed: total_parsed,
            components_found,
            conflicts_detected,
            llm_enriched: any_llm,
        },
    })
}

fn detect_cross_spec_conflicts(
    components: &mut [(usize, CanonicalAbstractComponent)],
) -> usize {
    let mut hash_groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, (_, comp)) in components.iter().enumerate() {
        hash_groups
            .entry(comp.semantic_fingerprint.purpose_hash.clone())
            .or_default()
            .push(idx);
    }

    let mut conflict_id_counter = 0u32;
    let mut total_conflicts = 0usize;

    for (_hash, indices) in &hash_groups {
        if indices.len() <= 1 {
            continue;
        }

        let mut prop_entries: HashMap<String, Vec<usize>> = HashMap::new();
        for &idx in indices {
            let comp = &components[idx].1;
            for prop in &comp.props {
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
                        .1
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
                    let spec_idx = components[idx].0;
                    let comp = &components[idx].1;
                    comp.source_repos
                        .first()
                        .map(|s| format!("spec{}:{}", spec_idx, s.file_path))
                        .unwrap_or_else(|| format!("spec{}:unknown", spec_idx))
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
                    .1
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

    #[test]
    fn merge_empty_specs() {
        let result = merge_specs(&[]).unwrap();
        assert!(result.components.is_empty());
    }

    #[test]
    fn merge_single_spec_passthrough() {
        let spec = empty_spec();
        let result = merge_specs(&[spec]).unwrap();
        assert_eq!(result.stats.files_scanned, 5);
    }

    #[test]
    fn merge_two_specs_accumulates_stats() {
        let spec1 = empty_spec();
        let spec2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(),
            components: vec![],
            stats: PipelineStats {
                files_scanned: 10, files_parsed: 7, components_found: 0,
                conflicts_detected: 0, llm_enriched: true,
            },
        };
        let result = merge_specs(&[spec1, spec2]).unwrap();
        assert_eq!(result.stats.files_scanned, 15);
        assert_eq!(result.stats.files_parsed, 10);
        assert!(result.stats.llm_enriched);
    }

    #[test]
    fn merge_detects_cross_spec_type_conflict() {
        let comp1 = make_component("Button", "disabled", AbstractPropType::ControlFlag);
        let comp2 = make_component("Button", "disabled", AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)));

        let spec1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(), components: vec![comp1],
            stats: PipelineStats { files_scanned: 1, files_parsed: 1, components_found: 1, conflicts_detected: 0, llm_enriched: false },
        };
        let spec2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(), components: vec![comp2],
            stats: PipelineStats { files_scanned: 1, files_parsed: 1, components_found: 1, conflicts_detected: 0, llm_enriched: false },
        };

        let result = merge_specs(&[spec1, spec2]).unwrap();
        assert_eq!(result.stats.conflicts_detected, 2);

        for comp in &result.components {
            let disabled = comp.props.iter().find(|p| p.canonical_name == "disabled").unwrap();
            assert_eq!(disabled.conflicts.len(), 1);
            assert!(disabled.conflicts[0].id.starts_with("cross_"));
        }
    }

    #[test]
    fn merge_no_conflict_for_matching_types() {
        let comp1 = make_component("Input", "value", AbstractPropType::ControlledValue(Box::new(AbstractPropType::Any)));
        let comp2 = make_component("Input", "value", AbstractPropType::ControlledValue(Box::new(AbstractPropType::Any)));

        let spec1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(), components: vec![comp1],
            stats: PipelineStats { files_scanned: 1, files_parsed: 1, components_found: 1, conflicts_detected: 0, llm_enriched: false },
        };
        let spec2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(), components: vec![comp2],
            stats: PipelineStats { files_scanned: 1, files_parsed: 1, components_found: 1, conflicts_detected: 0, llm_enriched: false },
        };

        let result = merge_specs(&[spec1, spec2]).unwrap();
        assert_eq!(result.stats.conflicts_detected, 0);
    }

    #[test]
    fn merge_unrelated_components_no_conflicts() {
        let comp_a = make_component("Button", "label", AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)));
        let comp_b = make_component("Modal", "open", AbstractPropType::ControlFlag);

        let spec1 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(), components: vec![comp_a],
            stats: PipelineStats { files_scanned: 1, files_parsed: 1, components_found: 1, conflicts_detected: 0, llm_enriched: false },
        };
        let spec2 = SynthesisOutput {
            ucp_version: "4.0.0".to_string(), components: vec![comp_b],
            stats: PipelineStats { files_scanned: 1, files_parsed: 1, components_found: 1, conflicts_detected: 0, llm_enriched: false },
        };

        let result = merge_specs(&[spec1, spec2]).unwrap();
        assert_eq!(result.stats.conflicts_detected, 0);
        assert_eq!(result.stats.components_found, 2);
    }
}
