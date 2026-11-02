//! DESIGN.md import: YAML front matter + Markdown component tables ->
//! [`PackageManifest`].
//!
//! Design tokens declared in the front matter (colors, typography,
//! spacing) are converted into CSS custom properties and stored in the
//! manifest's `global_styles`, so a single DESIGN.md file round-trips into
//! both component specs *and* usable token output.

use serde::Deserialize;
use ucp_core::cam::{
    AbstractPropType, AbstractReactivity, CanonicalAbstractComponent, CanonicalAbstractEvent,
    CanonicalAbstractProp, PackageManifest, SemanticFingerprint,
};
use ucp_core::Result;

#[derive(Debug, Deserialize)]
struct DesignMdFrontMatter {
    title: String,
    #[serde(default)]
    colors: Option<Vec<ColorToken>>,
    #[serde(default)]
    typography: Option<Vec<TypographyToken>>,
    #[serde(default)]
    spacing: Option<Vec<SpacingToken>>,
}

#[derive(Debug, Deserialize)]
struct ColorToken {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct TypographyToken {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct SpacingToken {
    name: String,
    value: String,
}

/// Parse a DESIGN.md file and produce a [`PackageManifest`].
///
/// # Errors
///
/// Errors when the document has no YAML front matter block or when the
/// front matter is not valid YAML with a `title` field.
pub fn parse_design_md(source: &str) -> Result<PackageManifest> {
    // Extract YAML front matter between --- markers
    let front_matter = extract_front_matter(source)?;
    let metadata: DesignMdFrontMatter = serde_norway::from_str(&front_matter)
        .map_err(|e| ucp_core::UcpError::Parsing(format!("Invalid YAML front matter: {e}")))?;

    // Extract component sections from Markdown body. Use  so that
    //  sequences inside table separators () do not split the
    // body; the front matter has exactly one opening and one closing fence.
    let body = source.splitn(3, "---").nth(2).unwrap_or(source);
    let components = extract_components(body);

    let global_styles = build_global_styles(
        metadata.colors.as_deref(),
        metadata.typography.as_deref(),
        metadata.spacing.as_deref(),
    );

    Ok(PackageManifest {
        name: metadata.title.clone(),
        version: "0.1.0".into(),
        frameworks: vec!["unknown".into()],
        components,
        global_styles,
        generated_by: "ucp-design-md-import".into(),
        generated_at: "imported".into(),
    })
}

/// Render front-matter tokens as a `:root { --token: value; }` CSS block.
///
/// Returns `None` when no tokens are declared. Names are kebab-cased and
/// prefixed by token kind (`--color-*`, `--font-*`, `--space-*`) following
/// common design-token conventions.
fn build_global_styles(
    colors: Option<&[ColorToken]>,
    typography: Option<&[TypographyToken]>,
    spacing: Option<&[SpacingToken]>,
) -> Option<String> {
    let mut declarations = Vec::new();
    for token in colors.into_iter().flatten() {
        declarations.push(format!(
            "--color-{}: {};",
            kebab_case(&token.name),
            token.value
        ));
    }
    for token in typography.into_iter().flatten() {
        declarations.push(format!(
            "--font-{}: {};",
            kebab_case(&token.name),
            token.value
        ));
    }
    for token in spacing.into_iter().flatten() {
        declarations.push(format!(
            "--space-{}: {};",
            kebab_case(&token.name),
            token.value
        ));
    }
    if declarations.is_empty() {
        return None;
    }
    Some(format!(":root {{\n  {}\n}}", declarations.join("\n  ")))
}

/// Convert a token name to kebab-case: `Primary Blue` / `primary_blue` /
/// `primaryBlue` all become `primary-blue`.
fn kebab_case(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut prev_lower_or_digit = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && prev_lower_or_digit {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
            prev_lower_or_digit = !ch.is_ascii_uppercase();
        } else if ch == '_' || ch == '-' || ch == ' ' {
            if !out.ends_with('-') && !out.is_empty() {
                out.push('-');
            }
            prev_lower_or_digit = false;
        }
    }
    out.trim_matches('-').to_string()
}

fn extract_front_matter(source: &str) -> Result<String> {
    let parts: Vec<&str> = source.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(ucp_core::UcpError::Parsing(
            "No YAML front matter found".into(),
        ));
    }
    Ok(parts[1].trim().to_string())
}

fn extract_components(body: &str) -> Vec<CanonicalAbstractComponent> {
    let mut components = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_props: Vec<CanonicalAbstractProp> = Vec::new();
    let mut in_props_table = false;
    let mut in_events_section = false;
    let mut current_events: Vec<CanonicalAbstractEvent> = Vec::new();

    for line in body.lines() {
        let trimmed = line.trim();

        // Component heading: exactly `### Button` (not `#### Props`).
        if trimmed.starts_with("### ") && !trimmed.starts_with("#### ") {
            if let Some(name) = current_name.take() {
                components.push(build_component(&name, &current_props, &current_events));
                current_props.clear();
                current_events.clear();
            }
            current_name = Some(trimmed.strip_prefix("### ").unwrap_or("").to_string());
            in_props_table = false;
            in_events_section = false;
            continue;
        }

        // Props table header: the Description column is optional, so match
        // on the leading cells rather than the exact full header.
        if trimmed.starts_with("| Name | Type |") {
            in_props_table = true;
            in_events_section = false;
            continue;
        }
        // Skip table separator
        if trimmed.starts_with("|---") || trimmed.is_empty() {
            continue;
        }

        // Props table row
        if in_props_table && trimmed.starts_with('|') {
            if let Some(prop) = parse_prop_row(trimmed) {
                current_props.push(prop);
            }
            continue;
        }
        // End of props table when we hit a non-table line
        if in_props_table && !trimmed.starts_with('|') {
            in_props_table = false;
        }

        // Events section
        if trimmed == "#### Events" {
            in_events_section = true;
            continue;
        }
        if in_events_section && trimmed.starts_with("- **`") {
            if let Some(event) = parse_event_line(trimmed) {
                current_events.push(event);
            }
            continue;
        }
        // End of events when we hit next section
        if in_events_section && !trimmed.starts_with("- **`") {
            in_events_section = false;
        }
    }

    // Last component
    if let Some(name) = current_name {
        components.push(build_component(&name, &current_props, &current_events));
    }

    components
}

