//! The extraction pipeline: scan a source directory, parse every eligible
//! file in parallel, and unify all findings into CAM components.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

use super::conflicts::attach_conflicts;
use super::output::{PipelineOptions, PipelineStats, SynthesisOutput};
use super::unification::{unify_rust_component, unify_rust_component_struct, unify_tsx_component};
use crate::extract::rust_ast::{self, RawComponentExtraction};
use crate::extract::tsx_ast::{self, RawTsxExtraction};
use crate::security::is_path_safe_to_parse;
use ucp_core::cam::CanonicalAbstractComponent;
use ucp_core::Result;

/// Run the extraction pipeline with default options.
///
/// # Errors
///
/// Returns I/O errors when the source directory cannot be read.
pub async fn run_pipeline(source_dir: &str) -> Result<SynthesisOutput> {
    run_pipeline_with_options(source_dir, &PipelineOptions::default()).await
}

/// One parsed file: path, raw source text, and raw extractions.
type ParsedFile = (String, Vec<RawComponentExtraction>, Vec<RawTsxExtraction>);

/// Read and parse one file on the rayon pool; per-file errors warn only.
fn parse_one_file(path: &Path) -> ParsedFile {
    let path_str = path.to_string_lossy().to_string();
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let ext = path.extension().and_then(|e| e.to_str());
            let (rust, tsx) = match ext {
                Some("rs") => (
                    rust_ast::extract_rust_components(&content)
                        .inspect_err(|e| eprintln!("  ⚠ Skipping {}: {e}", path.display()))
                        .unwrap_or_default(),
                    Vec::new(),
                ),
                Some("tsx" | "ts" | "jsx") => (
                    Vec::new(),
                    tsx_ast::extract_tsx_components(&content)
                        .inspect_err(|e| eprintln!("  ⚠ Skipping {}: {e}", path.display()))
                        .unwrap_or_default(),
                ),
                _ => (Vec::new(), Vec::new()),
            };
            (path_str, rust, tsx)
        }
        Err(e) => {
            eprintln!("  ⚠ Skipping {}: {e}", path.display());
            (path_str, Vec::new(), Vec::new())
        }
    }
}

/// Reduce parsed files into per-path extraction maps (deterministic order).
struct ReducedFiles {
    rust_extractions: BTreeMap<String, Vec<RawComponentExtraction>>,
    tsx_extractions: BTreeMap<String, Vec<RawTsxExtraction>>,
    files_parsed: usize,
}

fn reduce_parsed_files(mut parsed: Vec<ParsedFile>) -> ReducedFiles {
    parsed.sort_by(|a, b| a.0.cmp(&b.0));

    let mut reduced = ReducedFiles {
        rust_extractions: BTreeMap::new(),
        tsx_extractions: BTreeMap::new(),
        files_parsed: 0,
    };

    for (path_str, rust, tsx) in parsed {
        let parsed_file = !rust.is_empty() || !tsx.is_empty();
        if parsed_file {
            reduced.files_parsed += 1;
        }
        if !rust.is_empty() {
            reduced.rust_extractions.insert(path_str.clone(), rust);
        }
        if !tsx.is_empty() {
            reduced.tsx_extractions.insert(path_str, tsx);
        }
    }
    reduced
}

/// Run the extraction pipeline with explicit options.
///
/// The directory walk is sequential (cheap), but reading + AST-parsing of
/// all eligible files happens in parallel on the rayon pool, which is the
/// CPU-bound part.
///
/// # Errors
///
/// Returns I/O errors when the source directory cannot be read.
pub async fn run_pipeline_with_options(
    source_dir: &str,
    opts: &PipelineOptions,
) -> Result<SynthesisOutput> {
    // 1. Walk: collect eligible files (sequential, cheap directory I/O).
    let mut eligible: Vec<PathBuf> = Vec::new();
    walk_source_dir(source_dir, |path| {
        if is_eligible(path) && is_path_safe_to_parse(&path.to_string_lossy()) {
            eligible.push(path.to_path_buf());
        }
        Ok(())
    })?;

    // 2. Parse: files whose (mtime, size) still match the incremental cache
    //    are served from disk; only the rest hit the parallel parser.
    let mut cache = opts
        .cache_path
        .as_deref()
        .map(super::cache::ParseCache::open);
    let mut cache_hits = 0usize;
    let mut to_parse: Vec<PathBuf> = Vec::new();
    let mut served: Vec<ParsedFile> = Vec::new();
    for path in &eligible {
        let cached = cache.as_mut().and_then(|c| {
            super::cache::stat_file(path).and_then(|(mtime, size)| {
                c.get(path, mtime, size)
                    .map(|(rust, tsx)| (c, mtime, size, rust, tsx))
            })
        });
        match cached {
            Some((_, _, _, rust, tsx)) => {
                cache_hits += 1;
                served.push((path.to_string_lossy().to_string(), rust, tsx));
            }
            None => to_parse.push(path.clone()),
        }
    }
    let parsed: Vec<ParsedFile> = to_parse
        .par_iter()
        .map(|p| parse_one_file(p.as_path()))
        .collect();
    if let Some(c) = cache.as_mut() {
        for (path_str, rust, tsx) in &parsed {
            if let Some((mtime, size)) = super::cache::stat_file(Path::new(path_str)) {
                c.insert(Path::new(path_str), mtime, size, rust.clone(), tsx.clone());
            }
        }
        c.save();
    }

    let mut all_parsed = served;
    all_parsed.extend(parsed);
    let reduced = reduce_parsed_files(all_parsed);
    let files_parsed = reduced.files_parsed;
    let files_scanned = eligible.len();

    // 4. Unify raw extractions into CAM components.
    let mut all_components: Vec<CanonicalAbstractComponent> = Vec::new();
    for (file_path, raw_components) in &reduced.rust_extractions {
        for raw in raw_components {
            let cam = if raw.is_struct_pattern {
                unify_rust_component_struct(raw, file_path)?
            } else {
                unify_rust_component(raw, file_path)?
            };
            all_components.push(cam);
        }
    }
    for (file_path, raw_components) in &reduced.tsx_extractions {
        for raw in raw_components {
            all_components.push(unify_tsx_component(raw, file_path)?);
        }
    }

    // 5. Conflict detection + deterministic ordering.
    attach_conflicts(&mut all_components);
    all_components.sort_by(|a, b| a.id.cmp(&b.id));
    let conflicts_detected = super::conflicts::count_conflicts(&all_components);

    let components_found = reduced
        .rust_extractions
        .values()
        .map(Vec::len)
        .sum::<usize>()
        + reduced
            .tsx_extractions
            .values()
            .map(Vec::len)
            .sum::<usize>();

    Ok(SynthesisOutput {
        ucp_version: ucp_core::UCP_SPEC_VERSION.to_string(),
        components: all_components,
        provenance: None,
        curation_log: None,
        stats: PipelineStats {
            files_scanned,
            files_parsed,
            components_found,
            conflicts_detected,
            cache_hits,
        },
    })
}

