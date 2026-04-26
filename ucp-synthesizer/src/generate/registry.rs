use super::common::to_snake_case;
use super::dioxus::generate_component_code;
use serde::Serialize;
use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

// ── Full shadcn CLI 3.0+ schema types ────────────────────────────────────

#[derive(Serialize)]
struct RegistryIndex {
    #[serde(rename = "$schema")]
    schema: String,
    name: String,
    homepage: String,
    items: Vec<RegistryItem>,
}

#[derive(Serialize)]
struct RegistryItem {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    schema: Option<String>,
    name: String,
    #[serde(rename = "type")]
    item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    dependencies: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    dev_dependencies: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    #[serde(rename = "registryDependencies")]
    registry_dependencies: Vec<String>,
    files: Vec<RegistryFile>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssVars")]
    css_vars: Option<CssVars>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct CssVars {
    light: serde_json::Value,
    dark: serde_json::Value,
}

#[derive(Serialize)]
struct RegistryFile {
    path: String,
    #[serde(rename = "type")]
    file_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    content: String,
}

// ── Main generation logic ────────────────────────────────────────────────

pub fn generate_registry(
    manifest: &PackageManifest,
    output_dir: &str,
    namespace: Option<&str>,
    author: Option<&str>,
    homepage: Option<&str>,
) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    let mut items: Vec<RegistryItem> = Vec::new();

    for comp in &manifest.components {
        let raw_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let snake_name = to_snake_case(raw_name);
        let source_code = generate_component_code(comp);
        let file_path = format!("src/{}.rs", snake_name);

        let deps = resolve_dependencies(comp, &manifest.components, namespace);

        let title = Some(humanize_name(raw_name));

        let item = RegistryItem {
            schema: Some("https://ui.shadcn.com/schema/registry-item.json".to_string()),
            name: snake_name.clone(),
            item_type: "registry:ui".to_string(),
            title,
            description: None, // populated later from LLM
            author: author.map(|a| a.to_string()),
            dependencies: infer_npm_deps(manifest),
            dev_dependencies: vec![],
            registry_dependencies: deps,
            files: vec![RegistryFile {
                path: file_path,
                file_type: "registry:ui".to_string(),
                target: None,
                content: source_code,
            }],
            css_vars: None, // populated when token extraction is available
            meta: Some(serde_json::json!({
                "framework": manifest.frameworks.first().unwrap_or(&"unknown".to_string()),
                "generated_by": manifest.generated_by,
                "generated_at": manifest.generated_at,
            })),
        };

        let item_path = dir.join(format!("registry-item-{}.json", snake_name));
        let content = serde_json::to_string_pretty(&item).map_err(ucp_core::UcpError::Json)?;
        fs::write(&item_path, content).map_err(ucp_core::UcpError::Io)?;

        items.push(item);
    }

    // Write registry.json index (object, not array)
    let index = RegistryIndex {
        schema: "https://ui.shadcn.com/schema/registry.json".to_string(),
        name: namespace.unwrap_or(&manifest.name).to_string(),
        homepage: homepage
            .unwrap_or(&format!("https://github.com/{}", manifest.name))
            .to_string(),
        items,
    };

    let registry_json = serde_json::to_string_pretty(&index).map_err(ucp_core::UcpError::Json)?;
    fs::write(dir.join("registry.json"), registry_json).map_err(ucp_core::UcpError::Io)?;

    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────────

fn resolve_dependencies(
    comp: &CanonicalAbstractComponent,
    all_comps: &[CanonicalAbstractComponent],
    namespace: Option<&str>,
) -> Vec<String> {
    let mut deps: Vec<String> = Vec::new();
    let own_name = comp.id.rsplit(':').next().unwrap_or("");

    for prop in &comp.props {
        if let Some(conc) = &prop.concrete_type {
            if let Some(ref_name) = find_component_reference(conc, all_comps, own_name) {
                let dep = if let Some(ns) = namespace {
                    format!("@{}/{}", ns, ref_name)
                } else {
                    ref_name
                };
                deps.push(dep);
            }
        }
    }

    deps.sort();
    deps.dedup();
    deps
}

fn find_component_reference(
    concrete_type: &str,
    all_comps: &[CanonicalAbstractComponent],
    own_name: &str,
) -> Option<String> {
    for other in all_comps {
        let other_name = other.id.rsplit(':').next().unwrap_or("");
        if other_name.is_empty() || other_name == own_name {
            continue;
        }
        if contains_word(concrete_type, other_name) {
            return Some(to_snake_case(other_name));
        }
    }
    None
}

fn contains_word(haystack: &str, needle: &str) -> bool {
    if let Some(pos) = haystack.find(needle) {
        let after = &haystack[pos + needle.len()..];
        after.is_empty()
            || after.starts_with('>')
            || after.starts_with('<')
            || after.starts_with(',')
            || after.starts_with(' ')
            || after.starts_with(';')
    } else {
        false
    }
}

fn humanize_name(name: &str) -> String {
    let mut result = String::new();
    for c in name.chars() {
        if c == '_' || c == '-' {
            result.push(' ');
        } else if c.is_uppercase() && !result.is_empty() {
            result.push(' ');
            result.push(c);
        } else {
            result.push(c);
        }
    }
    result
}

fn infer_npm_deps(manifest: &PackageManifest) -> Vec<String> {
    let mut deps = Vec::new();
    if manifest.frameworks.iter().any(|f| f == "dioxus") {
        deps.push("dioxus@0.7".to_string());
    }
    if manifest.frameworks.iter().any(|f| f == "leptos") {
        deps.push("@leptos/core@0.7".to_string());
    }
    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_word() {
        assert!(contains_word("Option<Button>", "Button"));
        assert!(contains_word("Button", "Button"));
        assert!(contains_word("Vec<Button>", "Button"));
        assert!(!contains_word("ButtonVariant", "Button"));
    }

    #[test]
    fn test_humanize_name() {
        assert_eq!(humanize_name("button_group"), "button group");
        assert_eq!(humanize_name("DialogContent"), "Dialog Content");
    }
}
