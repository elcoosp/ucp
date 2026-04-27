//! MCP (Model Context Protocol) server for UCP spec interaction.
//!
//! Implements JSON-RPC 2.0 over stdio, exposing tools for component
//! introspection, consistency checking, and token retrieval.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use ucp_synthesizer::pipeline::SynthesisOutput;

#[derive(Debug, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
    pub id: Value,
}

#[derive(Debug, Serialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
    pub id: Value,
}

#[derive(Debug, Serialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
}

/// Run the MCP server loop, reading JSON-RPC requests from stdin and writing
/// responses to stdout. Each line is one request/response pair.
pub async fn run_mcp_server(spec: SynthesisOutput) {
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        if line.trim().is_empty() {
            continue;
        }
        let request: McpRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let error = McpError { code: -32700, message: e.to_string() };
                let response = McpResponse {
                    jsonrpc: "2.0".into(),
                    result: None,
                    error: Some(error),
                    id: Value::Null,
                };
                let json = serde_json::to_string(&response).unwrap();
                let mut stdout = tokio::io::stdout();
                let _ = stdout.write_all(json.as_bytes()).await;
                let _ = stdout.write_all(b"\n").await;
                continue;
            }
        };

        let response = handle_request(&request, &spec);
        let json = serde_json::to_string(&response).unwrap();
        let mut stdout = tokio::io::stdout();
        let _ = stdout.write_all(json.as_bytes()).await;
        let _ = stdout.write_all(b"\n").await;
    }
}

fn handle_request(req: &McpRequest, spec: &SynthesisOutput) -> McpResponse {
    match req.method.as_str() {
        "tools/list" => tools_list_response(req.id.clone()),
        "tools/call" => tools_call_response(req.id.clone(), req.params.clone(), spec),
        _ => error_response(req.id.clone(), -32601, "Method not found"),
    }
}

fn tools_list_response(id: Value) -> McpResponse {
    let tools = serde_json::json!([
        {
            "name": "list_components",
            "description": "List all components in the spec",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "get_component",
            "description": "Get full metadata for a component",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string" }
                },
                "required": ["name"]
            }
        },
        {
            "name": "check_consistency",
            "description": "Check whether a code snippet uses a known component with correct props",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "code": { "type": "string" },
                    "component": { "type": "string" }
                },
                "required": ["code", "component"]
            }
        },
        {
            "name": "get_tokens",
            "description": "Retrieve design tokens from the spec",
            "inputSchema": { "type": "object", "properties": {} }
        }
    ]);

    McpResponse {
        jsonrpc: "2.0".into(),
        result: Some(serde_json::json!({"tools": tools})),
        error: None,
        id,
    }
}

fn tools_call_response(id: Value, params: Option<Value>, spec: &SynthesisOutput) -> McpResponse {
    let tool_name = params
        .as_ref()
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("");
    let args = params.as_ref().and_then(|p| p.get("arguments"));

    match tool_name {
        "list_components" => {
            let names: Vec<&str> = spec.components.iter().map(|c| c.id.as_str()).collect();
            let result = serde_json::json!({"components": names, "count": names.len()});
            McpResponse { jsonrpc: "2.0".into(), result: Some(result), error: None, id }
        }
        "get_component" => {
            let name = args.and_then(|a| a.get("name")).and_then(|n| n.as_str()).unwrap_or("");
            if let Some(comp) = spec.components.iter().find(|c| c.id.ends_with(name) || c.id == name) {
                let result = component_to_json(comp);
                McpResponse { jsonrpc: "2.0".into(), result: Some(result), error: None, id }
            } else {
                error_response(id, -32602, "Component not found")
            }
        }
        "check_consistency" => {
            let result = serde_json::json!({
                "consistent": false,
                "message": "Consistency check not yet implemented"
            });
            McpResponse { jsonrpc: "2.0".into(), result: Some(result), error: None, id }
        }
        "get_tokens" => {
            let result = serde_json::json!({"tokens": {}});
            McpResponse { jsonrpc: "2.0".into(), result: Some(result), error: None, id }
        }
        _ => error_response(id, -32601, "Tool not found"),
    }
}

fn component_to_json(comp: &ucp_core::cam::CanonicalAbstractComponent) -> Value {
    serde_json::json!({
        "id": comp.id,
        "props": comp.props.iter().map(|p| serde_json::json!({
            "name": p.canonical_name,
            "abstract_type": format!("{:?}", p.abstract_type),
            "concrete_type": p.concrete_type,
            "required": p.reactivity != ucp_core::cam::AbstractReactivity::Static,
        })).collect::<Vec<_>>(),
        "events": comp.events.iter().map(|e| serde_json::json!({
            "name": e.canonical_name,
            "payload": format!("{:?}", e.abstract_payload),
        })).collect::<Vec<_>>(),
        "state_machine": comp.extracted_state_machine.as_ref().map(|sm| serde_json::json!({
            "initial": sm.initial,
            "states": sm.states.keys().collect::<Vec<_>>(),
        })),
    })
}

