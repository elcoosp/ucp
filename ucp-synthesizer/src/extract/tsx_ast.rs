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

pub fn extract_tsx_components(code: &str) -> Result<Vec<RawTsxExtraction>> {
    let mut components = Vec::new();
    let mut current_props: Vec<RawTsxPropExtraction> = Vec::new();
    let mut in_interface = false;
    let mut current_line = 0usize;

    for line in code.lines() {
        current_line += 1;
        let trimmed = line.trim();

        if trimmed.starts_with("export interface") && trimmed.contains("Props") {
            in_interface = true;
            current_props.clear();
            continue;
        }

        if in_interface {
            if trimmed.starts_with("}") {
                in_interface = false;
                continue;
            }
            if let Some(colon_pos) = trimmed.find(':') {
                let prop_part = &trimmed[..colon_pos];
                let prop_name = prop_part.trim().trim_end_matches('?');
                if !prop_name.is_empty()
                    && prop_name
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_alphabetic() || c == '_')
                {
                    let is_optional = prop_part.trim().ends_with('?');
                    let type_part = trimmed[colon_pos + 1..]
                        .trim()
                        .trim_end_matches(';')
                        .trim()
                        .to_string();
                    current_props.push(RawTsxPropExtraction {
                        name: prop_name.to_string(),
                        raw_type: type_part,
                        is_optional,
                    });
                }
            }
            continue;
        }

        if trimmed.starts_with("export const") && trimmed.contains("=>") {
            if let Some(name_part) = trimmed.strip_prefix("export const") {
                let name = name_part
                    .trim()
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
                    .unwrap_or("");
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
