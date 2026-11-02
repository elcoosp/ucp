//! Merge performance baseline (0.13.0).
//!
//! Cross-framework merging is the flagship path; this keeps its cost visible
//! as normalization and conflict logic evolve.

use criterion::{criterion_group, criterion_main, Criterion};
use ucp_core::cam::{
    AbstractPropType, AbstractReactivity, CanonicalAbstractComponent, CanonicalAbstractProp,
    PropSourceMapping, SemanticFingerprint, SourceAttribution,
};
use ucp_synthesizer::merge::{merge_specs, MergeOptions};
use ucp_synthesizer::pipeline::{PipelineStats, SynthesisOutput};

fn synth_spec(n_components: usize, n_props: usize, framework: &str) -> SynthesisOutput {
    let components = (0..n_components)
        .map(|ci| CanonicalAbstractComponent {
            id: format!("{framework}:src/comp{ci}.x:Comp{ci}"),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: format!("{ci:016x}"),
                normalized_prop_names: (0..n_props).map(|pi| format!("prop{pi}")).collect(),
            },
            props: (0..n_props)
                .map(|pi| CanonicalAbstractProp {
                    canonical_name: format!("prop{pi}"),
                    abstract_type: AbstractPropType::ControlFlag,
                    reactivity: AbstractReactivity::Static,
                    concrete_type: Some("bool".into()),
                    sources: vec![PropSourceMapping {
                        repo_id: format!("src/comp{ci}.x"),
                        original_name: format!("prop{pi}"),
                        original_type: "boolean".into(),
                    }],
                    confidence: 0.9,
                    conflicts: vec![],
                })
                .collect(),
            events: vec![],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![SourceAttribution {
                repo_url: "local".into(),
                file_path: format!("src/comp{ci}.x"),
                line_start: 1,
            }],
            provided_context: None,
            consumed_contexts: vec![],
        })
        .collect();
    SynthesisOutput {
        ucp_version: ucp_core::UCP_SPEC_VERSION.into(),
        components,
        provenance: None,
        curation_log: None,
        stats: PipelineStats::default(),
    }
}

fn bench_merge(c: &mut Criterion) {
    let a = synth_spec(250, 30, "tsx");
    let b = synth_spec(250, 30, "rust");
    let both = vec![a, b];
    let mut group = c.benchmark_group("merge");
    group.throughput(criterion::Throughput::Elements(500));
    group.bench_function("2x250-components-x30-props", |bench| {
        bench.iter(|| merge_specs(&both, MergeOptions::default()).unwrap());
    });
    group.finish();
}

criterion_group!(benches, bench_merge);
criterion_main!(benches);
