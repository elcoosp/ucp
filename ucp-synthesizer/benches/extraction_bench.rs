//! Extraction performance baseline (0.13.0).
//!
//! Tracks the cost of the tokenizer-based TSX scanner and the end-to-end
//! pipeline so regressions from cache/extractor changes stay visible.

use std::fmt::Write as _;

use criterion::{criterion_group, criterion_main, Criterion};
use ucp_synthesizer::extract::tsx_ast::extract_tsx_components;

/// Build one synthetic shadcn-style TSX file with `n_props` members.
fn synthetic_tsx(n_props: usize) -> String {
    let mut body = String::with_capacity(n_props * 48);
    for i in 0..n_props {
        match i % 5 {
            0 => writeln!(body, "  prop{i}?: string;").unwrap(),
            1 => writeln!(body, "  flag{i}?: boolean;").unwrap(),
            2 => writeln!(body, "  onEvent{i}?: (e: React.MouseEvent) => void;").unwrap(),
            3 => writeln!(body, "  items{i}?: string[];").unwrap(),
            _ => writeln!(body, "  slot{i}?: React.ReactNode;").unwrap(),
        }
    }
    format!(
        "export interface WidgetProps {{\n{body}}}\nexport const Widget = (p: WidgetProps) => <div/>;\n"
    )
}

fn bench_tsx_scan(c: &mut Criterion) {
    let small = synthetic_tsx(25);
    let big = synthetic_tsx(250);
    let mut group = c.benchmark_group("tsx-extract");
    group.throughput(criterion::Throughput::Bytes(small.len() as u64));
    group.bench_function("25-props", |b| {
        b.iter(|| extract_tsx_components(&small).unwrap());
    });
    group.throughput(criterion::Throughput::Bytes(big.len() as u64));
    group.bench_function("250-props", |b| {
        b.iter(|| extract_tsx_components(&big).unwrap());
    });
    group.finish();
}

fn bench_pipeline_end_to_end(c: &mut Criterion) {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).unwrap();
    for i in 0..100 {
        std::fs::write(src.join(format!("comp{i}.tsx")), synthetic_tsx(25)).unwrap();
    }
    let source = src.to_string_lossy().to_string();
    let out = tmp.path().join("out");
    let cache = out.join(".ucp-cache.json");
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("pipeline");
    group.bench_function("cold-100-files", |b| {
        b.iter(|| {
            let _ = std::fs::remove_file(&cache);
            rt.block_on(async {
                ucp_synthesizer::pipeline::run_pipeline_with_options(
                    &source,
                    &ucp_synthesizer::pipeline::PipelineOptions {
                        cache_path: Some(cache.clone()),
                    },
                )
                .await
                .unwrap()
            })
        });
    });
    group.bench_function("warm-100-files-cached", |b| {
        b.iter(|| {
            rt.block_on(async {
                ucp_synthesizer::pipeline::run_pipeline_with_options(
                    &source,
                    &ucp_synthesizer::pipeline::PipelineOptions {
                        cache_path: Some(cache.clone()),
                    },
                )
                .await
                .unwrap()
            })
        });
    });
    group.finish();
}

criterion_group!(benches, bench_tsx_scan, bench_pipeline_end_to_end);
criterion_main!(benches);
