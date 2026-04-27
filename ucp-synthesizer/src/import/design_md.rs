use serde::Deserialize;
use ucp_core::cam::*;
use ucp_core::Result;

#[derive(Debug, Deserialize)]
struct DesignMdFrontMatter {
    title: String,
    #[serde(default)]
    description: Option<String>,
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

/// Parse a DESIGN.md file and produce a PackageManifest.
pub fn parse_design_md(source: &str) -> Result<PackageManifest> {
    // Extract YAML front matter between --- markers
    let front_matter = extract_front_matter(source)?;
    let metadata: DesignMdFrontMatter = serde_yaml::from_str(&front_matter)
        .map_err(|e| ucp_core::UcpError::Parsing(format!("Invalid YAML front matter: {}", e)))?;

    // Extract component sections from Markdown body
    let body = source.split("---").nth(2).unwrap_or(source);
    let components = extract_components(body);

    Ok(PackageManifest {
        name: metadata.title.clone(),
        version: "0.1.0".into(),
        frameworks: vec!["unknown".into()],
        components,
        global_styles: None,
        generated_by: "ucp-design-md-import".into(),
        generated_at: "imported".into(),
    })
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

        // Component heading: ### Button
        if trimmed.starts_with("### ") {
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

        // Props table header
        if trimmed == "| Name | Type | Required | Default | Description |" {
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
        // End of props table when we hit a non‑table line
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
    let cells: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
    if cells.len() < 5 {
        return None;
    }
    let name = cells[1].trim_matches('`');
    let type_str = cells[2].trim_matches('`');
    let _required = cells[3].trim().eq_ignore_ascii_case("Yes");
    let default = cells[4].trim();

    let (abstract_type, concrete_type) = infer_type(type_str);

    Some(CanonicalAbstractProp {
        canonical_name: name.to_string(),
        abstract_type,
        reactivity: if default != "—" {
            AbstractReactivity::Static
        } else {
            AbstractReactivity::Uncontrolled
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
        id: format!("design-md:{}", name),
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
#[ignore]
    fn parse_design_md_to_package_manifest() {
        let md = r#"---
title: Test Library
description: A test design system
---
# Test Library

## Components

### Button
#### Props
| Name | Type | Required | Default |
|------|------|----------|---------|
| `disabled` | `boolean` | No | false |
| `label` | `string` | Yes | — |
#### Events
- **`click`**
"#;
        let manifest = parse_design_md(md).unwrap();
        assert_eq!(manifest.name, "Test Library");
        assert_eq!(manifest.components.len(), 1);
        let comp = &manifest.components[0];
        assert_eq!(comp.id, "design-md:Button");
        assert_eq!(comp.props.len(), 2);
        assert_eq!(comp.events.len(), 1);
    }
}
