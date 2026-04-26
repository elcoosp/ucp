use std::fs;
use std::path::Path;
use serde::Serialize;
use ucp_core::cam::*;
use ucp_core::Result;
use super::dioxus::generate_component_code;
use super::common::to_snake_case;

#[derive(Serialize)]
struct RegistryItem {
    name: String,
    #[serde(rename = "type")]
    item_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "registryDependencies")]
    registry_dependencies: Vec<String>,
    files: Vec<RegistryFile>,
}

#[derive(Serialize)]
struct RegistryFile {
    path: String,
    content: String,
}

/// Generate shadcn registry files from a package manifest.
pub fn generate_registry(manifest: &PackageManifest, output_dir: &str) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    let mut items: Vec<RegistryItem> = Vec::new();

    for comp in &manifest.components {
        let raw_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let snake_name = to_snake_case(raw_name);
        let source_code = generate_component_code(comp);
        let file_path = format!("src/{}.rs", snake_name);

        let deps = resolve_dependencies(comp, &manifest.components);

        let item = RegistryItem {
            name: snake_name.clone(),
            item_type: "registry:ui".to_string(),
            registry_dependencies: deps,
            files: vec![RegistryFile {
                path: file_path,
                content: source_code,
            }],
        };
        items.push(item);

        let item_path = dir.join(format!("registry-item-{}.json", snake_name));
        let content = serde_json::to_string_pretty(&item).map_err(ucp_core::UcpError::Json)?;
        fs::write(&item_path, content).map_err(ucp_core::UcpError::Io)?;
    }

    let registry_json = serde_json::to_string_pretty(&items).map_err(ucp_core::UcpError::Json)?;
    fs::write(dir.join("registry.json"), registry_json).map_err(ucp_core::UcpError::Io)?;

    Ok(())
}

fn resolve_dependencies(
    comp: &CanonicalAbstractComponent,
    all_comps: &[CanonicalAbstractComponent],
) -> Vec<String> {
    let mut deps: Vec<String> = Vec::new();
    let own_name = comp.id.rsplit(':').next().unwrap_or("");

    for prop in &comp.props {
        if let Some(conc) = &prop.concrete_type {
            if let Some(ref_name) = find_component_reference(conc, all_comps, own_name) {
                deps.push(ref_name);
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

/// Simple word-boundary check: the component name appears in the type string
/// followed by a non-alphanumeric character or end-of-string.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_word() {
        assert!(contains_word("Option<Button>", "Button"));
        assert!(contains_word("Button", "Button"));
        assert!(contains_word("Vec<Button>", "Button"));
        assert!(!contains_word("ButtonVariant", "Button")); // false positive prevention
    }
}
