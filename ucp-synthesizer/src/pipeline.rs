use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

use crate::extract::rust_ast::{self, RawComponentExtraction};
use crate::extract::tsx_ast::{self, RawTsxExtraction};
use crate::unify::map_raw_type_to_cam;

#[derive(Debug, Clone)]
pub struct FileExtraction {
    pub file_path: String,
    pub components: Vec<ExtractedComponent>,
}

#[derive(Debug, Clone)]
pub struct ExtractedComponent {
    pub name: String,
    pub props: Vec<ExtractedProp>,
    pub source_lang: SourceLanguage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceLanguage {
    Rust,
    Tsx,
}

#[derive(Debug, Clone)]
pub struct ExtractedProp {
    pub name: String,
    pub raw_type: String,
    pub has_default: bool,
    pub is_optional: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynthesisOutput {
    pub ucp_version: String,
    pub components: Vec<CanonicalAbstractComponent>,
    pub stats: PipelineStats,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PipelineStats {
    pub files_scanned: usize,
    pub files_parsed: usize,
    pub components_found: usize,
    pub conflicts_detected: usize,
}

pub fn run_pipeline(source_dir: &str) -> Result<SynthesisOutput> {
    let mut files_scanned = 0usize;
    let mut files_parsed = 0usize;
    let mut all_components: Vec<CanonicalAbstractComponent> = Vec::new();

    let mut rust_extractions: BTreeMap<String, Vec<RawComponentExtraction>> = BTreeMap::new();
    let mut tsx_extractions: BTreeMap<String, Vec<RawTsxExtraction>> = BTreeMap::new();

    walk_source_dir(source_dir, |path| {
        files_scanned += 1;

        match path.extension().and_then(|e| e.to_str()) {
            Some("rs") => {
                let content = std::fs::read_to_string(path)?;
                match rust_ast::extract_rust_components(&content) {
                    Ok(components) if !components.is_empty() => {
                        let path_str = path.to_string_lossy().to_string();
                        rust_extractions.insert(path_str, components);
                        files_parsed += 1;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("  ⚠ Skipping {}: {}", path.display(), e);
                    }
                }
            }
            Some("tsx") | Some("ts") => {
                let content = std::fs::read_to_string(path)?;
                match tsx_ast::extract_tsx_components(&content) {
                    Ok(components) if !components.is_empty() => {
                        let path_str = path.to_string_lossy().to_string();
                        tsx_extractions.insert(path_str, components);
                        files_parsed += 1;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("  ⚠ Skipping {}: {}", path.display(), e);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    })?;

    for (file_path, raw_components) in &rust_extractions {
        for raw in raw_components {
            let cam = unify_rust_component(raw, file_path)?;
            all_components.push(cam);
        }
    }

    for (file_path, raw_components) in &tsx_extractions {
        for raw in raw_components {
            let cam = unify_tsx_component(raw, file_path)?;
            all_components.push(cam);
        }
    }

    let conflicts_detected = all_components
        .iter()
        .map(|c| c.props.iter().filter(|p| !p.conflicts.is_empty()).count())
        .sum();

    Ok(SynthesisOutput {
        ucp_version: "4.0.0".to_string(),
        components: all_components,
        stats: PipelineStats {
            files_scanned,
            files_parsed,
            components_found: rust_extractions.values().map(|v| v.len()).sum::<usize>()
                + tsx_extractions.values().map(|v| v.len()).sum::<usize>(),
            conflicts_detected,
        },
    })
}

fn unify_rust_component(
    raw: &rust_ast::RawComponentExtraction,
    file_path: &str,
) -> Result<CanonicalAbstractComponent> {
    let props: Vec<CanonicalAbstractProp> = raw
        .props
        .iter()
        .map(|rp| {
            let cam_type = map_raw_type_to_cam(&rp.raw_type).unwrap_or(AbstractPropType::Any);
            let reactivity = derive_reactivity(&cam_type, rp.has_default);
            CanonicalAbstractProp {
                canonical_name: rp.name.clone(),
                abstract_type: cam_type,
                reactivity,
                sources: vec![PropSourceMapping {
                    repo_id: file_path.to_string(),
                    original_name: rp.name.clone(),
                    original_type: rp.raw_type.clone(),
                }],
                confidence: 0.9,
                conflicts: vec![],
            }
        })
        .collect();

    Ok(CanonicalAbstractComponent {
        id: format!("rust:{}:{}", file_path, raw.name),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: compute_purpose_hash(&raw.name, &props),
            normalized_prop_names: props.iter().map(|p| p.canonical_name.clone()).collect(),
        },
        props,
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![SourceAttribution {
            repo_url: file_path.to_string(),
            file_path: file_path.to_string(),
            line_start: 0,
        }],
    })
}

fn unify_tsx_component(
    raw: &tsx_ast::RawTsxExtraction,
    file_path: &str,
) -> Result<CanonicalAbstractComponent> {
    let props: Vec<CanonicalAbstractProp> = raw
        .props
        .iter()
        .map(|rp| {
            let cam_type = if rp.raw_type.contains("=>") || rp.raw_type.contains("void") {
                AbstractPropType::AsyncEventHandler(vec![])
            } else {
                map_raw_type_to_cam(&rp.raw_type).unwrap_or(AbstractPropType::Any)
            };
            let reactivity = derive_reactivity(&cam_type, false);
            CanonicalAbstractProp {
                canonical_name: rp.name.clone(),
                abstract_type: cam_type,
                reactivity,
                sources: vec![PropSourceMapping {
                    repo_id: file_path.to_string(),
                    original_name: rp.name.clone(),
                    original_type: rp.raw_type.clone(),
                }],
                confidence: 0.85,
                conflicts: vec![],
            }
        })
        .collect();

    Ok(CanonicalAbstractComponent {
        id: format!("tsx:{}:{}", file_path, raw.name),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: compute_purpose_hash(&raw.name, &props),
            normalized_prop_names: props.iter().map(|p| p.canonical_name.clone()).collect(),
        },
        props,
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![SourceAttribution {
            repo_url: file_path.to_string(),
            file_path: file_path.to_string(),
            line_start: 0,
        }],
    })
}

fn derive_reactivity(cam_type: &AbstractPropType, has_default: bool) -> AbstractReactivity {
    match cam_type {
        AbstractPropType::ControlledValue(_) => AbstractReactivity::Controlled,
        AbstractPropType::UncontrolledValue(_) => AbstractReactivity::Uncontrolled,
        AbstractPropType::ControlFlag if has_default => AbstractReactivity::Static,
        AbstractPropType::ControlFlag => AbstractReactivity::Static,
        AbstractPropType::StaticValue(_) => AbstractReactivity::Static,
        _ => AbstractReactivity::Static,
    }
}

fn compute_purpose_hash(name: &str, props: &[CanonicalAbstractProp]) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    name.to_lowercase().hash(&mut hasher);
    let mut prop_names: Vec<&str> = props.iter().map(|p| p.canonical_name.as_str()).collect();
    prop_names.sort();
    for pn in &prop_names {
        pn.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}

fn walk_source_dir<F>(dir: &str, mut callback: F) -> Result<()>
where
    F: FnMut(&std::path::Path) -> Result<()>,
{
    let root = Path::new(dir);
    if !root.exists() {
        return Ok(());
    }

    fn visit<F>(path: &Path, callback: &mut F, is_root: bool) -> Result<()>
    where
        F: FnMut(&std::path::Path) -> Result<()>,
    {
        if path.is_dir() {
            if !is_root {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.')
                        || [
                            "node_modules",
                            "target",
                            "dist",
                            "build",
                            ".git",
                        ]
                        .contains(&name)
                    {
                        return Ok(());
                    }
                }
            }
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                visit(&entry.path(), callback, false)?;
            }
        } else if path.is_file() {
            callback(path)?;
        }
        Ok(())
    }

    visit(root, &mut callback, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_on_empty_dir_returns_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let output = run_pipeline(tmp.path().to_str().unwrap()).unwrap();
        assert_eq!(output.components.len(), 0);
        assert_eq!(output.stats.files_scanned, 0);
    }

    #[test]
    fn pipeline_extracts_rust_components() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("button.rs"),
            r#"
#[component]
pub fn Button(
    #[prop(default)] disabled: bool,
) -> impl IntoView {
    view! { <button></button> }
}
"#,
        )
        .unwrap();

        let output = run_pipeline(tmp.path().to_str().unwrap()).unwrap();
        assert_eq!(output.components.len(), 1);
        assert_eq!(output.components[0].props.len(), 1);
        assert_eq!(output.components[0].props[0].canonical_name, "disabled");
        assert_eq!(output.stats.files_parsed, 1);
    }

