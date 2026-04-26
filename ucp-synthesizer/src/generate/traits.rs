use super::common::to_snake_case;
use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

/// A trait for generating framework‑specific component code.
pub trait CodeGenerator {
    /// The file extension for generated source files (e.g., "rs", "tsx", "js").
    fn file_extension(&self) -> &str;

    /// Map a single CAM prop to the framework‑specific type string.
    fn map_prop_type(&self, prop: &CanonicalAbstractProp) -> String;

    /// Generate the full source code for a single component.
    fn generate_component_code(&self, comp: &CanonicalAbstractComponent) -> String;

    /// Write the project configuration files (Cargo.toml, package.json, etc.).
    fn write_project_files(&self, manifest: &PackageManifest, dir: &Path) -> Result<()>;
}

/// Shared scaffold: create output directory, iterate components, write files.
pub fn generate_with<G: CodeGenerator>(
    manifest: &PackageManifest,
    output_dir: &str,
    gen: &G,
) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir.join("src")).map_err(ucp_core::UcpError::Io)?;

    for comp in &manifest.components {
        let raw_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let file_name = to_snake_case(raw_name);
        let file_path = dir
            .join("src")
            .join(format!("{}.{}", file_name, gen.file_extension()));
        let code = gen.generate_component_code(comp);
        fs::write(&file_path, code).map_err(ucp_core::UcpError::Io)?;
    }

    gen.write_project_files(manifest, dir)
}
