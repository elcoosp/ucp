use std::fs;
use std::path::Path;
use serde::Serialize;
use crate::pipeline::SynthesisOutput;
use crate::extract::tokens::DtcgTokens;
use ucp_core::cam::*;
use ucp_core::Result;

/// DESIGN.md YAML front matter for AI-native design specs.
#[derive(Serialize)]
struct DesignMdFrontMatter {
    colors: Option<Vec<ColorToken>>,
    typography: Option<Vec<TypographyToken>>,
    spacing: Option<Vec<SpacingToken>>,
}

#[derive(Serialize)]
struct ColorToken {
    name: String,
    value: String,
    description: Option<String>,
}

#[derive(Serialize)]
struct TypographyToken {
    name: String,
    value: String,
    description: Option<String>,
}

#[derive(Serialize)]
struct SpacingToken {
    name: String,
    value: String,
    description: Option<String>,
}

pub fn export_design_md(
    spec: &SynthesisOutput,
    tokens: Option<&DtcgTokens>,
    library_name: &str,
    version: &str,
    output_dir: &str,
) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    let mut md = String::new();

    // YAML front matter
    let front_matter = build_front_matter(tokens);
    let yaml = serde_yaml::to_string(&front_matter)
        .unwrap_or_else(|_| String::new());
    md.push_str("---\n");
    md.push_str(&yaml);
    md.push_str("---\n\n");

    // Header
    md.push_str(&format!("# {} v{}\n\n", library_name, version));
    md.push_str("This is an AI‑native design specification for the `{}` component library. ");
    md.push_str("It describes the design tokens, component props, variants, and state machines ");
    md.push_str("in a format optimised for AI coding agents.\n\n");

    // Design tokens section
    if let Some(tok) = tokens {
        md.push_str("## Design Tokens\n\n");
        if !tok.colors.is_empty() {
            md.push_str("### Colors\n\n");
            for (name, value) in &tok.colors {
                md.push_str(&format!("- **`{}`:** `{}`\n", name, value));
            }
            md.push('\n');
        }
        if !tok.spacing.is_empty() {
            md.push_str("### Spacing\n\n");
            for (name, value) in &tok.spacing {
                md.push_str(&format!("- **`{}`:** `{}`\n", name, value));
            }
            md.push('\n');
        }
        if !tok.typography.is_empty() {
            md.push_str("### Typography\n\n");
            for (name, value) in &tok.typography {
                md.push_str(&format!("- **`{}`:** `{}`\n", name, value));
            }
            md.push('\n');
        }
    }

    // Components section
    md.push_str("## Components\n\n");
    for comp in &spec.components {
        let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        md.push_str(&format!("### {}\n\n", name));

        let desc = format!("{} component", name);
        md.push_str(&format!("_{}_\n\n", desc));

        // Props table
        if !comp.props.is_empty() {
            md.push_str("#### Props\n\n");
            md.push_str("| Name | Type | Required | Default | Description |\n");
            md.push_str("|------|------|----------|---------|-------------|\n");
            for prop in &comp.props {
                let con_type = prop.concrete_type.clone().unwrap_or_else(
                    || format!("{:?}", prop.abstract_type));
                let required = if prop.reactivity != AbstractReactivity::Static { "Yes" } else { "No" };
                let default = if prop.reactivity == AbstractReactivity::Static { "default" } else { "—" };
                md.push_str(&format!("| `{}` | `{}` | {} | {} | |\n",
                    prop.canonical_name, con_type, required, default));
            }
            md.push('\n');
        }

        // Events
        if !comp.events.is_empty() {
            md.push_str("#### Events\n\n");
            for ev in &comp.events {
                md.push_str(&format!("- **`{}`**: `{:?}`\n", ev.canonical_name, ev.abstract_payload));
            }
            md.push('\n');
        }

        // Variants
        let variants = extract_variants(comp);
        if !variants.is_empty() {
            md.push_str("#### Variants\n\n");
            for v in variants {
                md.push_str(&format!("- **`{}`**: {}\n", v.0, v.1.join(", ")));
            }
            md.push('\n');
        }

        // State machine
        if let Some(ref sm) = comp.extracted_state_machine {
            md.push_str("#### State Machine\n\n");
            md.push_str(&format!("Initial state: **`{}`**\n\n", sm.initial));
            md.push_str("| State | Transitions |\n");
            md.push_str("|-------|-------------|\n");
            for (state_name, node) in &sm.states {
                let transitions: Vec<String> = node.on.as_ref()
                    .map(|on| on.iter().map(|(ev, t)| format!("`{}` → `{}`", ev, t.target)).collect())
                    .unwrap_or_default();
                md.push_str(&format!("| `{}` | {} |\n", state_name, transitions.join(", ")));
            }
            md.push('\n');
        }

        md.push_str("---\n\n");
    }

    fs::write(dir.join("DESIGN.md"), md).map_err(ucp_core::UcpError::Io)?;
    Ok(())
}

