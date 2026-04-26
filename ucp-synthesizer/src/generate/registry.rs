use super::common::to_snake_case;
use super::dioxus::generate_component_code;
use serde::Serialize;
use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

// ── Full shadcn CLI v4 schema types ─────────────────────────────────────

#[derive(Serialize)]
struct RegistryIndex {
    #[serde(rename = "$schema")]
    schema: String,
    name: String,
    homepage: String,
    items: Vec<RegistryItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preset: Option<Preset>,
}

#[derive(Serialize)]
struct Preset {
    name: String,
    colors: serde_json::Value,
    theme: String,
    icons: String,
    fonts: Vec<String>,
    radius: String,
}

#[derive(Serialize)]
struct RegistryItem {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    schema: Option<String>,
    name: String,
    #[serde(rename = "type")]
    item_type: String, // "registry:ui", "registry:base", "registry:font"
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

// ── Generation functions ────────────────────────────────────────────────

/// Generate individual registry items (v0.4.0 style, still works for v4).
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
            description: None,
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
            css_vars: None,
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

    let index = RegistryIndex {
        schema: "https://ui.shadcn.com/schema/registry.json".to_string(),
        name: namespace.unwrap_or(&manifest.name).to_string(),
        homepage: homepage
            .unwrap_or(&format!("https://github.com/{}", manifest.name))
            .to_string(),
        items,
        preset: None,
    };

    let registry_json = serde_json::to_string_pretty(&index).map_err(ucp_core::UcpError::Json)?;
    fs::write(dir.join("registry.json"), registry_json).map_err(ucp_core::UcpError::Io)?;

    Ok(())
}

/// Generate a `registry:base` item that bundles all components + global assets.
pub fn generate_registry_base(
    manifest: &PackageManifest,
    _tokens: Option<&serde_json::Value>, // DTCG tokens placeholder
    output_dir: &str,
    namespace: Option<&str>,
    author: Option<&str>,
    homepage: Option<&str>,
) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    let mut all_files = Vec::new();
    let mut all_deps: Vec<String> = Vec::new();

    for comp in &manifest.components {
        let raw_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let snake_name = to_snake_case(raw_name);
        let source_code = generate_component_code(comp);
        let file_path = format!("src/{}.rs", snake_name);
        let deps = resolve_dependencies(comp, &manifest.components, namespace);
        all_deps.extend(deps);

        all_files.push(RegistryFile {
            path: file_path,
            file_type: "registry:ui".to_string(),
            target: None,
            content: source_code,
        });
    }

    // Add a placeholder global CSS file
    all_files.push(RegistryFile {
        path: "src/globals.css".to_string(),
        file_type: "registry:style".to_string(),
        target: None,
        content: "/* Base design system styles */\n".to_string(),
    });

    all_deps.sort();
    all_deps.dedup();

    let item = RegistryItem {
        schema: Some("https://ui.shadcn.com/schema/registry-item.json".to_string()),
        name: "base".to_string(),
        item_type: "registry:base".to_string(),
        title: Some(format!("{} Base Design System", manifest.name)),
        description: Some("Complete design system including components and styles.".to_string()),
        author: author.map(|a| a.to_string()),
        dependencies: infer_npm_deps(manifest),
        dev_dependencies: vec![],
        registry_dependencies: all_deps,
        files: all_files,
        css_vars: None, // populated when DTCG tokens are available
        meta: Some(serde_json::json!({
            "framework": manifest.frameworks.first().unwrap_or(&"unknown".to_string()),
            "generated_by": manifest.generated_by,
            "generated_at": manifest.generated_at,
        })),
    };

    // Write the base item
    let item_path = dir.join("registry-item-base.json");
    let content = serde_json::to_string_pretty(&item).map_err(ucp_core::UcpError::Json)?;
    fs::write(&item_path, content).map_err(ucp_core::UcpError::Io)?;

    // Write registry.json index with the base item
    let index = RegistryIndex {
        schema: "https://ui.shadcn.com/schema/registry.json".to_string(),
        name: namespace.unwrap_or(&manifest.name).to_string(),
        homepage: homepage
            .unwrap_or(&format!("https://github.com/{}", manifest.name))
            .to_string(),
        items: vec![item],
        preset: None, // populated when tokens are available
    };

    let registry_json = serde_json::to_string_pretty(&index).map_err(ucp_core::UcpError::Json)?;
    fs::write(dir.join("registry.json"), registry_json).map_err(ucp_core::UcpError::Io)?;

    Ok(())
}

// ── Helpers (unchanged from v0.4.0) ─────────────────────────────────────

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

    fn make_test_manifest() -> PackageManifest {
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
        PackageManifest {
            name: "test-registry".into(),
            version: "0.1.0".into(),
            frameworks: vec!["dioxus".into()],
            components: vec![comp],
            global_styles: None,
            generated_by: "test".into(),
            generated_at: "now".into(),
        }
    }

    #[test]
    fn registry_index_omits_preset_when_none() {
        let tmp = tempfile::TempDir::new().unwrap();
        let manifest = make_test_manifest();
        generate_registry(&manifest, &tmp.path().to_string_lossy(), None, None, None).unwrap();
        let content = std::fs::read_to_string(tmp.path().join("registry.json")).unwrap();
        // preset should be absent since we pass None
        assert!(!content.contains("\"preset\""));
    }

    #[test]
    fn generate_registry_base_produces_single_item() {
        let tmp = tempfile::TempDir::new().unwrap();
        let manifest = make_test_manifest();
        generate_registry_base(
            &manifest,
            None,
            &tmp.path().to_string_lossy(),
            None,
            None,
            None,
        )
        .unwrap();

        let content = std::fs::read_to_string(tmp.path().join("registry.json")).unwrap();
        assert!(content.contains("\"registry:base\""));
        assert!(content.contains("\"base\""));
    }
}
