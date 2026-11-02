use super::rust_ast::{RawComponentExtraction, RawPropExtraction};
use ucp_core::Result;

/// Extract raw components from a Svelte source file.
///
/// Recognizes the Svelte 5 runes pattern
/// `let { prop = default, other }: Type = $props()` inside the top-level
/// `<script>` block. The destructuring pattern is located by brace
/// matching, so `: { ... }` type annotations after the pattern are
/// ignored rather than mistaken for the pattern itself.
///
/// # Errors
///
/// Never errors in practice; the signature keeps the extractor trait
/// uniform with the AST-based extractors. Returned as `Ok` always.
pub fn extract_svelte_components(source: &str) -> Result<Vec<RawComponentExtraction>> {
    let mut components = Vec::new();
    if let Some(script_start) = source.find("<script") {
        if let Some(script_rel_end) = source[script_start..].find("</script>") {
            let script_end = script_start + script_rel_end;
            let script = &source[script_start..script_end + "</script>".len()];
            if let Some(props) = parse_svelte_props_declaration(script) {
                if !props.is_empty() {
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
    Ok(components)
}

/// Parse the `let { ... } = $props()` binding out of a script block.
///
/// Returns the raw props declared by the destructuring pattern, or `None`
/// when the script has no `$props()` destructuring at all.
fn parse_svelte_props_declaration(script: &str) -> Option<Vec<RawPropExtraction>> {
    let props_pos = script.find("$props()")?;
    // The `=` binding the pattern to `$props()` is the last one before it.
    let before = &script[..props_pos];
    let eq_pos = before.rfind('=')?;
    let binding = &before[..eq_pos];
    // The destructure pattern is the first balanced `{ ... }` in the
    // binding; a trailing `: { ... }` type annotation comes after the
    // pattern and is therefore excluded by brace matching.
    let open_rel = binding.find('{')?;
    let mut depth = 0usize;
    let mut close_rel = None;
    for (i, ch) in binding[open_rel..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    close_rel = Some(open_rel + i);
                    break;
                }
            }
            _ => {}
        }
    }
    let close_rel = close_rel?;
    let pattern = &binding[open_rel + 1..close_rel];
    Some(parse_svelte_props(pattern))
}

fn parse_svelte_props(props_str: &str) -> Vec<RawPropExtraction> {
    let mut props = Vec::new();
    for part in props_str.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (name, has_default) = if let Some(eq) = part.find('=') {
            (part[..eq].trim().to_string(), true)
        } else {
            (part.to_string(), false)
        };
        let raw_type = if name.contains("disabled") || name.contains("visible") {
            "boolean".to_string()
        } else {
            "string".to_string()
        };
        let is_event = name.starts_with("on");
        let is_spread = name == "$$restProps";
        props.push(RawPropExtraction {
            name,
            raw_type,
            has_default,
            is_event,
            is_spread_attributes: is_spread,
        });
    }
    props
}

fn extract_svelte_component_name(source: &str) -> Option<String> {
    source
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().next())
        .map(std::string::ToString::to_string)
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
        assert!(
            props.iter().any(|p| p.name == "disabled"),
            "should have disabled prop"
        );
        assert!(
            props.iter().any(|p| p.name == "label"),
            "should have label prop"
        );
    }

    #[test]
    fn svelte_without_props_yields_no_components() {
        let source = "<script>const x = 1;</script><p>hi</p>";
        let comps = extract_svelte_components(source).unwrap();
        assert!(comps.is_empty());
    }
}