    #[test]
    fn pipeline_extracts_tsx_components() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("Button.tsx"),
            r#"
export interface ButtonProps {
  variant?: "default" | "destructive";
  disabled?: boolean;
  onClick?: () => void;
}
export const Button = (props: ButtonProps) => {
  return <button disabled={props.disabled}>{props.variant ?? "default"}</button>;
};
"#,
        )
        .unwrap();

        let output = run_pipeline(tmp.path().to_str().unwrap()).unwrap();
        assert_eq!(output.components.len(), 1);
        assert_eq!(output.components[0].props.len(), 3);
        let on_click = &output.components[0].props[2];
        assert_eq!(on_click.canonical_name, "onClick");
        assert!(matches!(
            on_click.abstract_type,
            AbstractPropType::AsyncEventHandler(_)
        ));
        assert_eq!(output.stats.files_parsed, 1);
    }

    #[test]
    fn pipeline_skips_node_modules_and_target() {
        let tmp = tempfile::tempdir().unwrap();
        let node = tmp.path().join("node_modules").join("pkg");
        let tgt = tmp.path().join("target").join("debug");
        std::fs::create_dir_all(&node).unwrap();
        std::fs::create_dir_all(&tgt).unwrap();
        std::fs::write(&node.join("comp.tsx"), "export const X = () => null;").unwrap();
        std::fs::write(&tgt.join("lib.rs"), "fn main() {}").unwrap();

        let output = run_pipeline(tmp.path().to_str().unwrap()).unwrap();
        assert_eq!(output.components.len(), 0);
    }

    #[test]
    fn purpose_hash_is_deterministic() {
        let h1 = compute_purpose_hash("Button", &[]);
        let h2 = compute_purpose_hash("Button", &[]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn purpose_hash_differs_by_name() {
        let h1 = compute_purpose_hash("Button", &[]);
        let h2 = compute_purpose_hash("Input", &[]);
        assert_ne!(h1, h2);
    }
}
