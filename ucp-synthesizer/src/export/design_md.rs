use crate::extract::tokens::DtcgTokens;
use crate::pipeline::SynthesisOutput;
use std::fmt::Write as _;

use serde::Serialize;
use std::fs;
use std::path::Path;
use ucp_core::cam::{AbstractReactivity, CanonicalAbstractComponent};
use ucp_core::Result;

/// DESIGN.md YAML front matter for v1.0 spec.
#[derive(Serialize)]
struct DesignMdFrontMatter {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colors: Option<Vec<ColorToken>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    typography: Option<Vec<TypographyToken>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spacing: Option<Vec<SpacingToken>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    breakpoints: Option<Vec<BreakpointToken>>,
}

#[derive(Serialize)]
struct ColorToken {
    name: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Serialize)]
struct TypographyToken {
    name: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Serialize)]
struct SpacingToken {
    name: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Serialize)]
struct BreakpointToken {
    name: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// Write the spec back out as a DESIGN.md document.
///
/// Produces YAML front matter (title, tokens) plus one Markdown section
/// per component with props/events tables, mirroring the importer.
///
/// # Errors
///
/// Errors on file-create or file-write failures.
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

    let front_matter = DesignMdFrontMatter {
        title: library_name.to_string(),
        description: Some(format!("Design system for {library_name}")),
        colors: tokens.map(|t| {
            t.colors
                .iter()
                .map(|(k, v)| ColorToken {
                    name: k.clone(),
                    value: v.clone(),
                    description: None,
                })
                .collect()
        }),
        typography: tokens.map(|t| {
            t.typography
                .iter()
                .map(|(k, v)| TypographyToken {
                    name: k.clone(),
                    value: v.clone(),
                    description: None,
                })
                .collect()
        }),
        spacing: tokens.map(|t| {
            t.spacing
                .iter()
                .map(|(k, v)| SpacingToken {
                    name: k.clone(),
                    value: v.clone(),
                    description: None,
                })
                .collect()
        }),
        breakpoints: None,
    };

    let yaml = serde_norway::to_string(&front_matter).unwrap_or_else(|_| String::new());
    md.push_str("---\n");
    md.push_str(&yaml);
    md.push_str("---\n\n");

    // Header
    let _ = writeln!(md, "# {library_name} v{version}\n");
    md.push_str("This is an AI-native design specification for the `{}` component library. ");
    md.push_str("It describes the design tokens, component props, variants, and state machines ");
    md.push_str("in a format optimised for AI coding agents.\n\n");

    // Design tokens section
    if let Some(tok) = tokens {
        write_tokens_section(&mut md, tok);
    }

    // Components section
    md.push_str("## Components\n\n");
    for comp in &spec.components {
        write_component_section(&mut md, comp);
    }

    fs::write(dir.join("DESIGN.md"), md).map_err(ucp_core::UcpError::Io)?;
    Ok(())
}

/// Write the `## Design Tokens` section (colors, spacing, typography).
fn write_tokens_section(md: &mut String, tok: &DtcgTokens) {
    md.push_str("## Design Tokens\n\n");
    for (heading, entries) in [
        ("Colors", &tok.colors),
        ("Spacing", &tok.spacing),
        ("Typography", &tok.typography),
    ] {
        if entries.is_empty() {
            continue;
        }
        let _ = writeln!(md, "### {heading}\n");
        for (name, value) in entries {
            let _ = writeln!(md, "- **`{name}`:** `{value}`");
        }
        md.push('\n');
    }
}

/// Write the `### <Component>` section: props, events, variants, machine.
fn write_component_section(md: &mut String, comp: &CanonicalAbstractComponent) {
    let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
    let _ = writeln!(md, "### {name}\n");
    let _ = writeln!(md, "_{name} component_\n");

    // Props table
    if !comp.props.is_empty() {
        md.push_str("#### Props\n\n");
        md.push_str("| Name | Type | Required | Default | Description |\n");
        md.push_str("|------|------|----------|---------|-------------|\n");
        for prop in &comp.props {
            let con_type = prop
                .concrete_type
                .clone()
                .unwrap_or_else(|| format!("{:?}", prop.abstract_type));
            let is_static = prop.reactivity == AbstractReactivity::Static;
            let (required, default) = if is_static {
                ("No", "default")
            } else {
                ("Yes", "\u{2014}")
            };
            let _ = writeln!(
                md,
                "| `{}` | `{con_type}` | {required} | {default} | |",
                prop.canonical_name
            );
        }
        md.push('\n');
    }

    // Events
    if !comp.events.is_empty() {
        md.push_str("#### Events\n\n");
        for ev in &comp.events {
            let _ = writeln!(
                md,
                "- **`{}`**: `{:?}`",
                ev.canonical_name, ev.abstract_payload
            );
        }
        md.push('\n');
    }

    // Variants
    let variants = extract_variants(comp);
    if !variants.is_empty() {
        md.push_str("#### Variants\n\n");
        for (prop_name, values) in variants {
            let _ = writeln!(md, "- **`{prop_name}`**: {}", values.join(", "));
        }
        md.push('\n');
    }

    // State machine
    if let Some(sm) = &comp.extracted_state_machine {
        write_state_machine_section(md, sm);
    }

    md.push_str("---\n\n");
}

/// Write the `#### State Machine` subsection for one component.
fn write_state_machine_section(md: &mut String, sm: &ucp_core::cam::StateMachine) {
    md.push_str("#### State Machine\n\n");
    let _ = writeln!(md, "Initial state: **`{}`**\n", sm.initial);
    md.push_str("| State | Transitions |\n");
    md.push_str("|-------|-------------|\n");
    for (state_name, node) in &sm.states {
        let transitions: Vec<String> = node
            .on
            .as_ref()
            .map(|on| {
                on.iter()
                    .map(|(ev, t)| format!("`{ev}` \u{2192} `{}`", t.target))
                    .collect()
            })
            .unwrap_or_default();
        let _ = writeln!(md, "| `{state_name}` | {} |", transitions.join(", "));
    }
    md.push('\n');
}

fn extract_variants(comp: &CanonicalAbstractComponent) -> Vec<(String, Vec<String>)> {
    crate::utils::enum_variant_map(comp, |name, values| (name.to_string(), values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::PipelineStats;
    use ucp_core::cam::{
        AbstractPropType, CanonicalAbstractComponent, CanonicalAbstractEvent,
        CanonicalAbstractProp, SemanticFingerprint, StateMachine, StateNode,
    };

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
                CanonicalAbstractProp {
                    canonical_name: "variant".into(),
                    abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                    reactivity: AbstractReactivity::Static,
                    concrete_type: Some("enum: Default, Destructive".into()),
                    sources: vec![],
                    confidence: 1.0,
                    conflicts: vec![],
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
        export_design_md(
            &output,
            None,
            "test-lib",
            "1.0.0",
            &tmp.path().to_string_lossy(),
        )
        .unwrap();
        let content = std::fs::read_to_string(tmp.path().join("DESIGN.md")).unwrap();
        assert!(content.contains("---\n"));
        assert!(content.contains("title: test-lib"));
        assert!(content.contains("description:"));
        assert!(content.contains("Button"));
        assert!(content.contains("disabled"));
        assert!(content.contains("Default, Destructive"));
        assert!(content.contains("idle"));
    }
}
