use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ucp_core::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtcgTokens {
    pub colors: HashMap<String, String>,
    pub spacing: HashMap<String, String>,
    pub typography: HashMap<String, String>,
}

/// Extract design tokens from CSS custom properties and Tailwind config.
pub fn extract_tokens_from_source(source: &str) -> Result<DtcgTokens> {
    let mut tokens = DtcgTokens {
        colors: HashMap::new(),
        spacing: HashMap::new(),
        typography: HashMap::new(),
    };
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("--") {
            if let Some((key, value)) = rest.split_once(':') {
                let key = format!("--{}", key.trim());
                let value = value.trim().trim_end_matches(';').trim().to_string();
                if key.starts_with("--color")
                    || key.starts_with("--primary")
                    || key.starts_with("--secondary")
                    || key.starts_with("--accent")
                    || key.starts_with("--destructive")
                    || key.starts_with("--muted")
                    || key.starts_with("--background")
                    || key.starts_with("--foreground")
                    || key.starts_with("--chart")
                    || key.starts_with("--sidebar")
                    || key.starts_with("--ring")
                    || key.starts_with("--input")
                    || key.starts_with("--border")
                {
                    tokens.colors.insert(key, value);
                } else if key.starts_with("--space") || key.starts_with("--spacing") {
                    tokens.spacing.insert(key, value);
                } else if key.starts_with("--font")
                    || key.starts_with("--text")
                    || key.starts_with("--leading")
                    || key.starts_with("--tracking")
                {
                    tokens.typography.insert(key, value);
                }
            }
        }
    }
    Ok(tokens)
}

/// Export tokens as DTCG-compliant JSON.
pub fn export_tokens_to_dtcg(tokens: &DtcgTokens, output_path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(tokens).map_err(ucp_core::UcpError::Json)?;
    std::fs::write(output_path, json).map_err(ucp_core::UcpError::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_colors_from_css() {
        let source = r#"
        --primary: oklch(0.645 0.246 16.439);
        --background: oklch(1 0 0);
        --font-size: 1rem;
        "#;
        let tokens = extract_tokens_from_source(source).unwrap();
        assert!(tokens.colors.contains_key("--primary"));
        assert!(tokens.colors.contains_key("--background"));
        assert!(tokens.typography.contains_key("--font-size"));
    }
}