/// Extension of a path as `&str`, if present.
fn ups_ext(path: &Path) -> Option<&str> {
    path.extension().and_then(|e| e.to_str())
}

/// True when the file extension is one of the handled source types.
fn is_eligible(path: &Path) -> bool {
    matches!(ups_ext(path), Some("rs" | "tsx" | "ts" | "jsx"))
}

/// Recursively visit all files under `dir`, skipping common ignore
/// directories (hidden dirs, `node_modules`, `target`, `dist`, `build`).
///
/// # Errors
///
/// Returns I/O errors when a directory cannot be listed.
pub fn walk_source_dir<F>(dir: &str, mut callback: F) -> Result<()>
where
    F: FnMut(&Path) -> Result<()>,
{
    let root = Path::new(dir);
    if !root.exists() {
        return Ok(());
    }
    visit(root, &mut callback, true)
}

fn visit<F>(path: &Path, callback: &mut F, is_root: bool) -> Result<()>
where
    F: FnMut(&Path) -> Result<()>,
{
    if path.is_dir() {
        if !is_root && is_ignored_dir(path) {
            return Ok(());
        }
        for entry in std::fs::read_dir(path)? {
            visit(&entry?.path(), callback, false)?;
        }
    } else if path.is_file() {
        callback(path)?;
    }
    Ok(())
}

/// Directory names that are always skipped during extraction walks.
const IGNORED_DIRS: [&str; 5] = ["node_modules", "target", "dist", "build", ".git"];

fn is_ignored_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| name.starts_with('.') || IGNORED_DIRS.contains(&name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_visits_only_relevant_files() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src/nested")).unwrap();
        std::fs::create_dir_all(tmp.path().join("node_modules")).unwrap();
        std::fs::create_dir_all(tmp.path().join(".hidden")).unwrap();
        std::fs::write(tmp.path().join("src/a.rs"), "pub fn a() {}").unwrap();
        std::fs::write(tmp.path().join("src/nested/b.tsx"), "").unwrap();
        std::fs::write(tmp.path().join("node_modules/c.rs"), "").unwrap();
        std::fs::write(tmp.path().join(".hidden/d.rs"), "").unwrap();
        std::fs::write(tmp.path().join("e.md"), "").unwrap();

        let mut files = Vec::new();
        walk_source_dir(tmp.path().to_str().unwrap(), |p| {
            files.push(p.to_string_lossy().to_string());
            Ok(())
        })
        .unwrap();
        files.sort();
        // The walk visits every file; ignored dirs are skipped, but
        // extension filtering happens separately in the pipeline.
        assert_eq!(files.len(), 3);
        assert!(files.iter().any(|f| f.ends_with("src/a.rs")));
        assert!(files.iter().any(|f| f.ends_with("src/nested/b.tsx")));
        assert!(files.iter().any(|f| f.ends_with("e.md")));
    }

    #[test]
    fn walk_nonexistent_dir_is_noop() {
        let mut calls = 0;
        walk_source_dir("/definitely/not/here", |_| {
            calls += 1;
            Ok(())
        })
        .unwrap();
        assert_eq!(calls, 0);
    }

    #[test]
    fn eligibility_follows_extension() {
        assert!(is_eligible(Path::new("a.rs")));
        assert!(is_eligible(Path::new("a.tsx")));
        assert!(is_eligible(Path::new("a.ts")));
        assert!(is_eligible(Path::new("a.jsx")));
        assert!(!is_eligible(Path::new("a.md")));
        assert!(!is_eligible(Path::new("a")));
    }
}
