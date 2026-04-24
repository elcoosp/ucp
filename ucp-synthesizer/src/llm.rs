use reqwest::Client;
use serde_json::json;
use ucp_core::{Result, UcpError};

pub fn build_ollama_payload(prompt: &str, model: &str) -> serde_json::Value {
    json!({
        "model": model,
        "prompt": format!(
            "You are a UI component analyzer. Return ONLY valid JSON.\n\n{}",
            prompt
        ),
        "stream": false,
        "format": "json",
        "options": {
            "temperature": 0.1
        }
    })
}

/// Typed representation of the LLM enrichment response.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct EnrichmentResponse {
    pub description: Option<String>,
    pub smdl: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
}

/// Parse a raw JSON value into a typed `EnrichmentResponse`.
pub fn parse_enrichment_response(value: serde_json::Value) -> Result<EnrichmentResponse> {
    serde_json::from_value(value)
        .map_err(|e| UcpError::LlmInference(format!("Failed to parse LLM response: {}", e)))
}

/// Infer behavior from a code string via Ollama instance at the given base URL.
pub async fn infer_behavior(
    client: &Client,
    base_url: &str,
    code: &str,
    prompt: &str,
    model: &str,
) -> Result<serde_json::Value> {
    let url = format!("{}/api/generate", base_url.trim_end_matches('/'));
    let full_prompt = format!("{}\n\nCode:\n```rust\n{}\n```", prompt, code);
    let payload = build_ollama_payload(&full_prompt, model);

    let res: serde_json::Value = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| UcpError::LlmInference(e.to_string()))?
        .json()
        .await
        .map_err(|e| UcpError::LlmInference(e.to_string()))?;

    let json_str = res["response"]
        .as_str()
        .ok_or_else(|| {
            UcpError::LlmInference("Missing 'response' field in Ollama output".to_string())
        })?;

    serde_json::from_str(json_str).map_err(|e| {
        UcpError::LlmInference(format!("Failed to parse LLM JSON: {}", e))
    })
}

/// Build a component enrichment prompt for the LLM.
pub fn build_enrichment_prompt(component_name: &str, code: &str) -> String {
    format!(
        "Analyze this UI component named '{}' and return JSON with:\n\
         {{\n\
         \"description\": \"A one-sentence semantic description of what this component does\",\n\
         \"smdl\": \"SMDL state machine definition or empty string if none\",\n\
         \"keywords\": [\"list\", \"of\", \"semantic\", \"keywords\"]\n\
         }}\n\n\
         Code:\n{}",
        component_name,
        code
    )
}
