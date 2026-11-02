//! AI-facing contract surfaces: the JSON component contract and the
//! Model Context Protocol server that exposes it over stdio.

/// JSON contract export for AI agent consumption.
pub mod ai_contract;
/// MCP (Model Context Protocol) stdio server over the synthesized spec.
pub mod mcp_server;
