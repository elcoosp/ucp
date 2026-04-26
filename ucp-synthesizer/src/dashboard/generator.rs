use crate::pipeline::SynthesisOutput;
use std::fs;
use std::path::Path;
use ucp_core::Result;

/// Generate a self-contained, interactive HTML dashboard from a UCP spec.
pub fn generate_dashboard(spec: &SynthesisOutput, output_dir: &str) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    let spec_json = serde_json::to_string_pretty(spec).map_err(ucp_core::UcpError::Json)?;

    let html = build_html(&spec_json);
    fs::write(dir.join("index.html"), html).map_err(ucp_core::UcpError::Io)?;

    Ok(())
}

fn build_html(spec_json: &str) -> String {
    let template = include_str!("template.html");
    template.replace("{spec_json}", spec_json)
}
