use biome_js_parser::parse_module;
use biome_js_syntax::{AnyJsRoot, JsParserOptions};
use ucp_core::Result;

#[derive(Debug, Clone)]
pub struct RawTsxExtraction {
    pub name: String,
    pub props: Vec<RawTsxPropExtraction>,
}

#[derive(Debug, Clone)]
pub struct RawTsxPropExtraction {
    pub name: String,
    pub raw_type: String,
    pub is_optional: bool,
}

pub fn extract_tsx_components(code: &str) -> Result<Vec<RawTsxExtraction>> {
    let parsed = parse_module(code, JsParserOptions::default())
        .map_err(|e| ucp_core::UcpError::Parsing(format!("TSX error: {}", e)))?;

    let mut components = Vec::new();

    for item in parsed.syntax().children() {
        if item.to_string().contains("export const") {
            // Very basic heuristic for arrow functions in TSX
            let text = item.to_string();
            if text.contains("=>") {
                // Try to isolate the function name
                if let Some(name) = text.split("export const ").nth(1) {
                    let name = name.split_whitespace().next().unwrap_or("");
                    let clean_name = name.split(|c:char| c.is_ascii_alphanumeric() || c == '_').collect();
                    components.push(RawTsxExtraction {
                        name: clean_name,
                        props: vec![RawTsxPropExtraction { name: "props".to_string(), raw_type: "Interface".to_string(), is_optional: false }],
                    });
                }
            }
        }
    }
    Ok(components)
}
