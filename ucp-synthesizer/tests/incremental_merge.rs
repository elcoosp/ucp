use ucp_core::cam::*;
use ucp_synthesizer::merge::{merge_specs, MergeOptions};
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

fn make_component(name: &str, prop_name: &str, prop_type: AbstractPropType) -> CanonicalAbstractComponent {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut hasher);
    let purpose_hash = format!("{:016x}", hasher.finish());
    CanonicalAbstractComponent {
        id: format!("rust:test.rs:{}", name),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash,
            normalized_prop_names: vec![prop_name.to_string()],
        },
        props: vec![CanonicalAbstractProp {
            canonical_name: prop_name.to_string(),
            abstract_type: prop_type,
            reactivity: AbstractReactivity::Static,
            concrete_type: None,
            sources: vec![], confidence: 1.0, conflicts: vec![],
        }],
        events: vec![], extracted_state_machine: None, extracted_parts: vec![],
        source_repos: vec![], provided_context: None, consumed_contexts: vec![],
    }
}

fn empty_spec() -> SynthesisOutput {
    SynthesisOutput {
        ucp_version: "4.0.0".into(), components: vec![],
        stats: PipelineStats {
            files_scanned: 0, files_parsed: 0, components_found: 0,
            conflicts_detected: 0, llm_enriched: false,
        },
        provenance: None,
        curation_log: None,
    }
}

#[test]
fn incremental_merge_adds_new_component() {
    let comp_a = make_component("Button", "disabled", AbstractPropType::ControlFlag);
    let comp_b = make_component("Dialog", "open", AbstractPropType::ControlFlag);
    let spec1 = SynthesisOutput { components: vec![comp_a.clone()], ..empty_spec() };
    let spec2 = SynthesisOutput { components: vec![comp_b.clone()], ..empty_spec() };

    let merged_first = merge_specs(&[spec1.clone(), spec2.clone()], MergeOptions::default()).unwrap();
    assert_eq!(merged_first.components.len(), 2);

    let comp_c = make_component("Accordion", "multiple", AbstractPropType::ControlFlag);
    let spec3 = SynthesisOutput { components: vec![comp_c.clone()], ..empty_spec() };

    let options = MergeOptions {
        incremental_base: Some(merged_first),
        weights: None,
    };
    let merged_second = merge_specs(&[spec3], options).unwrap();
    assert_eq!(merged_second.components.len(), 3, "Should have original 2 + new component C");
}

#[test]
fn incremental_merge_preserves_existing() {
    let comp_a = make_component("Button", "disabled", AbstractPropType::ControlFlag);
    let spec1 = SynthesisOutput { components: vec![comp_a.clone()], ..empty_spec() };
    let merged_base = merge_specs(&[spec1], MergeOptions::default()).unwrap();
    let options = MergeOptions { incremental_base: Some(merged_base), weights: None };
    let merged_again = merge_specs(&[], options).unwrap();
    assert_eq!(merged_again.components.len(), 1);
    assert!(
    merged_again.components[0].id.contains("Button"),
    "Expected component id to contain Button, got {}",
    merged_again.components[0].id
);
}

#[test]
fn weighted_merge_prefers_higher_weight() {
    let comp_a = make_component("Button", "disabled", AbstractPropType::ControlFlag);
    let mut comp_b = make_component("Button", "disabled", AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)));
    comp_b.semantic_fingerprint.purpose_hash = comp_a.semantic_fingerprint.purpose_hash.clone();
    let spec1 = SynthesisOutput { components: vec![comp_a], ..empty_spec() };
    let spec2 = SynthesisOutput { components: vec![comp_b], ..empty_spec() };

    let mut weights = HashMap::new();
    weights.insert("spec1".to_string(), 10.0);
    weights.insert("spec2".to_string(), 1.0);

    let merged = merge_specs(&[spec1, spec2], MergeOptions {
        incremental_base: None,
        weights: Some(weights),
    }).unwrap();

    let resolved_prop = merged.components[0].props.iter().find(|p| p.canonical_name == "disabled").unwrap();
    assert_eq!(resolved_prop.abstract_type, AbstractPropType::ControlFlag,
               "Higher weight spec1 should win");
}
