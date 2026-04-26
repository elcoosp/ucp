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

/// Extract design tokens from a Tailwind config file (JS/TS object).
pub fn extract_tokens_from_tailwind_config(source: &str) -> Result<DtcgTokens> {
    let mut tokens = DtcgTokens {
        colors: std::collections::HashMap::new(),
        spacing: std::collections::HashMap::new(),
        typography: std::collections::HashMap::new(),
    };

    // Very simple regex-free extraction for common Tailwind patterns
    let mut in_colors = false;
    let mut in_spacing = false;
    let mut in_font_family = false;

    for line in source.lines() {
        let trimmed = line.trim();

        if trimmed.contains("colors:") || trimmed.contains("colors:{") {
            in_colors = true;
            continue;
        }
        if trimmed.contains("spacing:") || trimmed.contains("spacing:{") {
            in_spacing = true;
            continue;
        }
        if trimmed.contains("fontFamily:") {
            in_font_family = true;
            continue;
        }

        // Exit sections on closing brace or new top-level key
        if trimmed.starts_with('}') || trimmed.contains("},") {
            in_colors = false;
            in_spacing = false;
            in_font_family = false;
            continue;
        }

        if let Some((key, value)) = trimmed.split_once(':') {
            let key = key
                .trim()
                .trim_matches(|c| c == '\'' || c == '"' || c == ',');
            let value = value
                .trim()
                .trim_matches(|c| c == '\'' || c == '"' || c == ',')
                .to_string();
            if in_colors {
                tokens.colors.insert(key.to_string(), value);
            } else if in_spacing {
                tokens.spacing.insert(key.to_string(), value);
            } else if in_font_family {
                tokens.typography.insert(key.to_string(), value);
            }
        }
    }

    Ok(tokens)
}

/// Extract design tokens from CSS modules (:root custom properties).
pub fn extract_tokens_from_css_modules(source: &str) -> Result<DtcgTokens> {
    // Reuse the existing CSS custom property parser
    extract_tokens_from_source(source)
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

    #[test]
    fn extract_from_tailwind_config() {
        let config = r#"
module.exports = {
  theme: {
    colors: {
      primary: '#ff0000',
    },
    spacing: {
      sm: '0.5rem',
    },
  }
}
"#;
        let tokens = extract_tokens_from_tailwind_config(config).unwrap();
        assert_eq!(tokens.colors.get("primary"), Some(&"#ff0000".to_string()));
        assert_eq!(tokens.spacing.get("sm"), Some(&"0.5rem".to_string()));
    }
}