fn build_front_matter(tokens: Option<&DtcgTokens>) -> DesignMdFrontMatter {
    match tokens {
        Some(tok) => DesignMdFrontMatter {
            colors: if tok.colors.is_empty() { None } else {
                Some(tok.colors.iter().map(|(k, v)| ColorToken {
                    name: k.clone(), value: v.clone(), description: None,
                }).collect())
            },
            typography: if tok.typography.is_empty() { None } else {
                Some(tok.typography.iter().map(|(k, v)| TypographyToken {
                    name: k.clone(), value: v.clone(), description: None,
                }).collect())
            },
            spacing: if tok.spacing.is_empty() { None } else {
                Some(tok.spacing.iter().map(|(k, v)| SpacingToken {
                    name: k.clone(), value: v.clone(), description: None,
                }).collect())
            },
        },
        None => DesignMdFrontMatter { colors: None, typography: None, spacing: None },
    }
}

fn extract_variants(comp: &CanonicalAbstractComponent) -> Vec<(String, Vec<String>)> {
    let mut variants = Vec::new();
    for p in &comp.props {
        if let Some(conc) = &p.concrete_type {
            if conc.starts_with("enum: ") {
                let values: Vec<String> = conc[6..].split(',').map(|s| s.trim().to_string()).collect();
                variants.push((p.canonical_name.clone(), values));
            }
        }
    }
    variants
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::PipelineStats;

    #[test]
    fn export_button_as_design_md() {
        let tmp = tempfile::TempDir::new().unwrap();
        let comp = CanonicalAbstractComponent {
            id: "rust:button.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "abc".into(),
                normalized_prop_names: vec!["disabled".into(), "label".into(), "variant".into()],
            },
            props: vec![
                CanonicalAbstractProp {
                    canonical_name: "disabled".into(),
                    abstract_type: AbstractPropType::ControlFlag,
                    reactivity: AbstractReactivity::Static,
                    concrete_type: Some("bool".into()),
                    sources: vec![], confidence: 1.0, conflicts: vec![],
                },
                CanonicalAbstractProp {
                    canonical_name: "label".into(),
                    abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                    reactivity: AbstractReactivity::Static,
                    concrete_type: Some("String".into()),
                    sources: vec![], confidence: 1.0, conflicts: vec![],
                },
                CanonicalAbstractProp {
                    canonical_name: "variant".into(),
                    abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                    reactivity: AbstractReactivity::Static,
                    concrete_type: Some("enum: Default, Destructive".into()),
                    sources: vec![], confidence: 1.0, conflicts: vec![],
                },
            ],
            events: vec![CanonicalAbstractEvent {
                canonical_name: "click".to_string(),
                abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
            }],
            extracted_state_machine: Some(StateMachine {
                id: "sm".into(),
                initial: "idle".into(),
                states: [("idle".into(), StateNode { on: None })].into(),
            }),
            extracted_parts: vec![],
            source_repos: vec![],
            provided_context: None,
            consumed_contexts: vec![],
        };
        let output = SynthesisOutput {
            ucp_version: "4.0.0".into(),
            components: vec![comp],
            stats: PipelineStats {
                files_scanned: 1, files_parsed: 1, components_found: 1,
                conflicts_detected: 0, llm_enriched: false,
            },
        };
        export_design_md(&output, None, "test-lib", "1.0.0", &tmp.path().to_string_lossy()).unwrap();
        let content = std::fs::read_to_string(tmp.path().join("DESIGN.md")).unwrap();
        assert!(content.contains("test-lib"));
        assert!(content.contains("Button"));
        assert!(content.contains("disabled"));
        assert!(content.contains("Default, Destructive"));
        assert!(content.contains("idle"));
    }
}
