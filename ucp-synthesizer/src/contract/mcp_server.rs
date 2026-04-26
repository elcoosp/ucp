use crate::pipeline::SynthesisOutput;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};
use ucp_core::Result;

#[derive(Deserialize)]
#[allow(dead_code)]
struct McpRequest {
    jsonrpc: String,
    method: String,
    #[serde(default)]
    params: Option<serde_json::Value>,
    id: serde_json::Value,
}

#[derive(Serialize)]
struct McpResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<McpError>,
    id: serde_json::Value,
}

#[derive(Serialize)]
struct McpError {
    code: i32,
    message: String,
}

pub async fn run_mcp_server(spec: &SynthesisOutput) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line.map_err(ucp_core::UcpError::Io)?;
        let response = handle_request(&line, spec);
        writeln!(stdout, "{}", response).map_err(ucp_core::UcpError::Io)?;
        stdout.flush().map_err(ucp_core::UcpError::Io)?;
    }
    Ok(())
}

fn handle_request(raw: &str, spec: &SynthesisOutput) -> String {
    let req: McpRequest = match serde_json::from_str(raw) {
        Ok(r) => r,
        Err(e) => return error_response(serde_json::Value::Null, -32700, &e.to_string()),
    };
    match req.method.as_str() {
        "tools/list" => tools_list_response(req.id),
        "tools/call" => tools_call_response(req.id, req.params, spec),
        _ => error_response(req.id, -32601, "Method not found"),
    }
}

fn error_response(id: serde_json::Value, code: i32, message: &str) -> String {
    let resp = McpResponse {
        jsonrpc: "2.0".into(),
        result: None,
        error: Some(McpError {
            code,
            message: message.into(),
        }),
        id,
    };
    serde_json::to_string(&resp).unwrap_or_else(|_| {
        r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"Internal error"},"id":null}"#.into()
    })
}

fn tools_list_response(id: serde_json::Value) -> String {
    let tools = serde_json::json!([
        {"name": "dashboard", "description": "Generate an interactive component dashboard", "inputSchema": {"type": "object", "properties": {}}},
        {"name": "list_components", "description": "List all components in the spec", "inputSchema": {"type": "object", "properties": {}}},
        {"name": "get_component", "description": "Get details for a specific component", "inputSchema": {"type": "object", "properties": {"name": {"type": "string"}}}}
    ]);
    let resp = McpResponse {
        jsonrpc: "2.0".into(),
        result: Some(serde_json::json!({"tools": tools})),
        error: None,
        id,
    };
    serde_json::to_string(&resp).unwrap_or_default()
}

fn tools_call_response(
    id: serde_json::Value,
    params: Option<serde_json::Value>,
    spec: &SynthesisOutput,
) -> String {
    let tool_name = params
        .as_ref()
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("");

    match tool_name {
        "dashboard" => {
            let spec_json = serde_json::to_string_pretty(spec).unwrap_or_default();
            let html = format!(
                r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><script src="https://cdn.tailwindcss.com"></script></head><body class="bg-gray-50 p-4"><h1 class="text-2xl font-bold">UCP Dashboard</h1><div id="list" class="grid grid-cols-3 gap-4"></div><script>const data={};data.components.forEach(c=>{{let d=document.createElement("div");d.className="bg-white p-4 rounded shadow";d.innerHTML=`<h3>${{c.id.split(":").pop()}}</h3>`;document.getElementById("list").appendChild(d)}})</script></body></html>"#,
                spec_json
            );
            let resp = McpResponse {
                jsonrpc: "2.0".into(),
                result: Some(serde_json::json!({"content": [{"type": "html", "text": html}]})),
                error: None,
                id,
            };
            serde_json::to_string(&resp).unwrap_or_default()
        }
        "list_components" => {
            let names: Vec<&str> = spec
                .components
                .iter()
                .map(|c| c.id.rsplit(':').next().unwrap_or(""))
                .collect();
            let resp = McpResponse {
                jsonrpc: "2.0".into(),
                result: Some(serde_json::json!({"components": names})),
                error: None,
                id,
            };
            serde_json::to_string(&resp).unwrap_or_default()
        }
        "get_component" => {
            let name = params
                .as_ref()
                .and_then(|p| p.get("arguments"))
                .and_then(|a| a.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("");
            let comp = spec.components.iter().find(|c| c.id.ends_with(name));
            let result = comp.map(|c| {
                serde_json::json!({
                    "id": c.id,
                    "props": c.props.len(),
                    "events": c.events.len(),
                })
            });
            let is_none = result.is_none();
            let resp = McpResponse {
                jsonrpc: "2.0".into(),
                result,
                error: if is_none {
                    Some(McpError {
                        code: -32602,
                        message: "Not found".into(),
                    })
                } else {
                    None
                },
                id,
            };
            serde_json::to_string(&resp).unwrap_or_default()
        }
        _ => error_response(id, -32601, "Tool not found"),
    }
}
