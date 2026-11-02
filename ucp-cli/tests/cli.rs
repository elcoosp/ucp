//! CLI behavior tests: honest surface, exit codes, machine-readable output.

use assert_cmd::Command;
use std::path::PathBuf;

fn ucp() -> Command {
    Command::cargo_bin("ucp-cli").expect("ucp-cli binary")
}

fn fixture_spec() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sample-spec.json")
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn help_does_not_mention_ai_engine() {
    let out = ucp().arg("--help").output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.to_lowercase().contains("ai unification engine"),
        "about line must be honest, got: {stdout}"
    );
    assert!(
        stdout.contains("component"),
        "about line must describe the tool, got: {stdout}"
    );
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn buzzword_commands_are_gone() {
    for gone in ["dashboard", "mcp", "mcp-server-json", "contract"] {
        let out = ucp().arg(gone).output().unwrap();
        assert!(!out.status.success(), "`ucp {gone}` must not exist anymore");
    }
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn validate_valid_spec_exits_zero_with_json_summary() {
    let mut cmd = ucp();
    cmd.arg("validate").arg("--json").arg(fixture_spec());
    cmd.assert().success();
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn validate_garbage_file_fails_with_error() {
    let mut cmd = ucp();
    cmd.arg("validate").arg("/dev/null");
    cmd.assert().failure();
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn validate_missing_file_fails() {
    let mut cmd = ucp();
    cmd.arg("validate").arg("/nonexistent/spec.json");
    cmd.assert().failure();
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn bootstrap_accepts_source_dir_and_writes_spec() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("widget.tsx"),
        r"export interface WidgetProps { label: string; }
export const Widget = (props: WidgetProps) => <div>{props.label}</div>;
",
    )
    .unwrap();
    let out_dir = dir.path().join("out");
    let mut cmd = ucp();
    cmd.arg("bootstrap")
        .arg("--source-dir")
        .arg(&src)
        .arg("--output-dir")
        .arg(&out_dir)
        .arg("--json");
    cmd.assert().success();
    assert!(out_dir.join("ucp-spec.json").exists());
}
