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
