use std::fs;
use std::path::Path;
use serde::Serialize;
use ucp_core::cam::*;
use ucp_core::Result;
use crate::pipeline::SynthesisOutput;

// ── Full W3C UI Specification Schema types ───────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct W3cUiSpec {
    schema: String,
    components: Vec<W3cComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    design_tokens: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct W3cComponent {
    id: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    anatomy: Vec<W3cPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    states: Option<W3cStateMachine>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    variants: Vec<W3cVariant>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    properties: Vec<W3cProp>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    events: Vec<W3cEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessibility: Option<W3cAccessibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    responsive: Option<W3cResponsive>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    dependencies: Vec<String>,
}

#[derive(Serialize)]
struct W3cPart {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    selectable: bool,
}

#[derive(Serialize)]
struct W3cStateMachine {
    initial: String,
    states: Vec<W3cState>,
}

#[derive(Serialize)]
struct W3cState {
    name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    on: Vec<W3cTransition>,
}

#[derive(Serialize)]
struct W3cTransition {
    event: String,
    target: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    side_effects: Vec<String>,
}

#[derive(Serialize)]
struct W3cVariant {
    name: String,
    #[serde(rename = "type")]
    variant_type: String,
    values: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct W3cProp {
    name: String,
    #[serde(rename = "type")]
    prop_type: String,
    required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    constraints: Option<W3cConstraints>,
}

#[derive(Serialize)]
struct W3cConstraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    enum_values: Vec<String>,
}

#[derive(Serialize)]
struct W3cEvent {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct W3cAccessibility {
    role: Option<String>,
    aria_label: Option<String>,
    keyboard: Option<String>,
    focus_management: Option<String>,
}

#[derive(Serialize)]
struct W3cResponsive {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    breakpoints: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geometry: Option<String>,
}

// ── Mapping logic ─────────────────────────────────────────────────────────

pub fn export_w3c(spec: &SynthesisOutput, output_dir: &str) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    let mut components = Vec::new();
    for comp in &spec.components {
        components.push(cam_to_w3c(comp));
    }

    let w3c_spec = W3cUiSpec {
        schema: "https://www.w3.org/community/uispec/draft".to_string(),
        components,
        design_tokens: None, // placeholder until tokens are integrated
    };

    let json = serde_json::to_string_pretty(&w3c_spec).map_err(ucp_core::UcpError::Json)?;
    fs::write(dir.join("ucp-spec.w3c.json"), json).map_err(ucp_core::UcpError::Io)?;
    Ok(())
}

fn cam_to_w3c(comp: &CanonicalAbstractComponent) -> W3cComponent {
    let name = comp.id.rsplit(':').next().unwrap_or(&comp.id).to_string();

    W3cComponent {
        id: comp.id.clone(),
        name: name.clone(),
        category: None, // future: infer from prop patterns
        description: None, // populated by LLM enrichment
        anatomy: comp.extracted_parts.iter().map(|p| W3cPart {
            name: p.name.clone(),
            description: None,
            selectable: p.selectable,
        }).collect(),
        states: comp.extracted_state_machine.as_ref().map(cam_states_to_w3c),
        variants: extract_variants(comp),
        properties: comp.props.iter().map(cam_prop_to_w3c).collect(),
        events: comp.events.iter().map(cam_event_to_w3c).collect(),
        accessibility: None, // future: from LLM hints or source parsing
        responsive: None,
        dependencies: vec![], // populated from registry deps
    }
}

fn cam_states_to_w3c(sm: &StateMachine) -> W3cStateMachine {
    W3cStateMachine {
        initial: sm.initial.clone(),
        states: sm.states.iter().map(|(name, node)| W3cState {
            name: name.clone(),
            on: node.on.as_ref().map(|trans| {
                trans.iter().map(|(event, t)| W3cTransition {
                    event: event.clone(),
                    target: t.target.clone(),
                    side_effects: t.side_effects.clone(),
                }).collect()
            }).unwrap_or_default(),
        }).collect(),
    }
}

fn cam_prop_to_w3c(p: &CanonicalAbstractProp) -> W3cProp {
    W3cProp {
        name: p.canonical_name.clone(),
        prop_type: format!("{:?}", p.abstract_type).to_lowercase(),
        required: p.reactivity != AbstractReactivity::Static,
        default: if p.reactivity == AbstractReactivity::Static { Some("default".to_string()) } else { None },
        description: p.concrete_type.clone(),
        constraints: None,
    }
}

fn cam_event_to_w3c(e: &CanonicalAbstractEvent) -> W3cEvent {
    W3cEvent {
        name: e.canonical_name.clone(),
        payload: Some(format!("{:?}", e.abstract_payload)),
        description: None,
    }
}

fn extract_variants(comp: &CanonicalAbstractComponent) -> Vec<W3cVariant> {
    let mut variants = Vec::new();
    for p in &comp.props {
        if let Some(conc) = &p.concrete_type {
            if conc.starts_with("enum: ") {
                let values: Vec<String> = conc[6..].split(',').map(|s| s.trim().to_string()).collect();
                variants.push(W3cVariant {
                    name: p.canonical_name.clone(),
                    variant_type: "enum".to_string(),
                    values,
                });
            }
        }
    }
    variants
}
