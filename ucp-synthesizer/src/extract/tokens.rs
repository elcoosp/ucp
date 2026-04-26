use std::collections::HashMap;
use ucp_core::Result;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DesignTokens {
    pub colors: HashMap<String, String>,
    pub spacing: HashMap<String, String>,
    pub typography: HashMap<String, String>,
}

/// Extract CSS custom properties and Tailwind theme tokens from source.
pub fn extract_tokens_from_source(source: &str) -> Result<DesignTokens> {
    let mut tokens = DesignTokens {
        colors: HashMap::new(),
        spacing: HashMap::new(),
        typography: HashMap::new(),
    };

    for line in source.lines() {
        let trimmed = line.trim();

        // CSS custom property: --color-primary: oklch(...);
        if let Some(rest) = trimmed.strip_prefix("--") {
            if let Some((key, value)) = rest.split_once(':') {
                let key = key.trim().to_string();
                let value = value.trim().trim_end_matches(';').trim().to_string();
                if key.starts_with("color")
                    || key.starts_with("primary")
                    || key.starts_with("secondary")
                    || key.starts_with("accent")
                    || key.starts_with("destructive")
                    || key.starts_with("muted")
                    || key.starts_with("border")
                    || key.starts_with("input")
                    || key.starts_with("ring")
                    || key.starts_with("background")
                    || key.starts_with("foreground")
                    || key.starts_with("chart")
                    || key.starts_with("sidebar")
                {
                    tokens.colors.insert(format!("--{}", key), value);
                } else if key.starts_with("spacing") || key.starts_with("space") {
                    tokens.spacing.insert(format!("--{}", key), value);
                } else if key.starts_with("font")
                    || key.starts_with("text")
                    || key.starts_with("leading")
                    || key.starts_with("tracking")
                {
                    tokens.typography.insert(format!("--{}", key), value);
                }
            }
        }
    }

    Ok(tokens)
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
