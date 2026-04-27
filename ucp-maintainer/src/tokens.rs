use std::collections::HashMap;
use ucp_core::Result;
use ucp_synthesizer::extract::tokens::DtcgTokens;

/// Options for token merging.
#[derive(Debug, Clone)]
pub struct TokenMergeOptions {
    /// Strategy when a token key appears in multiple sources:
    /// - "error": fail with a detailed conflict report (default)
    /// - "first-wins": keep the value from the first file that defines it
    /// - "last-wins": keep the value from the last file that defines it
    pub strategy: String,
    /// If true, produce output even when conflicts exist, using the chosen strategy.
    pub force: bool,
}

impl Default for TokenMergeOptions {
    fn default() -> Self {
        Self {
            strategy: "error".into(),
            force: false,
        }
    }
}

/// Result of a token merge operation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TokenMergeResult {
    pub merged: DtcgTokens,
    pub conflicts: Vec<TokenConflict>,
}

/// A token conflict: same key, different values from different sources.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TokenConflict {
    pub key: String,
    pub values: Vec<(String, String)>,   // (source_file, value)
}

/// Merge multiple `DtcgTokens` into a single token set.
///
/// Returns an error if conflicts are found and `options.force` is false.
pub fn merge_token_files(
    files: &[(String, DtcgTokens)],
    options: &TokenMergeOptions,
) -> Result<TokenMergeResult> {
    let mut colors: HashMap<String, Vec<(usize, &str)>> = HashMap::new();
    let mut spacing: HashMap<String, Vec<(usize, &str)>> = HashMap::new();
    let mut typography: HashMap<String, Vec<(usize, &str)>> = HashMap::new();

    for (idx, (_path, tokens)) in files.iter().enumerate() {
        for (key, value) in &tokens.colors {
            colors.entry(key.clone()).or_default().push((idx, value));
        }
        for (key, value) in &tokens.spacing {
            spacing.entry(key.clone()).or_default().push((idx, value));
        }
        for (key, value) in &tokens.typography {
            typography.entry(key.clone()).or_default().push((idx, value));
        }
    }

    let mut conflicts = Vec::new();
    let mut merged_colors = HashMap::new();
    let mut merged_spacing = HashMap::new();
    let mut merged_typography = HashMap::new();

    // Helper to resolve conflicts for one category
    fn resolve_category(
        entries: HashMap<String, Vec<(usize, &str)>>,
        files: &[(String, DtcgTokens)],
        strategy: &str,
        force: bool,
        conflicts: &mut Vec<TokenConflict>,
    ) -> Result<HashMap<String, String>> {
        let mut merged = HashMap::new();

        for (key, occurrences) in entries {
            // Determine if there's a conflict (different values)
            let values: Vec<&str> = occurrences.iter().map(|(_, v)| *v).collect();
            let first_val = values[0];
            let all_same = values.iter().all(|v| *v == first_val);

            if all_same {
                merged.insert(key, first_val.to_string());
            } else {
                // Conflict detected
                let conflict = TokenConflict {
                    key: key.clone(),
                    values: occurrences
                        .iter()
                        .map(|(idx, val)| (files[*idx].0.clone(), val.to_string()))
                        .collect(),
                };

                if !force {
                    conflicts.push(conflict);
                    // Don't stop; collect all conflicts first
                } else {
                    // Apply strategy
                    let chosen = match strategy {
                        "first-wins" => occurrences[0].1.to_string(),
                        "last-wins" => occurrences.last().unwrap().1.to_string(),
                        // For "error" with force we still need a choice; fall back to first-wins
                        _ => occurrences[0].1.to_string(),
                    };
                    merged.insert(key, chosen);
                    conflicts.push(conflict); // Still record the conflict for reporting
                }
            }
        }

        Ok(merged)
    }

    merged_colors = resolve_category(colors, files, &options.strategy, options.force, &mut conflicts)?;
    merged_spacing = resolve_category(spacing, files, &options.strategy, options.force, &mut conflicts)?;
    merged_typography = resolve_category(typography, files, &options.strategy, options.force, &mut conflicts)?;

    if !conflicts.is_empty() && !options.force {
        return Err(ucp_core::UcpError::Conflict(format!(
            "Token merge has {} conflict(s). Use --force or --strategy to resolve.",
            conflicts.len()
        )));
    }

    Ok(TokenMergeResult {
        merged: DtcgTokens {
            colors: merged_colors,
            spacing: merged_spacing,
            typography: merged_typography,
        },
        conflicts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tokens(primary_color: &str) -> DtcgTokens {
        let mut colors = HashMap::new();
        colors.insert("--primary".into(), primary_color.into());
        DtcgTokens {
            colors,
            spacing: HashMap::new(),
            typography: HashMap::new(),
        }
    }

    #[test]
    fn merge_identical_tokens_no_conflict() {
        let a = make_tokens("#ff0000");
        let b = make_tokens("#ff0000");
        let opts = TokenMergeOptions::default();
        let result = merge_token_files(
            &[("a.json".into(), a), ("b.json".into(), b)],
            &opts,
        ).unwrap();
        assert_eq!(result.conflicts.len(), 0);
        assert_eq!(result.merged.colors.get("--primary").unwrap(), "#ff0000");
    }

    #[test]
    fn merge_conflicting_tokens_error_without_force() {
        let a = make_tokens("#ff0000");
        let b = make_tokens("#0000ff");
        let opts = TokenMergeOptions::default(); // strategy = "error", force = false
        let result = merge_token_files(
            &[("a.json".into(), a), ("b.json".into(), b)],
            &opts,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("conflict"));
    }

    #[test]
    fn merge_conflicting_tokens_first_wins_with_force() {
        let a = make_tokens("#ff0000");
        let b = make_tokens("#0000ff");
        let opts = TokenMergeOptions {
            strategy: "first-wins".into(),
            force: true,
        };
        let result = merge_token_files(
            &[("a.json".into(), a), ("b.json".into(), b)],
            &opts,
        ).unwrap();
        assert_eq!(result.merged.colors.get("--primary").unwrap(), "#ff0000");
        assert_eq!(result.conflicts.len(), 1);
    }

    #[test]
    fn merge_conflicting_tokens_last_wins_with_force() {
        let a = make_tokens("#ff0000");
        let b = make_tokens("#0000ff");
        let opts = TokenMergeOptions {
            strategy: "last-wins".into(),
            force: true,
        };
        let result = merge_token_files(
            &[("a.json".into(), a), ("b.json".into(), b)],
            &opts,
        ).unwrap();
        assert_eq!(result.merged.colors.get("--primary").unwrap(), "#0000ff");
        assert_eq!(result.conflicts.len(), 1);
    }
}