fn parse_prop_row(line: &str) -> Option<CanonicalAbstractProp> {
    let cells: Vec<&str> = line.split('|').map(str::trim).collect();
    if cells.len() < 4 {
        return None;
    }
    let name = cells[1].trim_matches('`');
    if name.is_empty() || name == "Name" {
        return None;
    }
    let type_str = cells[2].trim_matches('`');
    let default = cells[4].trim();

    let (abstract_type, concrete_type) = infer_type(type_str);

    Some(CanonicalAbstractProp {
        canonical_name: name.to_string(),
        abstract_type,
        reactivity: if default == "—" || default.is_empty() {
            AbstractReactivity::Uncontrolled
        } else {
            AbstractReactivity::Static
        },
        concrete_type,
        sources: vec![],
        confidence: 0.8,
        conflicts: vec![],
    })
}

fn parse_event_line(line: &str) -> Option<CanonicalAbstractEvent> {
    let name = line.strip_prefix("- **`")?.split('`').next()?;
    Some(CanonicalAbstractEvent {
        canonical_name: name.to_string(),
        abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
    })
}

fn infer_type(type_str: &str) -> (AbstractPropType, Option<String>) {
    match type_str {
        "boolean" | "bool" => (AbstractPropType::ControlFlag, Some("bool".to_string())),
        "string" | "String" => (
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            Some("String".to_string()),
        ),
        "number" => (
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            Some("f64".to_string()),
        ),
        _ if type_str.contains("=>") || type_str.contains("void") => (
            AbstractPropType::AsyncEventHandler(vec![]),
            Some(type_str.to_string()),
        ),
        _ => (
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            Some(type_str.to_string()),
        ),
    }
}

fn build_component(
    name: &str,
    props: &[CanonicalAbstractProp],
    events: &[CanonicalAbstractEvent],
) -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: format!("design-md:{name}"),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: format!("{:016x}", name.len()),
            normalized_prop_names: props.iter().map(|p| p.canonical_name.clone()).collect(),
        },
        props: props.to_vec(),
        events: events.to_vec(),
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_design_md_to_package_manifest() {
        let md = r"---
title: Test Library
description: A test design system
---
# Test Library

## Components

### Button
#### Props
| Name | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `disabled` | `boolean` | No | false | Disables the button |
| `label` | `string` | Yes | — | Visible label |
#### Events
- **`click`**
";
        let manifest = parse_design_md(md).unwrap();
        assert_eq!(manifest.name, "Test Library");
        assert_eq!(manifest.components.len(), 1);
        let comp = &manifest.components[0];
        assert_eq!(comp.id, "design-md:Button");
        assert_eq!(comp.props.len(), 2);
        assert_eq!(comp.events.len(), 1);
    }

    #[test]
    fn four_column_prop_table_is_supported() {
        let md = "---\ntitle: T\n---\n### Btn\n#### Props\n| Name | Type | Required | Default |\n|---|---|---|---|\n| `on` | `boolean` | No | true |\n";
        let manifest = parse_design_md(md).unwrap();
        assert_eq!(manifest.components.len(), 1);
        assert_eq!(manifest.components[0].props.len(), 1);
        assert_eq!(manifest.components[0].props[0].canonical_name, "on");
    }

    #[test]
    fn missing_front_matter_is_an_error() {
        assert!(parse_design_md("# no front matter").is_err());
    }

    #[test]
    fn tokens_become_css_custom_properties() {
        let md = "---\ntitle: T\ncolors:\n  - name: Primary Blue\n    value: \"#0070f3\"\ntypography:\n  - name: sans\n    value: Inter, sans-serif\nspacing:\n  - name: md\n    value: 12px\n---\n### X\n#### Props\n| Name | Type | Required | Default |\n|---|---|---|---|\n";
        let manifest = parse_design_md(md).unwrap();
        let css = manifest.global_styles.expect("tokens present");
        assert!(css.contains("--color-primary-blue: #0070f3;"), "{css}");
        assert!(css.contains("--font-sans: Inter, sans-serif;"), "{css}");
        assert!(css.contains("--space-md: 12px;"), "{css}");
        assert!(css.starts_with(":root {"));
    }

    #[test]
    fn no_tokens_means_no_global_styles() {
        let md = "---\ntitle: T\n---\n";
        assert!(parse_design_md(md).unwrap().global_styles.is_none());
    }

    #[test]
    fn kebab_case_handles_common_styles() {
        assert_eq!(kebab_case("Primary Blue"), "primary-blue");
        assert_eq!(kebab_case("primary_blue"), "primary-blue");
        assert_eq!(kebab_case("primaryBlue"), "primary-blue");
        assert_eq!(kebab_case("  primaryBlue  "), "primary-blue");
        assert_eq!(kebab_case(""), "");
    }
}
