use ucp_synthesizer::extract::tokens::{
    extract_tokens_from_css_modules, extract_tokens_from_tailwind_config,
};

#[test]
fn tailwind_config_extracts_colors_and_spacing() {
    let config = r#"
module.exports = {
  theme: {
    colors: {
      primary: '#ff0000',
      secondary: '#00ff00',
    },
    spacing: {
      sm: '0.5rem',
      md: '1rem',
    },
  }
}
"#;
    let tokens = extract_tokens_from_tailwind_config(config).unwrap();
    assert_eq!(tokens.colors.get("primary"), Some(&"#ff0000".to_string()));
    assert_eq!(tokens.colors.get("secondary"), Some(&"#00ff00".to_string()));
    assert_eq!(tokens.spacing.get("sm"), Some(&"0.5rem".to_string()));
    assert_eq!(tokens.spacing.get("md"), Some(&"1rem".to_string()));
}

#[test]
fn css_modules_extracts_custom_properties() {
    let css = r#"
:root {
    --primary: oklch(0.645 0.246 16.439);
    --background: oklch(1 0 0);
    --font-size-base: 1rem;
}
"#;
    let tokens = extract_tokens_from_css_modules(css).unwrap();
    assert!(tokens.colors.contains_key("--primary"));
    assert!(tokens.colors.contains_key("--background"));
    assert!(tokens.typography.contains_key("--font-size-base"));
}
