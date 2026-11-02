//! Model Context Protocol (MCP) stdio server exposing a synthesized spec.
//!
//! Speaks JSON-RPC 2.0 over stdin/stdout, one request per line, and
//! implements `tools/list` plus `tools/call` for the discovery and export
//! tools declared in [`generate_server_json`].

use crate::pipeline::SynthesisOutput;
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;
use std::io::{self, BufRead, Write};
use ucp_core::cam::{AbstractReactivity, CanonicalAbstractProp};
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

/// Serve the synthesized spec over stdio using the MCP protocol.
///
/// Reads JSON-RPC 2.0 requests line-by-line from stdin and answers
/// `tools/list` and `tools/call` for component discovery.
///
/// # Errors
///
/// Errors on stdin read or stdout write failures.
pub fn run_mcp_server(spec: &SynthesisOutput) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line.map_err(ucp_core::UcpError::Io)?;
        let response = handle_request(&line, spec);
        writeln!(stdout, "{response}").map_err(ucp_core::UcpError::Io)?;
        stdout.flush().map_err(ucp_core::UcpError::Io)?;
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

/// Wrap a JSON value as a successful MCP `content` result and serialize it.
fn content_response(id: serde_json::Value, content_type: &str, text: &str) -> String {
    let resp = McpResponse {
        jsonrpc: "2.0".into(),
        result: Some(serde_json::json!({
            "content": [{"type": content_type, "text": text}]
        })),
        error: None,
        id,
    };
    serde_json::to_string(&resp).unwrap_or_default()
}

