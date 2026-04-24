use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

use crate::extract::rust_ast::{self, RawComponentExtraction};
use crate::extract::tsx_ast::{self, RawTsxExtraction};
use crate::llm::{build_enrichment_prompt, extract_source_code, infer_behavior};
use crate::unify::map_raw_type_to_cam;

#[derive(Debug, Clone)]
pub struct FileExtraction {
    pub file_path: String,
    pub components: Vec<ExtractedComponent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceLanguage {
    Rust,
    Tsx,
}

#[derive(Debug, Clone)]
pub struct ExtractedComponent {
    pub name: String,
    pub props: Vec<ExtractedProp>,
    pub source_lang: SourceLanguage,
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
    pub llm_enriched: bool,
}

#[derive(Debug, Clone)]
pub struct PipelineOptions {
    pub ollama_url: Option<String>,
    pub llm_model: String,
    pub dry_run: bool,
}

impl Default for PipelineOptions {
    fn default() -> Self {
        Self {
            ollama_url: None,
            llm_model: "glm-5:cloud".to_string(),
            dry_run: false,
        }
    }
}

pub async fn run_pipeline(source_dir: &str) -> Result<SynthesisOutput> {
    run_pipeline_with_options(source_dir, &PipelineOptions::default()).await
}

pub async fn run_pipeline_with_options(source_dir: &str, opts: &PipelineOptions) -> Result<SynthesisOutput> {
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

    // Optional LLM enrichment
    let mut llm_enriched = false;
    if let Some(ref _url) = opts.ollama_url {
        if !opts.dry_run {
            llm_enriched = enrich_components_with_llm(all_components.as_mut(), &opts.llm_model).await?;
            if llm_enriched {
                println!("   🧠 LLM enrichment applied to {} components", all_components.len());
            } else {
                eprintln!("   ⚠ LLM enrichment returned partial results");
            }
        } else {
            println!("   ℹ️ No Ollama URL provided, skipping LLM enrichment");
        }
    }

    // Detect conflicts
    detect_conflicts(&mut all_components);

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
            llm_enriched,
        },
    })
}

async fn enrich_components_with_llm(
    components: &mut [CanonicalAbstractComponent],
    model: &str,
) -> Result<bool> {
    let client = reqwest::Client::builder()
        .build()
        .map_err(|e| ucp_core::UcpError::LlmInference(e.to_string()))?;

    let mut any_success = false;

    for comp in components.iter_mut() {
        let source_code_vec = extract_source_code(std::slice::from_ref(&*comp));

        if source_code_vec.is_empty() {
            continue;
        }

        let source_code = source_code_vec.join("\n\n");
        let comp_display_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let prompt = build_enrichment_prompt(comp_display_name);

        match infer_behavior(&client, &source_code, &prompt, model).await {
            Ok(llm_json) => {
                if let Some(desc) = llm_json.get("description").and_then(|v| v.as_str()) {
                    comp.semantic_fingerprint.purpose_hash =
                        compute_purpose_hash_with_llm(&comp.semantic_fingerprint, desc);
                    any_success = true;
                }
            }
            Err(e) => {
                eprintln!("  ⚠ LLM enrichment failed for {}: {}", comp_display_name, e);
            }
        }
    }

    Ok(any_success)
}

fn compute_purpose_hash_with_llm(
    fingerprint: &SemanticFingerprint,
    llm_description: &str,
) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    fingerprint.purpose_hash.hash(&mut hasher);
    for word in llm_description.split_whitespace() {
        if word.len() > 3 {
            word.to_lowercase().hash(&mut hasher);
        }
    }
    format!("{:016x}", hasher.finish())
}

fn detect_conflicts(components: &mut [CanonicalAbstractComponent]) {
    let mut hash_groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, comp) in components.iter().enumerate() {
        hash_groups
            .entry(comp.semantic_fingerprint.purpose_hash.clone())
            .or_default()
            .push(idx);
    }

    let mut conflict_id_counter = 0u32;

    for (_purpose_hash, indices) in &hash_groups {
        if indices.len() <= 1 {
            continue;
        }

        let mut prop_entries: HashMap<String, Vec<usize>> = HashMap::new();
        for &idx in indices {
            let comp = &components[idx];
            for prop in &comp.props {
                prop_entries
                    .entry(prop.canonical_name.clone())
                    .or_default()
                    .push(idx);
            }
        }

        for (prop_name, member_indices) in &prop_entries {
            let present_in: Vec<String> = member_indices
                .iter()
                .map(|&idx| {
                    components[idx]
                        .source_repos
                        .first()
                        .map(|s| s.file_path.clone())
                        .unwrap_or_else(|| "unknown".to_string())
                })
                .collect();

            let mut type_variants: Vec<String> = member_indices
                .iter()
                .map(|&idx| {
                    components[idx]
                        .props
                        .iter()
                        .find(|p| p.canonical_name == *prop_name)
                        .map(|p| format!("{:?}", p.abstract_type))
                        .unwrap_or_else(|| "missing".to_string())
                })
                .collect();
            type_variants.sort();
            type_variants.dedup();

            if type_variants.len() <= 1 {
                continue;
            }

            conflict_id_counter += 1;
            let conflict_id = format!("conf_{:03}", conflict_id_counter);

            let has_count = member_indices.len();
            let missing_indices: Vec<usize> = (0..components.len())
                .filter(|i| !member_indices.contains(i))
                .collect();

            let absent_in: Vec<String> = missing_indices
                .iter()
                .map(|&idx| {
                    components[idx]
                        .source_repos
                        .first()
                        .map(|s| s.file_path.clone())
                        .unwrap_or_else(|| "unknown".to_string())
                })
                .filter(|s| !present_in.contains(s))
                .collect();

            let confidence = if has_count > 2 { 0.4 } else { 0.7 };

            let resolution = if has_count > 2 {
                ResolutionStrategy::FlagForHumanReview
            } else {
                ResolutionStrategy::IncludeMajority
            };

            for &idx in member_indices {
                if let Some(prop) = components[idx]
                    .props
                    .iter_mut()
                    .find(|p| p.canonical_name == *prop_name)
                {
                    prop.conflicts.push(Conflict {
                        id: conflict_id.clone(),
                        field: format!("props.{}", prop_name),
                        present_in: present_in.clone(),
                        absent_in: absent_in.clone(),
                        confidence,
                        resolution_suggestion: resolution.clone(),
                    });
                }
            }
        }
    }
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
