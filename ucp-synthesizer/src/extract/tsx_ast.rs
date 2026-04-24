use ucp_core::Result;

#[derive(Debug, Clone)]
pub struct RawTsxExtraction {
    pub name: String,
    pub line_start: usize,
    pub props: Vec<RawTsxPropExtraction>,
}

#[derive(Debug, Clone)]
pub struct RawTsxPropExtraction {
    pub name: String,
    pub raw_type: String,
    pub is_optional: bool,
}

/// Extract components from TSX/TS source code.
///
/// Supports:
/// - `export interface FooProps { ... }` and `export type FooProps = { ... }`
/// - Multi-line prop types via brace-depth tracking
///   (e.g. `data: Array<{ id: string; label: string }>`)
/// - Nested object types (e.g. `theme: { primary: string }`)
/// - Generic interfaces (e.g. `interface TableProps<T> { ... }`)
/// - Both `export const X =` and `export function X(` component forms
pub fn extract_tsx_components(code: &str) -> Result<Vec<RawTsxExtraction>> {
    let mut components = Vec::new();
    let mut current_props: Vec<RawTsxPropExtraction> = Vec::new();
    let mut in_block = false;
    let mut brace_depth = 0usize;
    let mut block_buffer = String::new();
    let mut current_line = 0usize;

    for line in code.lines() {
        current_line += 1;
        let trimmed = line.trim();

        // --- Detect start of interface/type block ---
        if !in_block {
            if is_props_declaration(trimmed) {
                in_block = true;
                brace_depth = 0;
                block_buffer.clear();
                current_props.clear();
            }
        }

        // --- Accumulate block content with brace tracking ---
        if in_block {
            block_buffer.push_str(trimmed);
            block_buffer.push('\n');

            for ch in trimmed.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => {
                        brace_depth -= 1;
                        if brace_depth == 0 {
                            // Block complete — parse props from the body
                            in_block = false;
                            if let Some(body) = extract_interface_body(&block_buffer) {
                                parse_props_from_body(&body, &mut current_props);
                            }
                            break;
                        }
                    }
                    _ => {}
                }
            }
            continue;
        }

        // --- Detect component export (const or function) ---
        if is_component_export(trimmed) {
            if let Some(name) = extract_component_name(trimmed) {
                if !name.is_empty() {
                    components.push(RawTsxExtraction {
                        name: name.to_string(),
                        line_start: current_line,
                        props: current_props.clone(),
                    });
                    current_props.clear();
                }
            }
        }
    }

    Ok(components)
}

/// Check if a line starts a Props interface or type alias that contains a
/// brace-delimited body (skip union types like `type X = "a" | "b"`).
fn is_props_declaration(line: &str) -> bool {
    let is_interface = line.starts_with("export interface") || line.starts_with("interface");
    let is_type = line.starts_with("export type") || line.starts_with("type");

    (is_interface || is_type) && line.contains("Props") && line.contains('{')
}

/// Check if a line is a component export (const arrow or function).
fn is_component_export(line: &str) -> bool {
    let is_const = line.starts_with("export const") || line.starts_with("const");
    let is_fn = line.starts_with("export function") || line.starts_with("function");

    (is_const && line.contains("=>")) || (is_fn && line.contains('('))
}

/// Extract everything between the first `{` and the last `}` in a block.
fn extract_interface_body(block: &str) -> Option<String> {
    let first = block.find('{')?;
    let last = block.rfind('}')?;
    if first >= last {
        return None;
    }
    Some(block[first + 1..last].to_string())
}

/// Split the interface body by `;` and `,` at depth 0 (respecting nested
/// braces and angle brackets), then parse each segment as a prop.
fn parse_props_from_body(body: &str, props: &mut Vec<RawTsxPropExtraction>) {
    let mut depth = 0usize;   // brace depth
    let mut angle = 0i32;     // < > depth
    let mut current = String::new();

    for ch in body.chars() {
        match ch {
            '{' => {
                depth += 1;
                current.push(ch);
            }
            '}' => {
                depth -= 1;
                current.push(ch);
            }
            '<' => {
                angle += 1;
                current.push(ch);
            }
            '>' => {
                angle -= 1;
                current.push(ch);
            }
            ';' | ',' if depth == 0 && angle == 0 => {
                parse_single_prop(current.trim(), props);
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    // Handle last prop without trailing separator
    if !current.trim().is_empty() {
        parse_single_prop(current.trim(), props);
    }
}

/// Parse a single prop declaration like `name?: Type` or `readonly name: Type`.
fn parse_single_prop(segment: &str, props: &mut Vec<RawTsxPropExtraction>) {
    let seg = segment.trim();

    // Skip empty segments, line comments, block comment starts
    if seg.is_empty() || seg.starts_with("//") || seg.starts_with("/*") || seg.starts_with("*") {
        return;
    }

    // Strip `readonly` keyword
    let seg = seg.strip_prefix("readonly").unwrap_or(seg).trim();

    // Find the colon separating name from type, respecting nested delimiters
    let colon_pos = match find_prop_colon(seg) {
        Some(pos) => pos,
        None => return,
    };

    let name_part = &seg[..colon_pos];
    let type_part = seg[colon_pos + 1..].trim();

    let prop_name = name_part.trim().trim_end_matches('?');
    if prop_name.is_empty() {
        return;
    }
    // Props must start with an alphabetic char or underscore
    if !prop_name
        .chars()
        .next()
        .map_or(false, |c| c.is_alphabetic() || c == '_')
    {
        return;
    }

    let is_optional = name_part.trim().ends_with('?');

    props.push(RawTsxPropExtraction {
        name: prop_name.to_string(),
        raw_type: type_part.to_string(),
        is_optional,
    });
}

/// Find the first `:` that is not inside `< >`, `( )`, or `[ ]` delimiters.
fn find_prop_colon(s: &str) -> Option<usize> {
    let mut angle = 0i32;
    let mut paren = 0i32;
    let mut bracket = 0i32;

    for (i, ch) in s.char_indices() {
        match ch {
            '<' => angle += 1,
            '>' => angle -= 1,
            '(' => paren += 1,
            ')' => paren -= 1,
            '[' => bracket += 1,
            ']' => bracket -= 1,
            ':' if angle == 0 && paren == 0 && bracket == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

/// Extract the component name from `export const Name = ...` or
/// `export function Name(...)` lines.
fn extract_component_name(line: &str) -> Option<&str> {
    let rest = line
        .strip_prefix("export const")
        .or_else(|| line.strip_prefix("const"))
        .or_else(|| line.strip_prefix("export function"))
        .or_else(|| line.strip_prefix("function"))?;

    rest.trim()
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .next()
        .filter(|n| !n.is_empty())
}