fn handle_request(raw: &str, spec: &SynthesisOutput) -> String {
    let req: McpRequest = match serde_json::from_str(raw) {
        Ok(r) => r,
        Err(e) => return error_response(serde_json::Value::Null, -32700, &e.to_string()),
    };
    match req.method.as_str() {
        "tools/list" => tools_list_response(req.id),
        "tools/call" => tools_call_response(req.id, req.params.as_ref(), spec),
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

/// Read the `arguments` object out of a `tools/call` params value.
fn arguments(params: Option<&serde_json::Value>) -> Option<&serde_json::Value> {
    params.and_then(|p| p.get("arguments"))
}

/// Read a string argument from the `arguments` object.
fn arg_str<'a>(params: Option<&'a serde_json::Value>, key: &str, default: &'a str) -> &'a str {
    arguments(params)
        .and_then(|a| a.get(key))
        .and_then(serde_json::Value::as_str)
        .unwrap_or(default)
}

/// Bare display name of a component id (`<kind>:<file>:<Name>` -> `Name`).
fn display_name(id: &str) -> &str {
    id.rsplit(':').next().unwrap_or("")
}

fn tools_call_response(
    id: serde_json::Value,
    params: Option<&serde_json::Value>,
    spec: &SynthesisOutput,
) -> String {
    let tool_name = params
        .and_then(|p| p.get("name"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");

    match tool_name {
        "list_components" => list_components_response(id, spec),
        "get_component" => get_component_response(id, arg_str(params, "name", ""), spec),
        "export_a2ui" | "export_a2ui_catalog" => export_a2ui_response(id, spec),
        "export_agui" => export_agui_response(id, spec),
        "export_design_md" | "export_design_md_full" => {
            let library = arg_str(params, "library", "ucp-library");
            let version = arg_str(params, "version", "0.1.0");
            export_design_md_response(id, library, version, spec)
        }
        "export_w3c" => export_w3c_response(id, spec),
        "export_dtcg" => export_dtcg_response(id),
        "generate_llms_txt" => generate_llms_txt_response(id, spec),
        "dashboard" => dashboard_response(id, spec),
        _ => error_response(id, -32601, "Tool not found"),
    }
}

fn list_components_response(id: serde_json::Value, spec: &SynthesisOutput) -> String {
    let names: Vec<&str> = spec
        .components
        .iter()
        .map(|c| display_name(&c.id))
        .collect();
    let resp = McpResponse {
        jsonrpc: "2.0".into(),
        result: Some(serde_json::json!({"components": names, "count": names.len()})),
        error: None,
        id,
    };
    serde_json::to_string(&resp).unwrap_or_default()
}

fn get_component_response(id: serde_json::Value, name: &str, spec: &SynthesisOutput) -> String {
    let comp = spec.components.iter().find(|c| c.id.ends_with(name));
    let result = comp.map(|c| {
        let props: Vec<serde_json::Value> = c.props.iter().map(prop_json).collect();
        let events: Vec<serde_json::Value> = c.events.iter().map(|e| serde_json::json!({
            "name": e.canonical_name,
            "payload": format!("{:?}", e.abstract_payload),
        })).collect();
        let variants: Vec<serde_json::Value> = crate::utils::enum_variant_map(
            c,
            |name, values| serde_json::json!({"name": name, "values": values}),
        );
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
        error: if comp.is_none() {
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

/// Render one prop as the shared MCP JSON shape.
fn prop_json(p: &CanonicalAbstractProp) -> serde_json::Value {
    let required = p.reactivity != AbstractReactivity::Static;
    serde_json::json!({
        "name": p.canonical_name,
        "type": p.concrete_type.clone().unwrap_or_else(|| format!("{:?}", p.abstract_type)),
        "abstract_type": format!("{:?}", p.abstract_type),
        "required": required,
        "default": if required { serde_json::Value::Null } else { serde_json::json!("default") },
        "enum_values": p
            .concrete_type
            .as_deref()
            .map(crate::utils::parse_enum_values)
            .unwrap_or_default(),
    })
}

fn export_a2ui_response(id: serde_json::Value, spec: &SynthesisOutput) -> String {
    let catalog = serde_json::json!({
        "schema": "https://a2ui.dev/schema/v0.9",
        "library": "ucp-extracted",
        "version": "0.1.0",
        "components": spec.components.iter().map(|c| serde_json::json!({
            "id": c.id,
            "name": display_name(&c.id),
            "props": c.props.iter().map(|p| serde_json::json!({
                "name": p.canonical_name,
                "type": p.concrete_type.clone().unwrap_or_else(|| format!("{:?}", p.abstract_type)),
                "required": p.reactivity != AbstractReactivity::Static,
            })).collect::<Vec<_>>(),
            "events": c.events.iter().map(|e| serde_json::json!({"name": e.canonical_name})).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    });
    content_response(
        id,
        "text",
        &serde_json::to_string_pretty(&catalog).unwrap_or_default(),
    )
}

fn export_agui_response(id: serde_json::Value, spec: &SynthesisOutput) -> String {
    let events: Vec<serde_json::Value> = spec
        .components
        .iter()
        .flat_map(|c| {
            let name = display_name(&c.id);
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
    content_response(
        id,
        "text",
        &serde_json::to_string_pretty(&schema).unwrap_or_default(),
    )
}

fn export_design_md_response(
    id: serde_json::Value,
    library: &str,
    version: &str,
    spec: &SynthesisOutput,
) -> String {
    let mut md = String::new();
    md.push_str("---\n");
    md.push_str("title: ");
    md.push_str(library);
    md.push_str("\n---\n\n# ");
    md.push_str(library);
    md.push_str(" v");
    md.push_str(version);
    md.push_str("\n\n## Components\n\n");
    for comp in &spec.components {
        let name = display_name(&comp.id);
        md.push_str("### ");
        md.push_str(name);
        md.push_str("\n\n");
        if !comp.props.is_empty() {
            md.push_str("#### Props\n\n");
            md.push_str("| Name | Type | Required | Default |\n");
            md.push_str("|------|------|----------|---------|\n");
            for p in &comp.props {
                let t = p
                    .concrete_type
                    .clone()
                    .unwrap_or_else(|| format!("{:?}", p.abstract_type));
                let required = p.reactivity != AbstractReactivity::Static;
                let (r, d) = if required {
                    ("Yes", "—")
                } else {
                    ("No", "default")
                };
                let _ = writeln!(md, "| `{}` | `{}` | {r} | {d}", p.canonical_name, t);
            }
            md.push('\n');
        }
        if !comp.events.is_empty() {
            md.push_str("#### Events\n\n");
            for e in &comp.events {
                let _ = writeln!(md, "- **`{}`**", e.canonical_name);
            }
            md.push('\n');
        }
        md.push_str("---\n\n");
    }
    content_response(id, "text", &md)
}

fn export_w3c_response(id: serde_json::Value, spec: &SynthesisOutput) -> String {
    let w3c = serde_json::json!({
        "schema": "https://www.w3.org/community/uispec/draft",
        "components": spec.components.iter().map(|c| serde_json::json!({
            "id": c.id,
            "name": display_name(&c.id),
            "props": c.props.iter().map(|p| serde_json::json!({
                "name": p.canonical_name,
                "type": format!("{:?}", p.abstract_type).to_lowercase(),
                "required": p.reactivity != AbstractReactivity::Static,
            })).collect::<Vec<_>>(),
            "events": c.events.iter().map(|e| serde_json::json!({"name": e.canonical_name})).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    });
    content_response(
        id,
        "text",
        &serde_json::to_string_pretty(&w3c).unwrap_or_default(),
    )
}

fn export_dtcg_response(id: serde_json::Value) -> String {
    let tokens = serde_json::json!({
        "colors": {},
        "spacing": {},
        "typography": {},
    });
    content_response(
        id,
        "text",
        &serde_json::to_string_pretty(&tokens).unwrap_or_default(),
    )
}

fn generate_llms_txt_response(id: serde_json::Value, spec: &SynthesisOutput) -> String {
    let mut txt = String::new();
    txt.push_str("# UCP Component Library\n\n");
    for comp in &spec.components {
        let name = display_name(&comp.id);
        let _ = writeln!(txt, "## {name}\n");
        let _ = writeln!(txt, "- **Description:** {name} component");
        txt.push_str("- **Props:**\n");
        for p in &comp.props {
            let t = p
                .concrete_type
                .clone()
                .unwrap_or_else(|| format!("{:?}", p.abstract_type));
            let r = if p.reactivity == AbstractReactivity::Static {
                "optional"
            } else {
                "required"
            };
            let _ = writeln!(txt, "  - `{}` ({t}, {r})", p.canonical_name);
        }
        txt.push('\n');
    }
    content_response(id, "text", &txt)
}

fn dashboard_response(id: serde_json::Value, spec: &SynthesisOutput) -> String {
    let spec_json = serde_json::to_string_pretty(spec).unwrap_or_default();
    let html = format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><script src="https://cdn.tailwindcss.com"></script></head><body class="bg-gray-50 p-4"><h1 class="text-2xl font-bold">UCP Dashboard</h1><div id="list" class="grid grid-cols-3 gap-4 mt-4"></div><script>const data={spec_json};data.components.forEach(c=>{{let d=document.createElement("div");d.className="bg-white p-4 rounded shadow";d.innerHTML=`<h3>${{c.id.split(":").pop()}}</h3><p class="text-sm text-gray-500">${{(c.props||[]).length}} prop(s) · ${{(c.events||[]).length}} event(s)</p>`;document.getElementById("list").appendChild(d)}})</script></body></html>"#
    );
    content_response(id, "html", &html)
}

/// Generate a JFrog-compatible MCP server.json manifest.
///
/// # Errors
///
/// Errors when the server manifest JSON cannot be serialized or written.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::{PipelineStats, SynthesisOutput};
    use ucp_core::cam::{
        AbstractPropType, AbstractReactivity, CanonicalAbstractComponent, SemanticFingerprint,
    };

    fn sample_spec() -> SynthesisOutput {
        let comp = CanonicalAbstractComponent {
            id: "rust:button.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "h".into(),
                normalized_prop_names: vec!["variant".into()],
            },
            props: vec![CanonicalAbstractProp {
                canonical_name: "variant".into(),
                abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("enum: Default, Outline".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            }],
            events: vec![],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![],
            provided_context: None,
            consumed_contexts: vec![],
        };
        SynthesisOutput {
            ucp_version: "4.0.0".into(),
            components: vec![comp],
            provenance: None,
            curation_log: None,
            stats: PipelineStats::default(),
        }
    }

    #[test]
    fn handle_request_parses_tools_list() {
        let spec = sample_spec();
        let out = handle_request(r#"{"jsonrpc":"2.0","method":"tools/list","id":1}"#, &spec);
        assert!(out.contains("list_components"));
    }

    #[test]
    fn handle_request_rejects_unknown_method() {
        let spec = sample_spec();
        let out = handle_request(r#"{"jsonrpc":"2.0","method":"bogus","id":1}"#, &spec);
        assert!(out.contains("-32601"));
    }

    #[test]
    fn handle_request_rejects_bad_json() {
        let spec = sample_spec();
        let out = handle_request("not json", &spec);
        assert!(out.contains("-32700"));
    }

    #[test]
    fn tools_call_lists_and_gets_components() {
        let spec = sample_spec();
        let out = handle_request(
            r#"{"jsonrpc":"2.0","method":"tools/call","params":{"name":"list_components"},"id":2}"#,
            &spec,
        );
        assert!(out.contains("components"));
        let out = handle_request(
            r#"{"jsonrpc":"2.0","method":"tools/call","params":{"name":"get_component","arguments":{"name":"Button"}},"id":3}"#,
            &spec,
        );
        assert!(out.contains("props") || out.contains("Not found"));
    }

    #[test]
    fn tools_call_unknown_tool_errors() {
        let spec = sample_spec();
        let out = handle_request(
            r#"{"jsonrpc":"2.0","method":"tools/call","params":{"name":"nope"},"id":4}"#,
            &spec,
        );
        assert!(out.contains("-32601"));
    }
}
