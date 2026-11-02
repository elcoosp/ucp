use crate::pipeline::SynthesisOutput;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use ucp_core::Result;

/// Write a Markdown-formatted `llms.txt` summary of the spec.
///
/// The output lists every component with its props and events in a
/// compact, LLM-friendly form.
///
/// # Errors
///
/// Errors on file-create or file-write failures.
pub fn export_llms_txt(spec: &SynthesisOutput, output_dir: &str) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    let mut txt = String::new();
    txt.push_str("# UCP Component Library\n\n");

    for comp in &spec.components {
        let name = comp.id.rsplit(':').next().unwrap_or("");
        let _ = writeln!(txt, "## {name}\n");
        let _ = writeln!(txt, "- **Description:** {name} component");
        if !comp.props.is_empty() {
            txt.push_str("- **Props:**\n");
            for p in &comp.props {
                let t = p
                    .concrete_type
                    .clone()
                    .unwrap_or_else(|| format!("{:?}", p.abstract_type));
                let r = if p.reactivity == ucp_core::cam::AbstractReactivity::Static {
                    "optional"
                } else {
                    "required"
                };
                let _ = writeln!(txt, "  - `{}` ({t}, {r})", p.canonical_name);
            }
        }
        if !comp.events.is_empty() {
            txt.push_str("- **Events:**\n");
            for e in &comp.events {
                let _ = writeln!(txt, "  - `{}`", e.canonical_name);
            }
        }
        txt.push('\n');
    }

    fs::write(dir.join("llms.txt"), txt).map_err(ucp_core::UcpError::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::PipelineStats;
    use ucp_core::cam::*;

    #[test]
    fn llms_txt_contains_component_props() {
        let tmp = tempfile::TempDir::new().unwrap();
        let comp = CanonicalAbstractComponent {
            id: "rust:button.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "abc".into(),
                normalized_prop_names: vec!["disabled".into(), "label".into()],
            },
            props: vec![
                CanonicalAbstractProp {
                    canonical_name: "disabled".into(),
                    abstract_type: AbstractPropType::ControlFlag,
                    reactivity: AbstractReactivity::Static,
                    concrete_type: Some("bool".into()),
                    sources: vec![],
                    confidence: 1.0,
                    conflicts: vec![],
                },
                CanonicalAbstractProp {
                    canonical_name: "label".into(),
                    abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                    reactivity: AbstractReactivity::Static,
                    concrete_type: Some("String".into()),
                    sources: vec![],
                    confidence: 1.0,
                    conflicts: vec![],
                },
            ],
            events: vec![],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![],
            provided_context: None,
            consumed_contexts: vec![],
        };
        let output = SynthesisOutput {
            ucp_version: "4.0.0".into(),
            components: vec![comp],
            provenance: None,
            curation_log: None,
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                cache_hits: 0,
            },
        };
        export_llms_txt(&output, &tmp.path().to_string_lossy()).unwrap();
        let content = std::fs::read_to_string(tmp.path().join("llms.txt")).unwrap();
        assert!(content.contains("# UCP Component Library"));
        assert!(content.contains("## Button"));
        assert!(content.contains("disabled"));
        assert!(content.contains("bool"));
    }
}
