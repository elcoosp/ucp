use crate::pipeline::SynthesisOutput;
use std::io::{self, BufRead, Write};
use ucp_core::Result;

/// Minimal MCP server that communicates over stdio (JSON-RPC).
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

fn handle_request(request: &str, spec: &SynthesisOutput) -> String {
    if request.contains("dashboard") || request.contains("tools/call") && request.contains("dashboard") {
        return build_dashboard_response(spec);
    }

    // Extremely minimal MCP handler: respond to list_components
    if request.contains("list_components") {
        let names: Vec<&str> = spec
            .components
            .iter()
            .map(|c| c.id.rsplit(':').next().unwrap_or(""))
            .collect();
        format!(
            r#"{{"jsonrpc":"2.0","result":{{"components":{:?}}}}}"#,
            names
        )
    } else if request.contains("get_component") {
        // Very basic: return the first component
        if let Some(comp) = spec.components.first() {
            format!(
                r#"{{"jsonrpc":"2.0","result":{{"id":"{}","name":"{}","props":{}}}}}"#,
                comp.id,
                comp.id.rsplit(':').next().unwrap_or(""),
                comp.props.len()
            )
        } else {
            r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"Not found"}}"#.to_string()
        }
    } else {
        r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"}}"#.to_string()
    }
}

fn build_dashboard_response(spec: &SynthesisOutput) -> String {
    let spec_json = serde_json::to_string_pretty(spec).unwrap_or_else(|_| "{}".into());
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>UCP Dashboard</title>
<script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-50 text-gray-900 min-h-screen">
<div id="app" class="container mx-auto p-4 max-w-7xl">
<header class="mb-8">
<h1 class="text-3xl font-bold">UCP Component Dashboard</h1>
<p class="text-gray-600 mt-2">{} component(s) loaded via MCP</p>
</header>
<div id="component-list" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4"></div>
</div>
<script>
const data = {spec_json};
const components = data.components || [];
const container = document.getElementById('component-list');
components.forEach(comp => {{
  const name = comp.id.split(':').pop();
  const card = document.createElement('div');
  card.className = 'bg-white rounded-lg shadow p-4 cursor-pointer hover:shadow-md';
  card.innerHTML = `<h3 class="font-semibold text-lg">${{name}}</h3><p class="text-sm text-gray-500">${{(comp.props||[]).length}} prop(s) · ${{(comp.events||[]).length}} event(s)</p>`;
  container.appendChild(card);
}});
</script>
</body>
</html>"#,
        spec.components.len()
    );
    format!(r#"{{"jsonrpc":"2.0","result":{{"content":[{{"type":"html","text":{}}}]}}}}"#, serde_json::to_string(&html).unwrap())
}

