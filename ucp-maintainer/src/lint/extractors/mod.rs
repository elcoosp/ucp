//! Source file extractors for different frameworks.

use crate::lint::usage::ComponentUsage;
use std::path::{Path, PathBuf};

pub trait Extractor: Send + Sync {
    fn language(&self) -> &str;
    fn extensions(&self) -> &[&str];
    fn extract_usages(&self, content: &str, file_path: &Path) -> Vec<ComponentUsage>;
}

pub struct StubExtractor;

impl Extractor for StubExtractor {
    fn language(&self) -> &str { "stub" }
    fn extensions(&self) -> &[&str] { &[] }
    fn extract_usages(&self, _content: &str, _file_path: &Path) -> Vec<ComponentUsage> { vec![] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_extractor_returns_empty() {
        assert!(StubExtractor.extract_usages("", Path::new("test.jsx")).is_empty());
    }
}
