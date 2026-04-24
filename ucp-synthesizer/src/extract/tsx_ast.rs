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
/// - Nested object types and generic interfaces
/// - `export const X =` and `export function X(` component forms
/// - `export default function X(` named default exports
/// - `const X: React.FC<XProps> =` and `const X: FC<XProps> =` forms
/// - `class X extends React.Component<XProps>` class components
pub fn extract_tsx_components(code: &str) -> Result<Vec<RawTsxExtraction>> {
    let mut components = Vec::new();
    let mut current_props: Vec<RawTsxPropExtraction> = Vec::new();
    let mut in_block = false;
    let mut brace_depth = 0usize;
    let mut block_buffer = String::new();

    for (current_line, line) in code.lines().enumerate() {
        let trimmed = line.trim();

        // --- Detect start of interface/type block ---
        if !in_block && is_props_declaration(trimmed) {
            in_block = true;
            brace_depth = 0;
            block_buffer.clear();
            current_props.clear();
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

        // --- Detect component export ---
        if is_component_export(trimmed) {
            if let Some(name) = extract_component_name(trimmed) {
                if !name.is_empty() {
                    components.push(RawTsxExtraction {
                        name: name.to_string(),
                        line_start: current_line + 1,
                        props: current_props.clone(),
                    });
                    current_props.clear();
                }
            }
        }
    }

    Ok(components)
}

/// Check if a line starts a Props interface or type alias with braces.
fn is_props_declaration(line: &str) -> bool {
    let is_interface = line.starts_with("export interface") || line.starts_with("interface");
    let is_type = line.starts_with("export type") || line.starts_with("type");

    (is_interface || is_type) && line.contains("Props") && line.contains('{')
}

/// Check if a line is a component export.
fn is_component_export(line: &str) -> bool {
    let is_const = line.starts_with("export const") || line.starts_with("const");
    let is_fn = line.starts_with("export function") || line.starts_with("function");
    let is_default = line.starts_with("export default");

    (is_const && line.contains("=>"))
        || (is_fn && line.contains('('))
        || (is_default && (line.contains("=>") || line.contains('(')))
        || is_react_fc_declaration(line)
        || is_class_component(line)
}

/// Check for `const X: React.FC<Props> =` or `const X: FC<Props> =`.
fn is_react_fc_declaration(line: &str) -> bool {
    let stripped = line
        .strip_prefix("export ")
        .unwrap_or(line)
        .strip_prefix("const ")
        .unwrap_or("");

    if !stripped.contains(':') || !stripped.contains('=') {
        return false;
    }

    let before_colon = stripped.split(':').next().unwrap_or("").trim();
    let after_colon = stripped.split(':').nth(1).unwrap_or("");

    // Name must be a valid identifier
    if before_colon.is_empty()
        || !before_colon
            .chars()
            .next()
            .is_some_and(|c| c.is_alphabetic() || c == '_')
    {
        return false;
    }

    // Type side must contain React.FC, FC, React.PureComponent, etc.
    let type_part = after_colon.split('=').next().unwrap_or("").trim();
    type_part.contains("React.FC")
        || type_part.contains("React.PureComponent")
        || (type_part.starts_with("FC<") || type_part.starts_with("FC <"))
        || (type_part.starts_with("PureComponent<") || type_part.starts_with("PureComponent <"))
}

/// Check for `class X extends React.Component<Props>` patterns.
fn is_class_component(line: &str) -> bool {
    let stripped = line.strip_prefix("export ").unwrap_or(line);

    if !stripped.starts_with("class ") {
        return false;
    }

    stripped.contains("extends")
        && (stripped.contains("React.Component")
            || stripped.contains("React.PureComponent")
            || stripped.contains("Component<"))
}

/// Extract the component name from various export forms.
fn extract_component_name(line: &str) -> Option<&str> {
    // Named default function
    if line.starts_with("export default function") {
        let rest = line.strip_prefix("export default function")?;
        return rest
            .trim()
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .next()
            .filter(|n| !n.is_empty());
    }

    // Named default class
    if line.starts_with("export default class") {
        let rest = line.strip_prefix("export default class")?;
        return rest
            .trim()
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .next()
            .filter(|n| !n.is_empty());
    }

    // Class component: `class X extends ...`
    if line.starts_with("class ") || line.starts_with("export class ") {
        let rest = line
            .strip_prefix("export class ")
            .or_else(|| line.strip_prefix("class "))?;
        return rest
            .trim()
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .next()
            .filter(|n| !n.is_empty());
    }

    // React.FC: `const X: React.FC<Props> =`
    if is_react_fc_declaration(line) {
        let stripped = line
            .strip_prefix("export ")
            .unwrap_or(line)
            .strip_prefix("const ")
            .unwrap_or("");
        return stripped
            .split(':')
            .next()
            .map(|s| s.trim())
            .filter(|n| !n.is_empty());
    }

    // Regular const / function
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

fn extract_interface_body(block: &str) -> Option<String> {
    let first = block.find('{')?;
    let last = block.rfind('}')?;
    if first >= last {
        return None;
    }
    Some(block[first + 1..last].to_string())
}

fn parse_props_from_body(body: &str, props: &mut Vec<RawTsxPropExtraction>) {
    let mut depth = 0usize;
    let mut angle = 0i32;
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

    if !current.trim().is_empty() {
        parse_single_prop(current.trim(), props);
    }
}

fn parse_single_prop(segment: &str, props: &mut Vec<RawTsxPropExtraction>) {
    let seg = segment.trim();

    if seg.is_empty() || seg.starts_with("//") || seg.starts_with("/*") || seg.starts_with("*") {
        return;
    }

    let seg = seg.strip_prefix("readonly").unwrap_or(seg).trim();

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
    if !prop_name
        .chars()
        .next()
        .is_some_and(|c| c.is_alphabetic() || c == '_')
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
