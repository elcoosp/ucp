//! LLM enrichment of extracted components (Ollama-compatible backends).
//!
//! For every component whose source is available, a prompt is sent to the
//! model asking for a semantic description and an SMDL state machine.
//! Successful descriptions are folded into the semantic fingerprint so
//! that LLM-enriched specs cluster together even when prop sets diverge.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::hash::BuildHasher;

use ucp_core::cam::{CanonicalAbstractComponent, SemanticFingerprint};

use crate::llm::{build_enrichment_prompt, infer_behavior, parse_enrichment_response};

/// Enrich components in place using an Ollama-compatible endpoint.
///
/// Returns `true` when at least one component was successfully enriched.
/// Failures for individual components are logged to stderr and skipped.
///
/// # Errors
///
/// Returns errors from HTTP client construction; per-component inference
/// failures are reported but not fatal.
pub async fn enrich_components_with_llm<S: BuildHasher>(
    components: &mut [CanonicalAbstractComponent],
    source_map: &HashMap<String, String, S>,
    ollama_url: &str,
    model: &str,
) -> ucp_core::Result<bool> {
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
        if let Some(provided_ctx) = &comp.provided_context {
            let _ = write!(full_prompt, "\n\nProvides context: {provided_ctx}.");
        }
        if !comp.consumed_contexts.is_empty() {
            let _ = write!(
                full_prompt,
                "\nConsumes contexts: {}.",
                comp.consumed_contexts.join(", ")
            );
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
                                comp.extracted_state_machine = Some(smdl.into());
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("  ⚠ LLM enrichment failed for {comp_display_name}: {e}"),
        }
    }

    Ok(any_success)
}

/// Blend an LLM description into a component's semantic fingerprint.
///
/// Words longer than three characters are folded (lower-cased) into the
/// hash together with the original structural hash.
#[must_use]
pub fn compute_purpose_hash_with_llm(
    fingerprint: &SemanticFingerprint,
    llm_description: &str,
) -> String {
    let mut bytes: Vec<u8> = fingerprint.purpose_hash.as_bytes().to_vec();
    for word in llm_description.split_whitespace() {
        if word.len() > 3 {
            bytes.push(0x1f);
            bytes.extend_from_slice(word.to_lowercase().as_bytes());
        }
    }
    format!("{:016x}", ucp_core::fnv1a64(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn llm_hash_blends_description_words() {
        let fp = SemanticFingerprint {
            purpose_hash: "abc123".to_string(),
            normalized_prop_names: vec![],
        };
        let h1 = compute_purpose_hash_with_llm(&fp, "Accessible disclosure button");
        let h2 = compute_purpose_hash_with_llm(&fp, "Accessible disclosure button");
        let h3 = compute_purpose_hash_with_llm(&fp, "Radio group selector");
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
        // Short words (<= 3 chars) are ignored.
        assert_eq!(
            compute_purpose_hash_with_llm(&fp, "a an the of"),
            compute_purpose_hash_with_llm(&fp, "")
        );
    }

    #[test]
    fn smdl_conversion_via_from_trait() {
        let smdl =
            ucp_core::smdl::parse_smdl("component T { state A { on GO -> B {} } state B {} }", "T")
                .unwrap();
        let sm: ucp_core::cam::StateMachine = smdl.into();
        assert_eq!(sm.initial, "A");
        assert!(sm.transition("A", "GO").is_some());
    }
}
