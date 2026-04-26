use crate::llm::{build_enrichment_prompt, infer_behavior, parse_enrichment_response};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use ucp_core::cam::*;
use ucp_core::Result;

pub async fn enrich_components_with_llm(
    components: &mut [CanonicalAbstractComponent],
    source_map: &HashMap<String, String>,
    ollama_url: &str,
    model: &str,
) -> Result<bool> {
    let client = reqwest::Client::builder()
        .build()
        .map_err(|e| ucp_core::UcpError::Http(e.to_string()))?;

    let mut any_success = false;

    for comp in components.iter_mut() {
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

        let mut full_prompt = prompt;
        if let Some(ref provided_ctx) = comp.provided_context {
            full_prompt.push_str(&format!("\n\nProvides context: {}.", provided_ctx));
        }
        if !comp.consumed_contexts.is_empty() {
            full_prompt.push_str(&format!(
                "\nConsumes contexts: {}.",
                comp.consumed_contexts.join(", ")
            ));
        }

        match infer_behavior(&client, ollama_url, &source_code, &full_prompt, model).await {
            Ok(llm_json) => {
                if let Ok(llm_response) = parse_enrichment_response(llm_json) {
                    if let Some(desc) = llm_response.description.as_deref() {
                        comp.semantic_fingerprint.purpose_hash =
                            compute_purpose_hash_with_llm(&comp.semantic_fingerprint, desc);
                        any_success = true;
                    }
                    if let Some(smdl_str) = llm_response.smdl.as_deref() {
                        if !smdl_str.is_empty() {
                            if let Ok(smdl) =
                                ucp_core::smdl::parse_smdl(smdl_str, comp_display_name)
                            {
                                if let Some(machine) = smdl_to_state_machine(&smdl) {
                                    comp.extracted_state_machine = Some(machine);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("  ⚠ LLM enrichment failed for {}: {}", comp_display_name, e),
        }
    }

    Ok(any_success)
}

pub fn smdl_to_state_machine(smdl: &ucp_core::smdl::SmdlComponent) -> Option<StateMachine> {
    let id = smdl.id.clone();
    let initial = smdl.initial.clone();
    let mut states = BTreeMap::new();
    for (state_name, state_value) in &smdl.states {
        let mut transitions = BTreeMap::new();
        if let Some(on_map) = &state_value.on {
            for (event_name, trans_value) in on_map {
                transitions.insert(
                    event_name.clone(),
                    Transition {
                        target: trans_value.target.clone(),
                        side_effects: trans_value.side_effects.clone(),
                    },
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
    Some(StateMachine {
        id,
        initial,
        states,
    })
}

pub fn compute_purpose_hash_with_llm(
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
