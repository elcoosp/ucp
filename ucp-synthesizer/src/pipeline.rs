use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

use crate::extract::rust_ast::{self, RawComponentExtraction};
use crate::extract::tsx_ast::{self, RawTsxExtraction};
use crate::llm::{build_enrichment_prompt, infer_behavior};
use crate::security::is_path_safe_to_parse;
use crate::unify::map_raw_type_to_cam;

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
    let mut source_map: HashMap<String, String> = HashMap::new();

    let mut rust_extractions: BTreeMap<String, Vec<RawComponentExtraction>> = BTreeMap::new();
    let mut tsx_extractions: BTreeMap<String, Vec<RawTsxExtraction>> = BTreeMap::new();

    walk_source_dir(source_dir, |path| {
        let path_str = path.to_string_lossy().to_string();
        let ext = path.extension().and_then(|e| e.to_str());

        // Only process supported extensions
        if !matches!(ext, Some("rs") | Some("tsx") | Some("ts")) {
            return Ok(());
        }

        // Security: skip sensitive or out-of-scope paths
        if !is_path_safe_to_parse(&path_str) {
            return Ok(());
        }

        files_scanned += 1;
        let content = std::fs::read_to_string(path)?;
        source_map.insert(path_str.clone(), content.clone());

        match ext {
            Some("rs") => {
                match rust_ast::extract_rust_components(&content) {
                    Ok(components) if !components.is_empty() => {
                        rust_extractions.insert(path_str, components);
                        files_parsed += 1;
                    }
                    Ok(_) => {}
                    Err(e) => eprintln!("  ⚠ Skipping {}: {}", path.display(), e),
                }
            }
            Some("tsx") | Some("ts") => {
                match tsx_ast::extract_tsx_components(&content) {
                    Ok(components) if !components.is_empty() => {
                        tsx_extractions.insert(path_str, components);
                        files_parsed += 1;
                    }
                    Ok(_) => {}
                    Err(e) => eprintln!("  ⚠ Skipping {}: {}", path.display(), e),
                }
            }
            _ => unreachable!(),
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
            llm_enriched = enrich_components_with_llm(
                all_components.as_mut(),
                &source_map,
                &opts.llm_model,
            )
            .await?;
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

/// Enrich components via LLM, using a pre-populated source map to avoid
/// re-reading files from disk.
async fn enrich_components_with_llm(
    components: &mut [CanonicalAbstractComponent],
    source_map: &HashMap<String, String>,
    model: &str,
) -> Result<bool> {
    let client = reqwest::Client::builder()
        .build()
        .map_err(|e| ucp_core::UcpError::LlmInference(e.to_string()))?;

    let mut any_success = false;

    for comp in components.iter_mut() {
        // Collect source code from the pre-populated source map
        let source_code_vec: Vec<String> = comp
            .source_repos
            .iter()
            .filter_map(|src| source_map.get(&src.file_path).cloned())
            .collect();

        if source_code_vec.is_empty() {
            continue;
        }

        let source_code = source_code_vec.join("\n\n");
        let comp_display_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let prompt = build_enrichment_prompt(comp_display_name, &source_code);

        match infer_behavior(&client, &source_code, &prompt, model).await {
            Ok(llm_json) => {
                // Enrich semantic fingerprint from description
                if let Some(desc) = llm_json.get("description").and_then(|v| v.as_str()) {
                    comp.semantic_fingerprint.purpose_hash =
                        compute_purpose_hash_with_llm(&comp.semantic_fingerprint, desc);
                    any_success = true;
                }

                // Parse SMDL state machine if returned by LLM
                if let Some(smdl_str) = llm_json.get("smdl").and_then(|v| v.as_str()) {
                    if !smdl_str.is_empty() {
                        match ucp_core::smdl::parse_smdl(smdl_str) {
                            Ok(smdl_json) => {
                                if let Some(machine) = smdl_json_to_state_machine(&smdl_json) {
                                    comp.extracted_state_machine = Some(machine);
                                }
                            }
                            Err(e) => {
                                eprintln!(
                                    "  ⚠ SMDL parse failed for {}: {}",
                                    comp_display_name, e
                                );
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "  ⚠ LLM enrichment failed for {}: {}",
                    comp_display_name, e
                );
            }
        }
    }

    Ok(any_success)
}

/// Convert the serde_json::Value returned by `ucp_core::smdl::parse_smdl`
/// into a proper `StateMachine` CAM struct.
///
/// Note: This is a free function rather than `impl TryFrom<&Value> for StateMachine`
/// because both `StateMachine` (ucp_core) and `Value` (serde_json) are foreign types,
/// which violates Rust's orphan rules.
fn smdl_json_to_state_machine(value: &serde_json::Value) -> Option<StateMachine> {
    let id = value.get("id")?.as_str()?.to_string();
    let initial = value.get("initial")?.as_str()?.to_string();

    let mut states = BTreeMap::new();
    if let Some(states_obj) = value.get("states").and_then(|v| v.as_object()) {
        for (state_name, state_value) in states_obj {
            let mut transitions = BTreeMap::new();
            if let Some(on_map) = state_value.get("on").and_then(|v| v.as_object()) {
                for (event_name, trans_value) in on_map {
                    let target = trans_value.get("target")?.as_str()?.to_string();
                    let side_effects = trans_value
                        .get("sideEffects")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();
                    transitions.insert(
                        event_name.clone(),
                        Transition { target, side_effects },
                    );
                }
            }
            states.insert(
                state_name.clone(),
                StateNode {
                    on: if transitions.is_empty() {
                        None
                    } else {
                        Some(transitions)
                    },
                },
            );
        }
    }

    Some(StateMachine {
        id,
        initial,
        states,
    })
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
            line_start: raw.line_start,
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
            line_start: raw.line_start,
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
    fn smdl_json_to_state_machine_converts_correctly() {
        let json = serde_json::json!({
            "id": "test-dialog",
            "initial": "Closed",
            "states": {
                "Closed": {
                    "on": {
                        "OPEN": {
                            "target": "Open",
                            "sideEffects": ["focus: move_to"]
                        }
                    }
                },
                "Open": {
                    "on": {
                        "CLOSE": {
                            "target": "Closed",
                            "sideEffects": []
                        }
                    }
                }
            }
        });

        let machine = smdl_json_to_state_machine(&json).unwrap();
        assert_eq!(machine.id, "test-dialog");
        assert_eq!(machine.initial, "Closed");
        assert_eq!(machine.states.len(), 2);

        let closed = &machine.states["Closed"];
        assert!(closed.on.is_some());
        let on = closed.on.as_ref().unwrap();
        assert_eq!(on["OPEN"].target, "Open");
        assert_eq!(on["OPEN"].side_effects, vec!["focus: move_to"]);

        let open = &machine.states["Open"];
        assert!(open.on.is_some());
        assert_eq!(open.on.as_ref().unwrap()["CLOSE"].target, "Closed");
        assert!(open.on.as_ref().unwrap()["CLOSE"].side_effects.is_empty());
    }

    #[test]
    fn smdl_json_to_state_machine_returns_none_for_missing_fields() {
        assert!(smdl_json_to_state_machine(&serde_json::json!({"id": "x"})).is_none());
        assert!(smdl_json_to_state_machine(&serde_json::json!({"initial": "A"})).is_none());
    }

    #[test]
    fn smdl_json_to_state_machine_handles_empty_states() {
        let json = serde_json::json!({
            "id": "empty",
            "initial": "Idle",
            "states": { "Idle": {} }
        });
        let machine = smdl_json_to_state_machine(&json).unwrap();
        assert!(machine.states["Idle"].on.is_none());
    }
}
