use ucp_core::Result;
use super::rust_ast::{RawComponentExtraction, RawPropExtraction};

pub fn extract_svelte_components(source: &str) -> Result<Vec<RawComponentExtraction>> {
    let mut components = Vec::new();
    // Look for <script lang="ts"> blocks with let { ... } = $props()
    if let Some(script_start) = source.find("<script") {
        if let Some(script_end) = source[script_start..].find("</script>") {
            let script = &source[script_start..script_start + script_end];
            // Extract props: let { prop1, prop2 = default } = $props()
            if let Some(props_start) = script.find("$props()") {
                let before = &script[..script_start + props_start];
                // Find the destructuring pattern
                if let Some(brace_start) = before.rfind('{') {
                    if let Some(brace_end) = before[brace_start..].find('}') {
                        let props_str = &before[brace_start + 1..brace_start + brace_end];
                        let props = parse_svelte_props(props_str);
                        if !props.is_empty() {
                            // Find component name from filename or export
                            let name = extract_svelte_component_name(source);
                            components.push(RawComponentExtraction {
                                name: name.unwrap_or_else(|| "SvelteComponent".to_string()),
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
        }
    }
    Ok(components)
}

fn parse_svelte_props(props_str: &str) -> Vec<RawPropExtraction> {
    let mut props = Vec::new();
    for part in props_str.split(',') {
        let part = part.trim();
        if part.is_empty() { continue; }
        let (name, has_default) = if let Some(eq) = part.find('=') {
            (part[..eq].trim().to_string(), true)
        } else {
            (part.to_string(), false)
        };
        // Infer type from name convention or annotation
        let raw_type = if name.contains("disabled") || name.contains("visible") { "boolean".to_string() } else { "string".to_string() };
        props.push(RawPropExtraction {
            name,
            raw_type,
            has_default,
            is_event: name.starts_with("on"),
            is_spread_attributes: name == "$$restProps",
        });
    }
    props
}

fn extract_svelte_component_name(source: &str) -> Option<String> {
    // Simple heuristic: use first word of source
    source.lines().next().and_then(|l| l.split_whitespace().next()).map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_svelte_button() {
        let source = r#"<script lang="ts">
  let { disabled = false, label }: { disabled: boolean; label: string } = $props();
</script>
<button {disabled}>{label}</button>"#;
        let comps = extract_svelte_components(source).unwrap();
        assert!(!comps.is_empty());
        let props = &comps[0].props;
        assert!(props.iter().any(|p| p.name == "disabled" && p.has_default));
        assert!(props.iter().any(|p| p.name == "label"));
    }
}
