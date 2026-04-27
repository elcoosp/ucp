//! Integration tests for the MCP server.
//!
//! These tests spawn the `ucp mcp` CLI subcommand and verify
//! JSON-RPC request/response behaviour over stdio.

use std::io::Write;
use std::process::{Command, Stdio};
use predicates;

/// Send a single JSON-RPC request to the MCP server and return the response.
fn send_mcp_request(spec_path: &str, request: &str) -> String {
    let mut child = Command::new(assert_cmd::cargo::cargo_bin("ucp"))
        .args(&["mcp", "--spec", spec_path])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn ucp process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin.write_all(request.as_bytes()).expect("Failed to write request");
        stdin.write_all(b"\n").expect("Failed to write newline");
    }

    let output = child.wait_with_output().expect("Failed to wait for process");
    String::from_utf8(output.stdout).expect("Output was not valid UTF-8")
}

/// Minimal valid SynthesisOutput JSON for testing.
fn minimal_spec_json() -> String {
    r#"{"ucp_version":"4.0.0","components":[],"stats":{"files_scanned":0,"files_parsed":0,"components_found":0,"conflicts_detected":0,"llm_enriched":false}}"#.to_string()
}

#[test]
fn mcp_tools_list_responds() {
    let spec_path = "tests/mcp.json";
    std::fs::write(spec_path, &minimal_spec_json()).expect("Failed to write test spec");

    let response = send_mcp_request(spec_path, r#"{"jsonrpc":"2.0","method":"tools/list","id":1}"#);
    let trimmed = response.trim();

    let res: serde_json::Value = serde_json::from_str(trimmed)
        .expect("Response should be valid JSON");

    assert!(
        res.get("result").is_some(),
        "Response should have 'result' field, got: {}",
        trimmed
    );
    assert!(
        res["result"]["tools"].as_array().is_some(),
        "result.tools should be an array"
    );
}

#[test]
fn mcp_tools_list_response_is_valid_json_rpc() {
    let spec_path = "tests/mcp.json";
    std::fs::write(spec_path, &minimal_spec_json()).expect("Failed to write test spec");

    let response = send_mcp_request(spec_path, r#"{"jsonrpc":"2.0","method":"tools/list","id":42}"#);
    let res: serde_json::Value = serde_json::from_str(response.trim())
        .expect("Response should be valid JSON");

    assert_eq!(res["jsonrpc"], "2.0");
    assert_eq!(res["id"], 42);
    assert!(res["error"].is_null());
}

#[test]
fn mcp_unknown_method_returns_error() {
    let spec_path = "tests/mcp.json";
    std::fs::write(spec_path, &minimal_spec_json()).expect("Failed to write test spec");

    let response = send_mcp_request(
        spec_path,
        r#"{"jsonrpc":"2.0","method":"nonexistent/method","id":1}"#,
    );
    let res: serde_json::Value = serde_json::from_str(response.trim())
        .expect("Response should be valid JSON");

    assert!(res["error"].is_object(), "Should return an error object");
    assert_eq!(res["error"]["code"], -32601);
}

#[test]
fn mcp_invalid_json_returns_parse_error() {
    let spec_path = "tests/mcp.json";
    std::fs::write(spec_path, &minimal_spec_json()).expect("Failed to write test spec");

    let response = send_mcp_request(spec_path, "not json at all");
    let res: serde_json::Value = serde_json::from_str(response.trim())
        .expect("Response should be valid JSON");

    assert!(res["error"].is_object());
    assert_eq!(res["error"]["code"], -32700);
}

#[test]
fn mcp_empty_lines_are_ignored() {
    let spec_path = "tests/mcp.json";
    std::fs::write(spec_path, &minimal_spec_json()).expect("Failed to write test spec");

    let mut child = Command::new(assert_cmd::cargo::cargo_bin("ucp"))
        .args(&["mcp", "--spec", spec_path])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn ucp process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin.write_all(b"\n").unwrap();
        stdin.write_all(b"\n").unwrap();
        stdin.write_all(br#"{"jsonrpc":"2.0","method":"tools/list","id":1}"#).unwrap();
        stdin.write_all(b"\n").unwrap();
    }

    let output = child.wait_with_output().expect("Failed to wait");
    let stdout = String::from_utf8(output.stdout).unwrap();

    let mut found = false;
    for line in stdout.lines() {
        if let Ok(res) = serde_json::from_str::<serde_json::Value>(line) {
            if res.get("result").is_some() && res["result"]["tools"].is_array() {
                found = true;
                break;
            }
        }
    }
    assert!(found, "Should find a valid tools/list response among output lines");
}

#[test]
fn cli_mcp_help() {
    let mut cmd = assert_cmd::Command::cargo_bin("ucp").unwrap();
    cmd.arg("mcp").arg("--help");
    cmd.assert().success().stdout(predicates::str::contains("--spec"));
}
