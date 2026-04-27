use ucp_maintainer::tokens::{merge_token_files, TokenMergeOptions};
use ucp_synthesizer::extract::tokens::DtcgTokens;
use std::collections::HashMap;

fn make_tokens(primary: &str, background: &str) -> DtcgTokens {
    let mut colors = HashMap::new();
    colors.insert("--primary".into(), primary.into());
    colors.insert("--background".into(), background.into());
    DtcgTokens {
        colors,
        spacing: HashMap::new(),
        typography: HashMap::new(),
    }
}

#[test]
fn integration_merge_two_files_no_conflict() {
    let a = make_tokens("#ff0000", "#ffffff");
    let b = make_tokens("#ff0000", "#ffffff");
    let result = merge_token_files(
        &[("a.json".into(), a), ("b.json".into(), b)],
        &TokenMergeOptions::default(),
    ).unwrap();
    assert!(result.conflicts.is_empty());
    assert_eq!(result.merged.colors.len(), 2);
}

#[test]
fn integration_merge_conflict_first_wins_force() {
    let a = make_tokens("#ff0000", "#ffffff");
    let b = make_tokens("#0000ff", "#000000");
    let opts = TokenMergeOptions { strategy: "first-wins".into(), force: true };
    let result = merge_token_files(
        &[("a.json".into(), a), ("b.json".into(), b)],
        &opts,
    ).unwrap();
    assert_eq!(result.conflicts.len(), 2); // both keys conflict
    assert_eq!(result.merged.colors.get("--primary").unwrap(), "#ff0000");
    assert_eq!(result.merged.colors.get("--background").unwrap(), "#ffffff");
}

#[test]
fn integration_detects_conflict_on_partial_overlap() {
    let a = make_tokens("#ff0000", "#ffffff");
    let mut b_tokens = make_tokens("#ff0000", "#eeeeee");
    // b has an extra spacing token, a doesn't
    b_tokens.spacing.insert("--gap".into(), "8px".into());
    let result = merge_token_files(
        &[("a.json".into(), a), ("b.json".into(), b_tokens)],
        &TokenMergeOptions::default(),
    );
    // Without --force and with default "error" strategy, conflicting values should error
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("1 conflict"));
}

#[test]
fn integration_partial_overlap_resolved_with_force() {
    let a = make_tokens("#ff0000", "#ffffff");
    let mut b_tokens = make_tokens("#ff0000", "#eeeeee");
    b_tokens.spacing.insert("--gap".into(), "8px".into());
    let opts = TokenMergeOptions { strategy: "first-wins".into(), force: true };
    let result = merge_token_files(
        &[("a.json".into(), a), ("b.json".into(), b_tokens)],
        &opts,
    ).unwrap();
    // --background has a conflict, but it's resolved with first-wins
    assert_eq!(result.conflicts.len(), 1);
    assert_eq!(result.merged.colors.get("--background").unwrap(), "#ffffff");
    // --gap was only in b, so it carries over
    assert_eq!(result.merged.spacing.get("--gap").unwrap(), "8px");
}
