use ucp_core::Result;
use super::rust_ast::{RawComponentExtraction, RawPropExtraction};

pub fn extract_vue_components(source: &str) -> Result<Vec<RawComponentExtraction>> {
    let mut components = Vec::new();
    // Look for <script setup lang="ts"> blocks
    if let Some(script_start) = source.find("<script setup") {
        if let Some(script_end) = source[script_start..].find("</script>") {
            let script = &source[script_start..script_start + script_end];
            // Extract defineProps<{ ... }>()
            if let Some(props_start) = script.find("defineProps<{") {
                let inner = &script[props_start + "defineProps<{".len()..];
                if let Some(props_end) = inner.find("}>()") {
                    let props_str = &inner[..props_end];
                    let props = parse_vue_props(props_str);
                    // Extract defineEmits<{ ... }>()
                    let events = extract_vue_events(script);
                    let name = extract_vue_component_name(source);
                    components.push(RawComponentExtraction {
                        name: name.unwrap_or_else(|| "VueComponent".to_string()),
                        line_start: 1,
                        props,
                        is_struct_pattern: false,
                        provided_context: None,
                        consumed_contexts: vec![],
                    });
                }
            }
        }
    }
    Ok(components)
}

fn parse_vue_props(props_str: &str) -> Vec<RawPropExtraction> {
    let mut props = Vec::new();
    for part in props_str.split(';') {
        let part = part.trim();
        if part.is_empty() { continue; }
        let has_default = part.contains('?');
        let name_part = part.split(':').next().unwrap_or(part).trim();
        let name = name_part.trim_end_matches('?');
        let raw_type = part.split(':').nth(1).unwrap_or("any").trim().to_string();
        props.push(RawPropExtraction {
            name: name.to_string(),
            raw_type,
            has_default,
            is_event: false,
            is_spread_attributes: false,
        });
    }
    props
}

fn extract_vue_events(script: &str) -> Vec<String> {
    if let Some(emits_start) = script.find("defineEmits<{") {
        let inner = &script[emits_start + "defineEmits<{".len()..];
        if let Some(emits_end) = inner.find("}>()") {
            return inner[..emits_end].split(',').map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string()).collect();
        }
    }
    vec![]
}

fn extract_vue_component_name(source: &str) -> Option<String> {
    source.lines().next().and_then(|l| l.split_whitespace().next()).map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_vue_button() {
        let source = r#"<script setup lang="ts">
defineProps<{ disabled?: boolean; label: string }>();
defineEmits<{ click: [] }>();
</script>
<template><button :disabled="disabled" @click="$emit('click')">{{ label }}</button></template>"#;
        let comps = extract_vue_components(source).unwrap();
        assert!(!comps.is_empty());
        let props = &comps[0].props;
        assert!(props.iter().any(|p| p.name == "disabled" && p.has_default));
        assert!(props.iter().any(|p| p.name == "label" && !p.has_default));
    }
}
