use ucp_synthesizer::llm::build_ollama_payload;

#[test]
fn payload_includes_format_json() {
    let payload = build_ollama_payload("Analyze this", "glm-5:cloud");
    let json_str = serde_json::to_string(&payload).unwrap();

    assert!(json_str.contains("\"format\":\"json\""));
    assert!(json_str.contains("\"stream\":false"));
    assert!(json_str.contains("\"glm-5:cloud\""));
    assert!(json_str.contains("\"temperature\":0.1"));
}

#[test]
fn payload_contains_system_prompt() {
    let payload = build_ollama_payload("Extract props", "glm-5:cloud");
    let prompt = payload["prompt"].as_str().unwrap();
    assert!(prompt.contains("UI component analyzer"));
    assert!(prompt.contains("Extract props"));
}
