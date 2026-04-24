use std::collections::BTreeMap;

mod parser;

/// Parsed SMDL component with typed fields.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmdlComponent {
    pub id: String,
    pub initial: String,
    pub states: BTreeMap<String, SmdlState>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmdlState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on: Option<BTreeMap<String, SmdlTransition>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmdlTransition {
    pub target: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub side_effects: Vec<String>,
}

/// Parse an SMDL string into a typed `SmdlComponent`.
pub fn parse_smdl(input: &str) -> crate::Result<SmdlComponent> {
    parser::parse_smdl_internal(input)
}
