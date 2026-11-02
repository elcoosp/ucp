//! Incremental parse cache.
//!
//! Re-parsing every file on each pipeline run is the dominant cost for
//! watch mode and CI re-runs. The cache stores per-file raw extractions
//! keyed by `(mtime_ns, size)`: unchanged files are served from disk
//! instead of being tokenized/parsed again.
//!
//! The cache is best-effort by design: any load/parse/serialize failure
//! falls back to full parsing, never to an error.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::extract::rust_ast::RawComponentExtraction;
use crate::extract::tsx_ast::RawTsxExtraction;

/// Cached raw extractions for one source file.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    mtime_ns: u128,
    size: u64,
    rust: Vec<RawComponentExtraction>,
    tsx: Vec<RawTsxExtraction>,
}

/// On-disk cache format.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct CacheFile {
    /// Format version; mismatched versions are ignored.
    version: u32,
    entries: HashMap<String, CacheEntry>,
}

const CACHE_VERSION: u32 = 1;

/// An incremental parse cache bound to a file path.
pub struct ParseCache {
    path: PathBuf,
    entries: HashMap<String, CacheEntry>,
}

impl ParseCache {
    /// Open (or start an empty) cache at `path`.
    ///
    /// A missing, corrupted, or version-mismatched cache starts empty.
    #[must_use]
    pub fn open(path: &Path) -> Self {
        let entries = std::fs::read_to_string(path)
            .ok()
            .and_then(|c| serde_json::from_str::<CacheFile>(&c).ok())
            .filter(|f| f.version == CACHE_VERSION)
            .map(|f| f.entries)
            .unwrap_or_default();
        Self {
            path: path.to_path_buf(),
            entries,
        }
    }

    /// Cached extractions for `path` when its stat still matches.
    #[must_use]
    pub fn get(
        &self,
        path: &Path,
        mtime_ns: u128,
        size: u64,
    ) -> Option<(Vec<RawComponentExtraction>, Vec<RawTsxExtraction>)> {
        let entry = self.entries.get(&path.to_string_lossy().to_string())?;
        if entry.mtime_ns != mtime_ns || entry.size != size {
            return None;
        }
        Some((entry.rust.clone(), entry.tsx.clone()))
    }

    /// Store fresh extractions for `path`.
    pub fn insert(
        &mut self,
        path: &Path,
        mtime_ns: u128,
        size: u64,
        rust: Vec<RawComponentExtraction>,
        tsx: Vec<RawTsxExtraction>,
    ) {
        self.entries.insert(
            path.to_string_lossy().to_string(),
            CacheEntry {
                mtime_ns,
                size,
                rust,
                tsx,
            },
        );
    }

    /// Persist the cache; failures are swallowed (best-effort).
    pub fn save(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let file = CacheFile {
            version: CACHE_VERSION,
            entries: self.entries.clone(),
        };
        if let Ok(json) = serde_json::to_string(&file) {
            let _ = std::fs::write(&self.path, json);
        }
    }
}

/// Stat one file into `(mtime_ns, size)`, or `None` when unavailable.
#[must_use]
pub fn stat_file(path: &Path) -> Option<(u128, u64)> {
    let meta = std::fs::metadata(path).ok()?;
    let mtime = meta.modified().ok()?;
    let ns = mtime.duration_since(std::time::UNIX_EPOCH).ok()?.as_nanos();
    Some((ns, meta.len()))
}
