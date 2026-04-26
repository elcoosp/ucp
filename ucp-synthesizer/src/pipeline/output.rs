use ucp_core::cam::*;
use ucp_core::Result;

pub const BASE_CONFIDENCE_RUST: f32 = 0.95;
pub const BASE_CONFIDENCE_TSX: f32 = 0.90;
pub const ANY_PENALTY_PER_PROP: f32 = 0.08;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[must_use = "discarding synthesis output loses all extracted component data"]
pub struct SynthesisOutput {
    pub ucp_version: String,
    pub components: Vec<CanonicalAbstractComponent>,
    pub stats: PipelineStats,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PipelineStats {
    pub files_scanned: usize,
    pub files_parsed: usize,
    pub components_found: usize,
    pub conflicts_detected: usize,
    pub llm_enriched: bool,
}

#[derive(Debug, Clone)]
pub struct PipelineOptions {
    pub ollama_url: Option<String>,
    pub llm_model: String,
    pub dry_run: bool,
}

impl Default for PipelineOptions {
    fn default() -> Self {
        Self {
            ollama_url: None,
            llm_model: "glm-5:cloud".to_string(),
            dry_run: false,
        }
    }
}

impl SynthesisOutput {
    pub fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(ucp_core::UcpError::Io)?;
        serde_json::from_str(&content).map_err(ucp_core::UcpError::Json)
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(ucp_core::UcpError::Json)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(ucp_core::UcpError::Io)?;
        }
        std::fs::write(path, json).map_err(ucp_core::UcpError::Io)?;
        Ok(())
    }

    pub fn to_package_manifest(
        &self,
        name: &str,
        version: &str,
        frameworks: Vec<String>,
    ) -> PackageManifest {
        PackageManifest {
            name: name.to_string(),
            version: version.to_string(),
            frameworks,
            components: self.components.clone(),
            global_styles: None,
            generated_by: format!("ucp v{}", self.ucp_version),
            generated_at: "generated_at_placeholder".to_string(),
        }
    }
}
