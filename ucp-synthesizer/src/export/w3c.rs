use std::fs;
use std::path::Path;
use serde::Serialize;
use ucp_core::cam::*;
use ucp_core::Result;
use crate::pipeline::SynthesisOutput;

#[derive(Serialize)]
struct W3cUiSpec {
    schema: String,
    components: Vec<W3cComponent>,
}

#[derive(Serialize)]
struct W3cComponent {
    id: String,
    name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    props: Vec<W3cProp>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    events: Vec<W3cEvent>,
}

#[derive(Serialize)]
struct W3cProp {
    name: String,
    #[serde(rename = "type")]
    prop_type: String,
    required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Serialize)]
struct W3cEvent {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<String>,
}

pub fn export_w3c(spec: &SynthesisOutput, output_dir: &str) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    let mut components = Vec::new();
    for comp in &spec.components {
        let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let w3c_comp = W3cComponent {
            id: comp.id.clone(),
            name: name.to_string(),
            props: comp.props.iter().map(|p| W3cProp {
                name: p.canonical_name.clone(),
                prop_type: format!("{:?}", p.abstract_type).to_lowercase(),
                required: p.reactivity != AbstractReactivity::Static,
                description: p.concrete_type.clone(),
            }).collect(),
            events: comp.events.iter().map(|e| W3cEvent {
                name: e.canonical_name.clone(),
                payload: Some(format!("{:?}", e.abstract_payload)),
            }).collect(),
        };
        components.push(w3c_comp);
    }

    let w3c_spec = W3cUiSpec {
        schema: "https://www.w3.org/community/uispec/draft".to_string(),
        components,
    };

    let json = serde_json::to_string_pretty(&w3c_spec).map_err(ucp_core::UcpError::Json)?;
    fs::write(dir.join("ucp-spec.w3c.json"), json).map_err(ucp_core::UcpError::Io)?;
    Ok(())
}
