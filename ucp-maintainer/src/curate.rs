use ucp_core::Result;
use ucp_synthesizer::pipeline::SynthesisOutput;
use ucp_core::cam::ResolutionStrategy;

/// A resolution decision for a specific conflict.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Resolution {
    pub conflict_id: String,
    pub chosen_resolution: ResolutionStrategy,
    pub custom_rationale: Option<String>,
}

/// Apply a list of resolutions to a merged spec, returning a curated spec.
pub fn curate_spec(
    merged: &SynthesisOutput,
    resolutions: &[Resolution],
) -> Result<SynthesisOutput> {
    let mut curated = merged.clone();

    // Build a lookup: conflict_id -> Resolution
    let resolution_map: std::collections::HashMap<&str, &Resolution> = resolutions
        .iter()
        .map(|r| (r.conflict_id.as_str(), r))
        .collect();

    // Walk through all components and resolve conflicts
    for comp in &mut curated.components {
        for prop in &mut comp.props {
            // Process conflicts
            let mut new_conflicts = Vec::new();
            for conflict in &prop.conflicts {
                if let Some(resolution) = resolution_map.get(conflict.id.as_str()) {
                    // Apply the chosen resolution strategy
                    match &resolution.chosen_resolution {
                        ResolutionStrategy::IncludeMajority => {
                            // Keep the prop type that appears in the majority of sources
                            // For now, just keep the existing value (already merged)
                        }
                        ResolutionStrategy::ScopeToProfile(_profile) => {
                            // Mark as scoped, keep existing
                        }
                        ResolutionStrategy::FlagForHumanReview => {
                            // Leave as-is, already flagged
                            new_conflicts.push(conflict.clone());
                            continue; // Don't remove this conflict
                        }
                    }
                    // Record this decision in the curation log
                    let decision = ucp_synthesizer::pipeline::output::CurationDecision {
                        conflict_id: conflict.id.clone(),
                        chosen_resolution: format!("{:?}", resolution.chosen_resolution),
                        rationale: resolution.custom_rationale.clone(),
                        timestamp: chrono_now(),
                    };
                    if curated.curation_log.is_none() {
                        curated.curation_log = Some(Vec::new());
                    }
                    curated.curation_log.as_mut().unwrap().push(decision);
                    // Conflict resolved – don't add it to new_conflicts
                } else {
                    // Unresolved conflict stays
                    new_conflicts.push(conflict.clone());
                }
            }
            prop.conflicts = new_conflicts;
        }
    }

    Ok(curated)
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let d = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", d)
}

/// Count unresolved conflicts in a spec.
pub fn count_unresolved_conflicts(spec: &SynthesisOutput) -> usize {
    spec.components
        .iter()
        .flat_map(|c| c.props.iter())
        .flat_map(|p| p.conflicts.iter())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucp_core::cam::*;
    use ucp_synthesizer::pipeline::PipelineStats;

    fn empty_spec() -> SynthesisOutput {
        SynthesisOutput {
            ucp_version: "4.0.0".into(),
            components: vec![],
            stats: PipelineStats {
                files_scanned: 0, files_parsed: 0, components_found: 0,
                conflicts_detected: 0, llm_enriched: false,
            },
            provenance: None,
            curation_log: None,
        }
    }

    #[test]
    fn curate_resolves_conflicts() {
        let conflict_id = "conf_001".to_string();
        let comp = CanonicalAbstractComponent {
            id: "rust:test.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "abc".into(),
                normalized_prop_names: vec!["disabled".into()],
            },
            props: vec![CanonicalAbstractProp {
                canonical_name: "disabled".into(),
                abstract_type: AbstractPropType::ControlFlag,
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("bool".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![Conflict {
                    id: conflict_id.clone(),
                    field: "props.disabled".into(),
                    present_in: vec!["src/a.rs".into()],
                    absent_in: vec!["src/b.rs".into()],
                    confidence: 0.8,
                    resolution_suggestion: ResolutionStrategy::IncludeMajority,
                }],
            }],
            events: vec![],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![],
            provided_context: None,
            consumed_contexts: vec![],
        };

        let merged = SynthesisOutput {
            components: vec![comp],
            ..empty_spec()
        };

        let resolutions = vec![Resolution {
            conflict_id: conflict_id.clone(),
            chosen_resolution: ResolutionStrategy::IncludeMajority,
            custom_rationale: None,
        }];

        let curated = curate_spec(&merged, &resolutions).unwrap();
        assert_eq!(count_unresolved_conflicts(&curated), 0);
        assert!(curated.curation_log.is_some());
        assert_eq!(curated.curation_log.unwrap().len(), 1);
    }

    #[test]
    fn unresolved_conflicts_remain() {
        let conflict_id = "conf_002".to_string();
        let comp = CanonicalAbstractComponent {
            id: "rust:test.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "abc".into(),
                normalized_prop_names: vec!["disabled".into()],
            },
            props: vec![CanonicalAbstractProp {
                canonical_name: "disabled".into(),
                abstract_type: AbstractPropType::ControlFlag,
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("bool".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![Conflict {
                    id: conflict_id.clone(),
                    field: "props.disabled".into(),
                    present_in: vec!["src/a.rs".into()],
                    absent_in: vec!["src/b.rs".into()],
                    confidence: 0.8,
                    resolution_suggestion: ResolutionStrategy::IncludeMajority,
                }],
            }],
            events: vec![],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![],
            provided_context: None,
            consumed_contexts: vec![],
        };

        let merged = SynthesisOutput {
            components: vec![comp],
            ..empty_spec()
        };

        // No resolutions applied
        let curated = curate_spec(&merged, &[]).unwrap();
        assert_eq!(count_unresolved_conflicts(&curated), 1);
        assert!(curated.curation_log.is_none());
    }
}
