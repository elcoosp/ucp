//! Incremental parse-cache behavior (0.13.0).
//!
//! Watch mode and CI re-runs must not re-parse unchanged files. The cache
//! keys each file by `(mtime_ns, size)` and stores its raw extractions.

use std::path::Path;

use ucp_synthesizer::pipeline::{run_pipeline_with_options, PipelineOptions};

const WIDGET: &str = "export interface WidgetProps { label: string; }\nexport const Widget = (props: WidgetProps) => <div>{props.label}</div>;\n";

fn write_source(dir: &Path, name: &str, content: &str) {
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join(name), content).unwrap();
}

async fn run(dir: &Path, cache: &Path) -> ucp_synthesizer::pipeline::SynthesisOutput {
    let opts = PipelineOptions {
        cache_path: Some(cache.to_path_buf()),
    };
    run_pipeline_with_options(&dir.join("src").to_string_lossy(), &opts)
        .await
        .unwrap()
}

/// First run populates the cache with zero hits; second identical run serves
/// every file from cache with identical results.
#[tokio::test]
async fn second_run_hits_cache_with_identical_output() {
    let tmp = tempfile::tempdir().unwrap();
    write_source(tmp.path(), "widget.tsx", WIDGET);
    let cache = tmp.path().join(".ucp-cache.json");

    let first = run(tmp.path(), &cache).await;
    assert_eq!(first.stats.cache_hits, 0, "cold run must parse everything");
    assert!(cache.exists(), "cache file must be written");

    let second = run(tmp.path(), &cache).await;
    assert_eq!(
        second.stats.cache_hits, first.stats.files_parsed,
        "warm run must serve every file from cache"
    );
    assert_eq!(
        serde_json::to_string(&first.components).unwrap(),
        serde_json::to_string(&second.components).unwrap(),
        "cached extractions must reproduce identical components"
    );
}

/// Editing a file re-parses exactly that file.
#[tokio::test]
async fn modified_file_is_reparsed_only() {
    let tmp = tempfile::tempdir().unwrap();
    write_source(tmp.path(), "widget.tsx", WIDGET);
    let cache = tmp.path().join(".ucp-cache.json");
    let _ = run(tmp.path(), &cache).await;

    // Touch a second file, then modify the first.
    write_source(tmp.path(), "other.tsx", WIDGET);
    let _ = run(tmp.path(), &cache).await; // warm the new file too

    std::thread::sleep(std::time::Duration::from_millis(20));
    write_source(tmp.path(), "widget.tsx", &WIDGET.replace("label", "title"));
    let out = run(tmp.path(), &cache).await;
    assert_eq!(
        out.stats.cache_hits, 1,
        "only the unmodified file hits cache"
    );
    let titles: Vec<String> = out
        .components
        .iter()
        .flat_map(|c| c.props.iter().map(|p| p.canonical_name.clone()))
        .collect();
    assert!(titles.contains(&"title".to_string()), "{titles:?}");
}

/// A corrupted cache file is treated as absent, never fatal.
#[tokio::test]
async fn corrupted_cache_is_ignored() {
    let tmp = tempfile::tempdir().unwrap();
    write_source(tmp.path(), "widget.tsx", WIDGET);
    let cache = tmp.path().join(".ucp-cache.json");
    std::fs::write(&cache, "not json at all {{{").unwrap();
    let out = run(tmp.path(), &cache).await;
    assert_eq!(out.stats.cache_hits, 0);
    assert_eq!(out.components.len(), 1);
}

/// Without a cache path the pipeline behaves exactly as before.
#[tokio::test]
async fn no_cache_path_parses_everything() {
    let tmp = tempfile::tempdir().unwrap();
    write_source(tmp.path(), "widget.tsx", WIDGET);
    let opts = PipelineOptions::default();
    let out = run_pipeline_with_options(&tmp.path().join("src").to_string_lossy(), &opts)
        .await
        .unwrap();
    assert_eq!(out.stats.cache_hits, 0);
    assert_eq!(out.components.len(), 1);
}