fn error_response(id: Value, code: i32, message: &str) -> McpResponse {
    McpResponse {
        jsonrpc: "2.0".into(),
        result: None,
        error: Some(McpError { code, message: message.into() }),
        id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucp_synthesizer::pipeline::PipelineStats;

    fn empty_spec() -> SynthesisOutput {
        SynthesisOutput {
            ucp_version: "4.0.0".into(),
            components: vec![],
            stats: PipelineStats {
                files_scanned: 0, files_parsed: 0, components_found: 0,
                conflicts_detected: 0, llm_enriched: false,
            },
            provenance: None,
            curation_log: None,
        }
    }

    #[test]
    fn deserialize_list_tools_request() {
        let raw = r#"{"jsonrpc":"2.0","method":"tools/list","id":1}"#;
        let req: McpRequest = serde_json::from_str(raw).unwrap();
        assert_eq!(req.method, "tools/list");
        assert_eq!(req.id, 1);
    }

    #[test]
    fn deserialize_tools_call_request_with_params() {
        let raw = r#"{"jsonrpc":"2.0","method":"tools/call","params":{"name":"get_component","arguments":{"name":"Button"}},"id":2}"#;
        let req: McpRequest = serde_json::from_str(raw).unwrap();
        assert_eq!(req.method, "tools/call");
        assert_eq!(req.id, 2);
        assert!(req.params.is_some());
        let params = req.params.unwrap();
        assert_eq!(params["name"], "get_component");
        assert_eq!(params["arguments"]["name"], "Button");
    }

    #[test]
    fn deserialize_request_with_null_params() {
        let raw = r#"{"jsonrpc":"2.0","method":"tools/call","params":null,"id":3}"#;
        let req: McpRequest = serde_json::from_str(raw).unwrap();
        assert!(req.params.is_none());
    }

    #[test]
    fn tools_list_response_has_tools_array() {
        let resp = tools_list_response(Value::from(1));
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert!(result["tools"].is_array());
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 4);
        assert_eq!(tools[0]["name"], "list_components");
        assert_eq!(tools[1]["name"], "get_component");
        assert_eq!(tools[2]["name"], "check_consistency");
        assert_eq!(tools[3]["name"], "get_tokens");
    }

    #[test]
    fn tools_list_response_has_input_schemas() {
        let resp = tools_list_response(Value::from(1));
        let result = resp.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        let get_comp = &tools[1];
        assert_eq!(get_comp["inputSchema"]["required"], serde_json::json!(["name"]));
    }

    #[test]
    fn error_response_has_correct_code() {
        let resp = error_response(Value::from(42), -32601, "Method not found");
        assert!(resp.result.is_none());
        assert!(resp.error.is_some());
        let err = resp.error.unwrap();
        assert_eq!(err.code, -32601);
        assert_eq!(err.message, "Method not found");
        assert_eq!(resp.id, 42);
    }

    #[test]
    fn list_components_on_empty_spec() {
        let spec = empty_spec();
        let params = serde_json::json!({"name": "list_components"});
        let resp = tools_call_response(Value::from(1), Some(params), &spec);
        assert!(resp.result.is_some());
        let result = resp.result.unwrap();
        assert_eq!(result["count"], 0);
        assert!(result["components"].as_array().unwrap().is_empty());
    }

    #[test]
    fn get_component_not_found() {
        let spec = empty_spec();
        let params = serde_json::json!({"name": "get_component", "arguments": {"name": "NonExistent"}});
        let resp = tools_call_response(Value::from(1), Some(params), &spec);
        assert!(resp.result.is_none());
        assert!(resp.error.is_some());
        let err = resp.error.unwrap();
        assert_eq!(err.code, -32602);
        assert_eq!(err.message, "Component not found");
    }

    #[test]
    fn unknown_tool_returns_error() {
        let spec = empty_spec();
        let params = serde_json::json!({"name": "nonexistent_tool"});
        let resp = tools_call_response(Value::from(1), Some(params), &spec);
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, -32601);
    }

    #[test]
    fn check_consistency_stub() {
        let spec = empty_spec();
        let params = serde_json::json!({"name": "check_consistency", "arguments": {"code": "<button/>", "component": "Button"}});
        let resp = tools_call_response(Value::from(1), Some(params), &spec);
        assert!(resp.result.is_some());
        let result = resp.result.unwrap();
        assert_eq!(result["consistent"], false);
    }

    #[test]
    fn get_tokens_stub() {
        let spec = empty_spec();
        let params = serde_json::json!({"name": "get_tokens"});
        let resp = tools_call_response(Value::from(1), Some(params), &spec);
        assert!(resp.result.is_some());
        let result = resp.result.unwrap();
        assert!(result["tokens"].is_object());
    }

    #[test]
    fn handle_request_dispatches_tools_list() {
        let spec = empty_spec();
        let req = McpRequest {
            jsonrpc: "2.0".into(),
            method: "tools/list".into(),
            params: None,
            id: Value::from(99),
        };
        let resp = handle_request(&req, &spec);
        assert!(resp.result.is_some());
        assert_eq!(resp.id, 99);
    }

    #[test]
    fn handle_request_dispatches_unknown_method() {
        let spec = empty_spec();
        let req = McpRequest {
            jsonrpc: "2.0".into(),
            method: "fake/method".into(),
            params: None,
            id: Value::Null,
        };
        let resp = handle_request(&req, &spec);
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, -32601);
    }

    #[test]
    fn response_serializes_to_valid_json() {
        let resp = tools_list_response(Value::from(1));
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["jsonrpc"], "2.0");
        assert!(parsed["result"].is_object());
        assert!(parsed["error"].is_null());
    }
}
