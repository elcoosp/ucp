use std::collections::{HashMap, HashSet};
use ucp_core::cam::*;

pub fn detect_conflicts(components: &mut [CanonicalAbstractComponent]) {
    let mut hash_groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, comp) in components.iter().enumerate() {
        hash_groups
            .entry(comp.semantic_fingerprint.purpose_hash.clone())
            .or_default()
            .push(idx);
    }
    let mut conflict_id_counter = 0u32;
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
            let conflict_id = format!("conf_{:03}", conflict_id_counter);
            let has_count = member_indices.len();
            let member_set: HashSet<usize> = member_indices.iter().copied().collect();
            let absent_in: Vec<String> = (0..components.len())
                .filter(|i| !member_set.contains(i))
                .map(|idx| {
                    components[idx]
                        .source_repos
                        .first()
                        .map(|s| s.file_path.clone())
                        .unwrap_or_else(|| "unknown".to_string())
                })
                .filter(|s| !present_in.contains(s))
                .collect();
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
                        absent_in: absent_in.clone(),
                        confidence,
                        resolution_suggestion: resolution.clone(),
                    });
                }
            }
        }
    }
}
