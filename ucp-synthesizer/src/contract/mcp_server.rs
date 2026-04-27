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
        let line = line.map_err(|e| ucp_core::UcpError::Io(e))?;
        let response = handle_request(&line, spec);
        writeln!(stdout, "{}", response).map_err(|e| ucp_core::UcpError::Io(e))?;
        stdout.flush().map_err(|e| ucp_core::UcpError::Io(e))?;
    }
    Ok(())
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

fn tools_list_response(id: serde_json::Value) -> String {
    let tools = serde_json::json!([
        {"name": "list_components", "description": "List all components in the spec"},
        {"name": "get_component", "description": "Get full metadata for a component", "inputSchema": {"type": "object", "properties": {"name": {"type": "string"}}}},
        {"name": "export_a2ui", "description": "Export A2UI catalog JSON"},
        {"name": "export_agui", "description": "Export AG-UI event schema JSON"},
        {"name": "export_design_md", "description": "Export DESIGN.md specification"},
        {"name": "export_w3c", "description": "Export W3C UI Spec JSON"},
        {"name": "export_dtcg", "description": "Export DTCG design tokens JSON"},
        {"name": "generate_llms_txt", "description": "Generate LLMs.txt documentation"},
        {"name": "export_a2ui_catalog", "description": "Export complete A2UI catalog with library metadata", "inputSchema": {"type": "object", "properties": {"library": {"type": "string"}, "version": {"type": "string"}}}},
        {"name": "export_design_md_full", "description": "Export DESIGN.md with token integration", "inputSchema": {"type": "object", "properties": {"library": {"type": "string"}, "version": {"type": "string"}}}},
        {"name": "dashboard", "description": "Generate interactive component dashboard HTML"}
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
        "list_components" => {
            let names: Vec<&str> = spec
                .components
                .iter()
                .map(|c| c.id.rsplit(':').next().unwrap_or(""))
                .collect();
            let resp = McpResponse {
                jsonrpc: "2.0".into(),
                result: Some(serde_json::json!({"components": names, "count": names.len()})),
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
            let is_none = comp.is_none();
            let result = comp.map(|c| {
                let props: Vec<serde_json::Value> = c.props.iter().map(|p| serde_json::json!({
                    "name": p.canonical_name,
                    "type": p.concrete_type.clone().unwrap_or_else(|| format!("{:?}", p.abstract_type)),
                    "abstract_type": format!("{:?}", p.abstract_type),
                    "required": p.reactivity != ucp_core::cam::AbstractReactivity::Static,
                    "default": if p.reactivity == ucp_core::cam::AbstractReactivity::Static { serde_json::json!("default") } else { serde_json::Value::Null },
                    "enum_values": extract_enum_values(p),
                })).collect();
                let events: Vec<serde_json::Value> = c.events.iter().map(|e| serde_json::json!({
                    "name": e.canonical_name,
                    "payload": format!("{:?}", e.abstract_payload),
                })).collect();
                let variants: Vec<serde_json::Value> = extract_variants_json(c);
                serde_json::json!({
                    "id": c.id,
                    "name": name,
                    "props": props,
                    "events": events,
                    "variants": variants,
                    "state_machine": c.extracted_state_machine.as_ref().map(|sm| serde_json::json!({
                        "initial": sm.initial,
                        "states": sm.states.keys().collect::<Vec<&String>>()
                    })),
                    "parts": c.extracted_parts.iter().map(|p| serde_json::json!({"name": p.name, "selectable": p.selectable})).collect::<Vec<_>>(),
                    "provided_context": c.provided_context,
                    "consumed_contexts": c.consumed_contexts,
                })
            });
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
        "export_a2ui" => {
            let catalog = serde_json::json!({
                "schema": "https://a2ui.dev/schema/v0.9",
                "library": "ucp-extracted",
                "version": "0.1.0",
                "components": spec.components.iter().map(|c| serde_json::json!({
                    "id": c.id,
                    "name": c.id.rsplit(':').next().unwrap_or(""),
                    "props": c.props.iter().map(|p| serde_json::json!({
                        "name": p.canonical_name,
                        "type": p.concrete_type.clone().unwrap_or_else(|| format!("{:?}", p.abstract_type)),
                        "required": p.reactivity != ucp_core::cam::AbstractReactivity::Static,
                    })).collect::<Vec<_>>(),
                    "events": c.events.iter().map(|e| serde_json::json!({"name": e.canonical_name})).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
            });
            let resp = McpResponse {
                jsonrpc: "2.0".into(),
                result: Some(
                    serde_json::json!({"content": [{"type": "text", "text": serde_json::to_string_pretty(&catalog).unwrap_or_default()}]}),
                ),
                error: None,
                id,
            };
            serde_json::to_string(&resp).unwrap_or_default()
        }
        "export_agui" => {
            let events: Vec<serde_json::Value> = spec
                .components
                .iter()
                .flat_map(|c| {
                    let name = c.id.rsplit(':').next().unwrap_or("");
                    c.events.iter().map(move |e| {
                        serde_json::json!({
                            "component": name,
                            "event": e.canonical_name,
                            "event_type": format!("component.{}", e.canonical_name),
                        })
                    })
                })
                .collect();
            let schema = serde_json::json!({"protocol": "ag-ui/v1", "events": events});
            let resp = McpResponse {
                jsonrpc: "2.0".into(),
                result: Some(
                    serde_json::json!({"content": [{"type": "text", "text": serde_json::to_string_pretty(&schema).unwrap_or_default()}]}),
                ),
                error: None,
                id,
            };
            serde_json::to_string(&resp).unwrap_or_default()
        }
        "export_design_md" | "export_design_md_full" => {
            let library = params
                .as_ref()
                .and_then(|p| p.get("arguments"))
                .and_then(|a| a.get("library"))
                .and_then(|l| l.as_str())
                .unwrap_or("ucp-library");
            let version = params
                .as_ref()
                .and_then(|p| p.get("arguments"))
                .and_then(|a| a.get("version"))
                .and_then(|v| v.as_str())
                .unwrap_or("0.1.0");
            // Build a minimal DESIGN.md in memory
            let mut md = String::new();
            md.push_str("---\n");
            md.push_str(&format!("title: {}\n", library));
            md.push_str("---\n\n");
            md.push_str(&format!("# {} v{}\n\n", library, version));
            md.push_str("## Components\n\n");
            for comp in &spec.components {
                let name = comp.id.rsplit(':').next().unwrap_or("");
                md.push_str(&format!("### {}\n\n", name));
                if !comp.props.is_empty() {
                    md.push_str("#### Props\n\n");
                    md.push_str("| Name | Type | Required | Default |\n");
                    md.push_str("|------|------|----------|---------|\n");
                    for p in &comp.props {
                        let t = p
                            .concrete_type
                            .clone()
                            .unwrap_or_else(|| format!("{:?}", p.abstract_type));
                        let r = if p.reactivity != ucp_core::cam::AbstractReactivity::Static {
                            "Yes"
                        } else {
                            "No"
                        };
                        let d = if p.reactivity == ucp_core::cam::AbstractReactivity::Static {
                            "default"
                        } else {
                            "—"
                        };
                        md.push_str(&format!(
                            "| `{}` | `{}` | {} | {} |\n",
                            p.canonical_name, t, r, d
                        ));
                    }
                    md.push('\n');
                }
                if !comp.events.is_empty() {
                    md.push_str("#### Events\n\n");
                    for e in &comp.events {
                        md.push_str(&format!("- **`{}`**\n", e.canonical_name));
                    }
                    md.push('\n');
                }
                md.push_str("---\n\n");
            }
            let resp = McpResponse {
                jsonrpc: "2.0".into(),
                result: Some(serde_json::json!({"content": [{"type": "text", "text": md}]})),
                error: None,
                id,
            };
            serde_json::to_string(&resp).unwrap_or_default()
        }
        "export_w3c" => {
            let w3c = serde_json::json!({
                "schema": "https://www.w3.org/community/uispec/draft",
                "components": spec.components.iter().map(|c| serde_json::json!({
                    "id": c.id,
                    "name": c.id.rsplit(':').next().unwrap_or(""),
                    "props": c.props.iter().map(|p| serde_json::json!({
                        "name": p.canonical_name,
                        "type": format!("{:?}", p.abstract_type).to_lowercase(),
                        "required": p.reactivity != ucp_core::cam::AbstractReactivity::Static,
                    })).collect::<Vec<_>>(),
                    "events": c.events.iter().map(|e| serde_json::json!({"name": e.canonical_name})).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
            });
            let resp = McpResponse {
                jsonrpc: "2.0".into(),
                result: Some(
                    serde_json::json!({"content": [{"type": "text", "text": serde_json::to_string_pretty(&w3c).unwrap_or_default()}]}),
                ),
                error: None,
                id,
            };
            serde_json::to_string(&resp).unwrap_or_default()
        }
        "export_dtcg" => {
            let tokens = serde_json::json!({
                "colors": {},
                "spacing": {},
                "typography": {},
            });
            let resp = McpResponse {
                jsonrpc: "2.0".into(),
                result: Some(
                    serde_json::json!({"content": [{"type": "text", "text": serde_json::to_string_pretty(&tokens).unwrap_or_default()}]}),
                ),
                error: None,
                id,
            };
            serde_json::to_string(&resp).unwrap_or_default()
        }
        "generate_llms_txt" => {
            let mut txt = String::new();
            txt.push_str("# UCP Component Library\n\n");
            for comp in &spec.components {
                let name = comp.id.rsplit(':').next().unwrap_or("");
                txt.push_str(&format!("## {}\n\n", name));
                txt.push_str(&format!("- **Description:** {} component\n", name));
                txt.push_str("- **Props:**\n");
                for p in &comp.props {
                    let t = p
                        .concrete_type
                        .clone()
                        .unwrap_or_else(|| format!("{:?}", p.abstract_type));
                    let r = if p.reactivity != ucp_core::cam::AbstractReactivity::Static {
                        "required"
                    } else {
                        "optional"
                    };
                    txt.push_str(&format!("  - `{}` ({}, {})\n", p.canonical_name, t, r));
                }
                txt.push('\n');
            }
            let resp = McpResponse {
                jsonrpc: "2.0".into(),
                result: Some(serde_json::json!({"content": [{"type": "text", "text": txt}]})),
                error: None,
                id,
            };
            serde_json::to_string(&resp).unwrap_or_default()
        }
        "dashboard" => {
            let spec_json = serde_json::to_string_pretty(spec).unwrap_or_default();
            let html = format!(
                r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><script src="https://cdn.tailwindcss.com"></script></head><body class="bg-gray-50 p-4"><h1 class="text-2xl font-bold">UCP Dashboard</h1><div id="list" class="grid grid-cols-3 gap-4 mt-4"></div><script>const data={};data.components.forEach(c=>{{let d=document.createElement("div");d.className="bg-white p-4 rounded shadow";d.innerHTML=`<h3>${{c.id.split(":").pop()}}</h3><p class="text-sm text-gray-500">${{(c.props||[]).length}} prop(s) · ${{(c.events||[]).length}} event(s)</p>`;document.getElementById("list").appendChild(d)}})</script></body></html>"#,
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
        _ => error_response(id, -32601, "Tool not found"),
    }
}

fn extract_enum_values(p: &ucp_core::cam::CanonicalAbstractProp) -> Vec<String> {
    if let Some(conc) = &p.concrete_type {
        if conc.starts_with("enum: ") {
            return conc[6..].split(',').map(|s| s.trim().to_string()).collect();
        }
    }
    vec![]
}

fn extract_variants_json(c: &ucp_core::cam::CanonicalAbstractComponent) -> Vec<serde_json::Value> {
    c.props
        .iter()
        .filter_map(|p| {
            if let Some(conc) = &p.concrete_type {
                if conc.starts_with("enum: ") {
                    let values: Vec<String> =
                        conc[6..].split(',').map(|s| s.trim().to_string()).collect();
                    return Some(serde_json::json!({"name": p.canonical_name, "values": values}));
                }
            }
            None
        })
        .collect()
}

/// Generate a JFrog-compatible MCP server.json manifest.
pub fn generate_server_json(name: &str, description: &str, output_dir: &str) -> Result<()> {
    let manifest = serde_json::json!({
        "name": name,
        "description": description,
        "version": env!("CARGO_PKG_VERSION"),
        "tools": [
            {"name": "list_components", "description": "List all components in the spec"},
            {"name": "get_component", "description": "Get full metadata for a component"},
            {"name": "export_a2ui", "description": "Export A2UI catalog JSON"},
            {"name": "export_agui", "description": "Export AG-UI event schema JSON"},
            {"name": "export_design_md", "description": "Export DESIGN.md specification"},
            {"name": "export_w3c", "description": "Export W3C UI Spec JSON"},
            {"name": "export_dtcg", "description": "Export DTCG design tokens JSON"},
            {"name": "generate_llms_txt", "description": "Generate LLMs.txt documentation"},
            {"name": "dashboard", "description": "Generate interactive component dashboard HTML"}
        ],
        "transport": "stdio",
        "repository": "https://github.com/elcoosp/ucp"
    });
    let json = serde_json::to_string_pretty(&manifest).map_err(ucp_core::UcpError::Json)?;
    std::fs::write(std::path::Path::new(output_dir).join("server.json"), json)
        .map_err(ucp_core::UcpError::Io)?;
    Ok(())
}
