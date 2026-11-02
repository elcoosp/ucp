use crate::pipeline::SynthesisOutput;
use serde::Serialize;
use std::fs;
use std::path::Path;
use ucp_core::cam::{
    AbstractReactivity, CanonicalAbstractComponent, CanonicalAbstractEvent, CanonicalAbstractProp,
};
use ucp_core::Result;

/// Machine-readable contract describing every component for AI consumers.
///
/// The contract is the serialization target for AI agents that need to
/// discover which components exist, which props they take and which events
/// they emit, without reading framework source code.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiContract {
    /// Contract schema identifier (`ucp-contract/v1`).
    pub schema: String,
    /// Unix-seconds timestamp of when the contract was generated.
    pub generated_at: String,
    /// One entry per synthesized component.
    pub components: Vec<AiComponent>,
}

/// A single component in the AI contract.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiComponent {
    /// Fully qualified CAM id (`<kind>:<file>:<name>`).
    pub id: String,
    /// Bare component name (last `:`-separated segment of the id).
    pub name: String,
    /// Short human-readable description synthesized from the name.
    pub description: String,
    /// Accepted props.
    pub props: Vec<AiProp>,
    /// Emitted events.
    pub events: Vec<AiEvent>,
    /// Enum-valued props lifted into variants.
    pub variants: Vec<AiVariant>,
    /// Human-readable summary of the extracted state machine, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine: Option<String>,
    /// Names of sibling components this one depends on (currently empty).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
}

/// One prop of an [`AiComponent`].
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiProp {
    /// Prop name as declared in source.
    pub name: String,
    /// Concrete type when known, otherwise the debug form of the abstract type.
    #[serde(rename = "type")]
    pub prop_type: String,
    /// Whether the prop must be supplied by callers (non-static reactivity).
    pub required: bool,
    /// Default marker when the prop is static.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Allowed values when the concrete type is an enum list.
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "enumValues")]
    pub enum_values: Vec<String>,
    /// Extra description emitted only when no concrete type is known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// One event of an [`AiComponent`].
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvent {
    /// Event name as declared in source.
    pub name: String,
    /// Debug rendering of the abstract payload type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

/// One enum-valued prop exposed as a variant.
#[derive(Serialize)]
pub struct AiVariant {
    /// Prop the variant is derived from.
    pub name: String,
    /// Allowed values, split from the `enum: a, b, c` concrete type.
    pub values: Vec<String>,
}

/// Serialize the whole spec as an AI contract JSON file.
///
/// # Errors
///
/// Errors on JSON serialization or file-write failures.
pub fn export_ai_contract(spec: &SynthesisOutput, output_path: &str) -> Result<()> {
    let contract = AiContract {
        schema: "ucp-contract/v1".to_string(),
        generated_at: chrono_now(),
        components: spec.components.iter().map(cam_to_ai).collect(),
    };

    let json = serde_json::to_string_pretty(&contract).map_err(ucp_core::UcpError::Json)?;
    fs::write(Path::new(output_path), json).map_err(ucp_core::UcpError::Io)?;
    Ok(())
}

fn cam_to_ai(comp: &CanonicalAbstractComponent) -> AiComponent {
    let name = comp.id.rsplit(':').next().unwrap_or(&comp.id).to_string();
    AiComponent {
        id: comp.id.clone(),
        name: name.clone(),
        description: format!("{name} component"),
        props: comp.props.iter().map(cam_prop_to_ai).collect(),
        events: comp.events.iter().map(cam_event_to_ai).collect(),
        variants: extract_variants(comp),
        state_machine: comp
            .extracted_state_machine
            .as_ref()
            .map(|sm| format!("{} (initial: {})", sm.id, sm.initial)),
        dependencies: vec![],
    }
}

fn cam_prop_to_ai(p: &CanonicalAbstractProp) -> AiProp {
    AiProp {
        name: p.canonical_name.clone(),
        prop_type: p
            .concrete_type
            .clone()
            .unwrap_or_else(|| format!("{:?}", p.abstract_type)),
        required: p.reactivity != AbstractReactivity::Static,
        default: if p.reactivity == AbstractReactivity::Static {
            Some("default".to_string())
        } else {
            None
        },
        enum_values: extract_enum_values(p),
        description: if p.concrete_type.is_some() {
            None
        } else {
            Some(format!("Abstract: {:?}", p.abstract_type))
        },
    }
}

fn cam_event_to_ai(e: &CanonicalAbstractEvent) -> AiEvent {
    AiEvent {
        name: e.canonical_name.clone(),
        payload: Some(format!("{:?}", e.abstract_payload)),
    }
}

fn extract_variants(comp: &CanonicalAbstractComponent) -> Vec<AiVariant> {
    crate::utils::enum_variant_map(comp, |name, values| AiVariant {
        name: name.to_string(),
        values,
    })
}

fn extract_enum_values(p: &CanonicalAbstractProp) -> Vec<String> {
    p.concrete_type
        .as_deref()
        .map(crate::utils::parse_enum_values)
        .unwrap_or_default()
}

fn chrono_now() -> String {
    // Simple ISO date without chrono dependency
    use std::time::{SystemTime, UNIX_EPOCH};
    let d = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Quick and dirty: just show epoch for now
    format!("{d}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::PipelineStats;
    use ucp_core::cam::{
        AbstractPropType, AbstractReactivity, CanonicalAbstractComponent, CanonicalAbstractProp,
        SemanticFingerprint,
    };

    #[test]
    fn export_ai_contract_with_enum_variants() {
        let comp = CanonicalAbstractComponent {
            id: "rust:button.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "abc".into(),
                normalized_prop_names: vec!["variant".into()],
            },
            props: vec![CanonicalAbstractProp {
                canonical_name: "variant".into(),
                abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("enum: Default, Destructive, Outline".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            }],
            events: vec![],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![],
            provided_context: None,
            consumed_contexts: vec![],
        };
        let output = SynthesisOutput {
            ucp_version: "4.0.0".into(),
            components: vec![comp],
            provenance: None,
            curation_log: None,
            stats: PipelineStats {
                files_scanned: 1,
                files_parsed: 1,
                components_found: 1,
                conflicts_detected: 0,
                llm_enriched: false,
            },
        };
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("contract.json");
        export_ai_contract(&output, &path.to_string_lossy()).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("Default"));
        assert!(content.contains("Destructive"));
        assert!(content.contains("Outline"));
        assert!(content.contains("ucp-contract/v1"));
    }
}
