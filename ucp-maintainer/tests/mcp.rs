//! Tests for the MCP server - using direct handler invocation.

use serde_json::{Value, json};
use ucp_core::cam::*;
use ucp_synthesizer::contract::mcp_server::{handle_request, error_response};
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};

fn empty_spec() -> SynthesisOutput {
    SynthesisOutput {
        ucp_version: "0.13.0".into(),
        components: vec![],
        stats: PipelineStats {
            files_scanned: 0, files_parsed: 0, components_found: 0,
            conflicts_detected: 0, llm_enriched: false,
        },
        provenance: None, curation_log: None,
    }
}

fn spec_with_button() -> SynthesisOutput {
    let mut spec = empty_spec();
    spec.components.push(CanonicalAbstractComponent {
        id: "rust:button.rs:Button".into(),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: "abc123".into(),
            normalized_prop_names: vec!["label".into()],
        },
        props: vec![],
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![],
        provided_context: None,
        consumed_contexts: vec![],
    });
    spec
}

fn parse_response(raw: &str) -> Value {
    serde_json::from_str(raw).expect("response should be valid JSON")
}

// --- Error handling ---

#[test]
fn mcp_invalid_json_returns_parse_error() {
    let resp = parse_response(&handle_request("{invalid json", &empty_spec()));
    assert_eq!(resp["jsonrpc"], "2.0");
    assert_eq!(resp["error"]["code"], -32700);
    assert!(resp.get("result").is_none());
    assert!(!resp["error"]["message"].as_str().unwrap().is_empty());
}

#[test]
fn mcp_empty_lines_are_ignored() {
    let resp = parse_response(&handle_request("", &empty_spec()));
    assert_eq!(resp["error"]["code"], -32700);
}

#[test]
fn mcp_unknown_method_returns_error() {
    let req = json!({"jsonrpc": "2.0", "method": "foo/bar", "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &empty_spec()));
    assert_eq!(resp["error"]["code"], -32601);
    assert_eq!(resp["error"]["message"], "Method not found");
    assert_eq!(resp["id"], 1);
}

// --- tools/list ---

#[test]
fn mcp_tools_list_responds() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/list", "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &empty_spec()));
    assert_eq!(resp["jsonrpc"], "2.0");
    assert!(resp.get("error").is_none());
    let tools = resp["result"]["tools"].as_array().unwrap();
    assert!(!tools.is_empty());
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(names.contains(&"list_components"));
    assert!(names.contains(&"get_component"));
}

#[test]
fn mcp_tools_list_response_is_valid_json_rpc() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/list", "id": 42});
    let resp = parse_response(&handle_request(&req.to_string(), &empty_spec()));
    assert_eq!(resp["jsonrpc"], "2.0");
    assert_eq!(resp["id"], 42);
    assert!(resp.get("error").is_none());
}

#[test]
fn mcp_tools_list_includes_input_schema() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/list", "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &empty_spec()));
    let tools = resp["result"]["tools"].as_array().unwrap();
    let gc = tools.iter().find(|t| t["name"] == "get_component").unwrap();
    assert_eq!(gc["inputSchema"]["type"], "object");
    assert_eq!(gc["inputSchema"]["properties"]["name"]["type"], "string");
}

// --- tools/call: list_components ---

#[test]
fn mcp_list_components_empty() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "list_components"}, "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &empty_spec()));
    assert_eq!(resp["result"]["components"].as_array().unwrap().len(), 0);
    assert_eq!(resp["result"]["count"], 0);
}

#[test]
fn mcp_list_components_with_data() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "list_components"}, "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &spec_with_button()));
    let comps = resp["result"]["components"].as_array().unwrap();
    assert_eq!(comps.len(), 1);
    assert_eq!(comps[0], "Button");
    assert_eq!(resp["result"]["count"], 1);
}

// --- tools/call: get_component ---

#[test]
fn mcp_get_component_found() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "get_component", "arguments": {"name": "Button"}}, "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &spec_with_button()));
    assert!(resp.get("error").is_none());
    assert_eq!(resp["result"]["name"], "Button");
    assert_eq!(resp["result"]["id"], "rust:button.rs:Button");
    // Props/events are empty but structure exists
    assert!(resp["result"]["props"].is_array());
    assert!(resp["result"]["events"].is_array());
}

#[test]
fn mcp_get_component_not_found() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "get_component", "arguments": {"name": "Missing"}}, "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &spec_with_button()));
    assert_eq!(resp["error"]["code"], -32602);
    assert_eq!(resp["error"]["message"], "Not found");
}

// --- tools/call: export tools ---

#[test]
fn mcp_export_a2ui_returns_content() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "export_a2ui"}, "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &spec_with_button()));
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    let parsed: Value = serde_json::from_str(text).unwrap();
    assert_eq!(parsed["schema"], "https://a2ui.dev/schema/v0.9");
}

#[test]
fn mcp_export_agui_returns_content() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "export_agui"}, "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &spec_with_button()));
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    let parsed: Value = serde_json::from_str(text).unwrap();
    assert_eq!(parsed["protocol"], "ag-ui/v1");
}

#[test]
fn mcp_export_design_md_returns_content() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "export_design_md"}, "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &spec_with_button()));
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("ucp-library"));
}

#[test]
fn mcp_generate_llms_txt_returns_content() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "generate_llms_txt"}, "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &spec_with_button()));
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("UCP Component Library"));
}

#[test]
fn mcp_unknown_tool_returns_error() {
    let req = json!({"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "nonexistent"}, "id": 1});
    let resp = parse_response(&handle_request(&req.to_string(), &empty_spec()));
    assert_eq!(resp["error"]["code"], -32601);
}

// --- error_response helper ---

#[test]
fn error_response_format() {
    let resp = parse_response(&error_response(json!(42), -32600, "bad request"));
    assert_eq!(resp["jsonrpc"], "2.0");
    assert_eq!(resp["error"]["code"], -32600);
    assert_eq!(resp["error"]["message"], "bad request");
    assert_eq!(resp["id"], 42);
    assert!(resp.get("result").is_none());
}

#[test]
fn error_response_with_null_id() {
    let resp = parse_response(&error_response(Value::Null, -32700, "parse error"));
    assert_eq!(resp["id"], Value::Null);
}
