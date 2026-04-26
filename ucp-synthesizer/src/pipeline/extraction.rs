use super::conflicts::detect_conflicts;
use super::enrichment::enrich_components_with_llm;
use std::collections::{BTreeMap, HashMap};

use super::output::{PipelineOptions, PipelineStats, SynthesisOutput};
use super::unification::{unify_rust_component, unify_rust_component_struct, unify_tsx_component};
use crate::extract::rust_ast::{self, RawComponentExtraction};
use crate::extract::tsx_ast::{self, RawTsxExtraction};
use crate::security::is_path_safe_to_parse;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

pub async fn run_pipeline(source_dir: &str) -> Result<SynthesisOutput> {
    run_pipeline_with_options(source_dir, &PipelineOptions::default()).await
}

pub async fn run_pipeline_with_options(
    source_dir: &str,
    opts: &PipelineOptions,
) -> Result<SynthesisOutput> {
    let mut files_scanned = 0usize;
    let mut files_parsed = 0usize;
    let mut all_components: Vec<CanonicalAbstractComponent> = Vec::new();
    let mut source_map: HashMap<String, String> = HashMap::new();

    let mut rust_extractions: BTreeMap<String, Vec<RawComponentExtraction>> = BTreeMap::new();
    let mut tsx_extractions: BTreeMap<String, Vec<RawTsxExtraction>> = BTreeMap::new();

    walk_source_dir(source_dir, |path| {
        let path_str = path.to_string_lossy().to_string();
        let ext = path.extension().and_then(|e| e.to_str());

        if !matches!(ext, Some("rs") | Some("tsx") | Some("ts") | Some("jsx")) {
            return Ok(());
        }

        if !is_path_safe_to_parse(&path_str) {
            return Ok(());
        }

        files_scanned += 1;
        let content = std::fs::read_to_string(path).map_err(ucp_core::UcpError::Io)?;
        source_map.insert(path_str.clone(), content.clone());

        match ext {
            Some("rs") => match rust_ast::extract_rust_components(&content) {
                Ok(components) if !components.is_empty() => {
                    rust_extractions.insert(path_str, components);
                    files_parsed += 1;
                }
                Ok(_) => {}
                Err(e) => eprintln!("  ⚠ Skipping {}: {}", path.display(), e),
            },
            Some("tsx") | Some("ts") | Some("jsx") => {
                match tsx_ast::extract_tsx_components(&content) {
                    Ok(components) if !components.is_empty() => {
                        tsx_extractions.insert(path_str, components);
                        files_parsed += 1;
                    }
                    Ok(_) => {}
                    Err(e) => eprintln!("  ⚠ Skipping {}: {}", path.display(), e),
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    })?;

    for (file_path, raw_components) in &rust_extractions {
        for raw in raw_components {
            let cam = if raw.is_struct_pattern {
                unify_rust_component_struct(raw, file_path)?
            } else {
                unify_rust_component(raw, file_path)?
            };
            all_components.push(cam);
        }
    }

    for (file_path, raw_components) in &tsx_extractions {
        for raw in raw_components {
            let cam = unify_tsx_component(raw, file_path)?;
            all_components.push(cam);
        }
    }

    let mut llm_enriched = false;
    if let Some(ref ollama_url) = opts.ollama_url {
        if !opts.dry_run {
            llm_enriched = enrich_components_with_llm(
                all_components.as_mut(),
                &source_map,
                ollama_url,
                &opts.llm_model,
            )
            .await?;
        } else {
            println!("   ℹ️ Dry run mode, skipping LLM enrichment");
        }
    }

    detect_conflicts(&mut all_components);

    let conflicts_detected = all_components
        .iter()
        .map(|c| c.props.iter().filter(|p| !p.conflicts.is_empty()).count())
        .sum();

    Ok(SynthesisOutput {
        ucp_version: "4.0.0".to_string(),
        components: all_components,
        stats: PipelineStats {
            files_scanned,
            files_parsed,
            components_found: rust_extractions.values().map(|v| v.len()).sum::<usize>()
                + tsx_extractions.values().map(|v| v.len()).sum::<usize>(),
            conflicts_detected,
            llm_enriched,
        },
    })
}

pub fn walk_source_dir<F>(dir: &str, mut callback: F) -> Result<()>
where
    F: FnMut(&std::path::Path) -> Result<()>,
{
    let root = Path::new(dir);
    if !root.exists() {
        return Ok(());
    }
    fn visit<F>(path: &Path, callback: &mut F, is_root: bool) -> Result<()>
    where
        F: FnMut(&std::path::Path) -> Result<()>,
    {
        if path.is_dir() {
            if !is_root {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.')
                        || ["node_modules", "target", "dist", "build", ".git"].contains(&name)
                    {
                        return Ok(());
                    }
                }
            }
            for entry in std::fs::read_dir(path)? {
                visit(&entry?.path(), callback, false)?;
            }
        } else if path.is_file() {
            callback(path)?;
        }
        Ok(())
    }
    visit(root, &mut callback, true)
}
