//! File‑system watcher that continuously rebuilds specs on source changes.

use anyhow::Context;
use notify::{Event, EventKind, RecursiveMode, Watcher, Config};
use std::path::Path;
use std::sync::mpsc::channel;
use ucp_synthesizer::pipeline::{PipelineOptions, SynthesisOutput};
use ucp_synthesizer::merge::MergeOptions;

/// Run a continuous watch loop that re‑extracts, re‑merges (if a base spec is given),
/// and re‑exports whenever source files change.
pub async fn run_watch(
    source_dir: &str,
    base_spec_path: Option<&Path>,
    output_dir: &Path,
    library: &str,
    version: &str,
    debounce_ms: u64,
) -> anyhow::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            // Only trigger on meaningful file changes (create, write, remove)
            match event.kind {
                EventKind::Create(_)
                | EventKind::Modify(_)
                | EventKind::Remove(_) => {
                    let _ = tx.send(());
                }
                _ => {}
            }
        }
    })?;

    watcher.configure(Config::default().with_poll_interval(std::time::Duration::from_millis(debounce_ms)))?;
    watcher.watch(Path::new(source_dir), RecursiveMode::Recursive)?;

    println!("👀 Watching {} for changes...", source_dir);
    println!("   Output directory: {}", output_dir.display());
    if let Some(base) = base_spec_path {
        println!("   Base spec: {}", base.display());
    }
    println!("   Press Ctrl-C to stop.");

    loop {
        // Wait for a filesystem event
        rx.recv().context("Watch channel closed")?;

        // Small delay to debounce rapid changes
        std::thread::sleep(std::time::Duration::from_millis(debounce_ms));

        // Drain any additional events that arrived during the sleep
        while rx.try_recv().is_ok() {}

        println!();
        println!("🔄 Change detected – rebuilding...");

        // Run the extraction pipeline
        let opts = PipelineOptions::default();
        match ucp_synthesizer::pipeline::run_pipeline_with_options(source_dir, &opts).await {
            Ok(fresh_spec) => {
                // If a base spec is provided, do an incremental merge
                let final_spec = if let Some(base_path) = base_spec_path {
                    match SynthesisOutput::load_from_file(base_path) {
                        Ok(base_spec) => {
                            let merge_opts = MergeOptions {
                                incremental_base: Some(base_spec),
                                weights: None,
                            };
                            match ucp_synthesizer::merge::merge_specs(&[fresh_spec], merge_opts) {
                                Ok(merged) => {
                                    // Save back to the base spec path
                                    if let Err(e) = merged.save_to_file(base_path) {
                                        eprintln!("⚠️  Failed to save merged spec: {}", e);
                                        continue;
                                    }
                                    merged
                                }
                                Err(e) => {
                                    eprintln!("⚠️  Merge failed: {}", e);
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠️  Failed to load base spec: {}", e);
                            continue;
                        }
                    }
                } else {
                    // No base spec — just save the fresh extraction
                    let output_path = output_dir.join("ucp-spec.json");
                    if let Err(e) = fresh_spec.save_to_file(&output_path) {
                        eprintln!("⚠️  Failed to save spec: {}", e);
                        continue;
                    }
                    fresh_spec
                };

                // Export all formats
                if let Err(e) = export_all_formats(&final_spec, output_dir, library, version) {
                    eprintln!("⚠️  Export failed: {}", e);
                    continue;
                }

                println!("✅ Rebuild complete – {} components", final_spec.components.len());
            }
            Err(e) => {
                eprintln!("⚠️  Extraction failed: {}", e);
            }
        }
    }
}

/// Export a spec to all supported formats.
fn export_all_formats(
    spec: &SynthesisOutput,
    output_dir: &Path,
    library: &str,
    version: &str,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(output_dir)?;

    ucp_synthesizer::export::a2ui::export_a2ui(spec, library, version, &output_dir.join("a2ui").to_string_lossy())?;
    ucp_synthesizer::export::ag_ui::export_ag_ui(spec, &output_dir.join("ag-ui").to_string_lossy())?;
    ucp_synthesizer::export::w3c::export_w3c(spec, &output_dir.join("w3c").to_string_lossy())?;
    ucp_synthesizer::export::design_md::export_design_md(spec, None, library, version, &output_dir.join("design-md").to_string_lossy())?;
    ucp_synthesizer::export::llms_txt::export_llms_txt(spec, &output_dir.join("llms-txt").to_string_lossy())?;
    ucp_synthesizer::contract::ai_contract::export_ai_contract(spec, &output_dir.join("ai-contract.json").to_string_lossy())?;

    let manifest = spec.to_package_manifest(library, version, vec!["dioxus".into()]);
    ucp_synthesizer::generate::registry::generate_registry(&manifest, &output_dir.join("registry").to_string_lossy(), None, None, None)?;

    Ok(())
}
