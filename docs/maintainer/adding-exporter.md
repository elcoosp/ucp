# How‑to: Add a New Export Format

Add a new export format to UCP's export‑all pipeline.

## 1. Create the Exporter Module

Add a new file: `ucp-synthesizer/src/export/myformat.rs`

``` rust
use crate::pipeline::SynthesisOutput;
use ucp_core::Result;
use std::fs;
use std::path::Path;

pub fn export_myformat(
    spec: &SynthesisOutput,
    library_name: &str,
    version: &str,
    output_dir: &str,
) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    // Build your format's representation from the spec
    // ...

    // Write the output file(s)
    // fs::write(dir.join("output.myformat"), content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::PipelineStats;
    use ucp_core::cam::*;

    #[test]
    fn export_myformat_basic() {
        let spec = SynthesisOutput {
            ucp_version: "4.0.0".into(),
            components: vec![/* test component */],
            stats: PipelineStats::default(),
            provenance: None,
            curation_log: None,
        };
        let tmp = tempfile::TempDir::new().unwrap();
        export_myformat(&spec, "test-lib", "1.0.0", &tmp.path().to_string_lossy()).unwrap();
        // Assert output file exists and contains expected content
    }
}
```

## 2. Register in the Export Module

Add the module to `ucp-synthesizer/src/export/mod.rs`:

``` rust
pub mod myformat;
```

## 3. Add to Export‑All

In `ucp-maintainer/src/watch.rs` (the `export_all_formats` function)
and in `ucp-cli/src/main.rs` (the `cmd_export_all` function), add a
call to your exporter.

## 4. Add CLI Support

To expose the exporter as a standalone subcommand:

``` rust
// In the ExportTarget enum:
#[clap(name = "myformat")]
MyFormat,
```

And handle the new variant in `cmd_export`.

## 5. Write Integration Tests

Add a snapshot test in `ucp-synthesizer/tests/export_snapshots.rs`
or a dedicated integration test file.

## 6. Update Documentation

- Add your format to the [Export Tutorial](../../user/tutorial-export.md)
- Update this guide's list of supported formats
- Add a section to the [Command Reference](../../user/commands/export-all.md)
