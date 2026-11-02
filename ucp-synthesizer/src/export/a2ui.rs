use crate::pipeline::SynthesisOutput;
use serde::Serialize;
use std::fs;
use std::path::Path;
use ucp_core::cam::{
    AbstractReactivity, CanonicalAbstractComponent, CanonicalAbstractEvent, CanonicalAbstractProp,
};
use ucp_core::Result;

/// Top-level A2UI catalog document.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct A2uiCatalog {
    /// Catalog schema identifier (`a2ui/v1`).
    pub schema: String,
    /// Library display name.
    pub library: String,
    /// Spec version string.
    pub version: String,
    /// One entry per synthesized component.
    pub components: Vec<A2uiComponent>,
}

/// One component in the A2UI catalog.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct A2uiComponent {
    /// Fully qualified CAM id.
    pub id: String,
    /// Bare component name.
    pub name: String,
    /// Short description synthesized from the name.
    pub description: String,
    /// Accepted props.
    pub props: Vec<A2uiProp>,
    /// Emitted events.
    pub events: Vec<A2uiEvent>,
    /// Enum-valued props lifted into variants.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub variants: Vec<A2uiVariant>,
    /// State-machine summary, when extraction produced one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine: Option<String>,
}

/// One prop of an [`A2uiComponent`].
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct A2uiProp {
    /// Prop name as declared in source.
    pub name: String,
    /// Concrete type when known, else abstract-type debug form.
    #[serde(rename = "type")]
    pub prop_type: String,
    /// Whether callers must supply the prop.
    pub required: bool,
    /// Default marker for static props.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Allowed values for enum-typed props.
    #[serde(skip_serializing_if = "Vec::is_empty", default, rename = "enumValues")]
    pub enum_values: Vec<String>,
    /// Extra description when no concrete type is known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// One event of an [`A2uiComponent`].
#[derive(Serialize)]
pub struct A2uiEvent {
    /// Event name as declared in source.
    pub name: String,
    /// Debug rendering of the abstract payload type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

/// One enum-valued prop exposed as a variant.
#[derive(Serialize)]
pub struct A2uiVariant {
    /// Prop the variant is derived from.
    pub name: String,
    /// Allowed values.
    pub values: Vec<String>,
}

/// Write the spec as an A2UI catalog (`a2ui.json`) into `output_dir`.
///
/// # Errors
///
/// Errors on JSON serialization or file-write failures.
pub fn export_a2ui(
    spec: &SynthesisOutput,
    library_name: &str,
    version: &str,
    output_dir: &str,
) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    let catalog = A2uiCatalog {
        schema: "https://a2ui.dev/schema/v0.9".to_string(),
        library: library_name.to_string(),
        version: version.to_string(),
        components: spec.components.iter().map(cam_to_a2ui).collect(),
    };

    let json = serde_json::to_string_pretty(&catalog).map_err(ucp_core::UcpError::Json)?;
    fs::write(dir.join("a2ui-catalog.json"), json).map_err(ucp_core::UcpError::Io)?;
    Ok(())
}

fn cam_to_a2ui(comp: &CanonicalAbstractComponent) -> A2uiComponent {
    let name = comp.id.rsplit(':').next().unwrap_or(&comp.id).to_string();
    A2uiComponent {
        id: comp.id.clone(),
        name: name.clone(),
        description: format!("{name} component"),
        props: comp.props.iter().map(cam_prop_to_a2ui).collect(),
        events: comp.events.iter().map(cam_event_to_a2ui).collect(),
        variants: extract_variants(comp),
        state_machine: comp
            .extracted_state_machine
            .as_ref()
            .map(|sm| format!("{} (initial: {})", sm.id, sm.initial)),
    }
}

fn cam_prop_to_a2ui(p: &CanonicalAbstractProp) -> A2uiProp {
    A2uiProp {
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
        description: p.concrete_type.clone(),
    }
}

fn cam_event_to_a2ui(e: &CanonicalAbstractEvent) -> A2uiEvent {
    A2uiEvent {
        name: e.canonical_name.clone(),
        payload: Some(format!("{:?}", e.abstract_payload)),
    }
}

fn extract_variants(comp: &CanonicalAbstractComponent) -> Vec<A2uiVariant> {
    crate::utils::enum_variant_map(comp, |name, values| A2uiVariant {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::PipelineStats;
    use ucp_core::cam::{
        AbstractPropType, AbstractReactivity, CanonicalAbstractComponent, CanonicalAbstractEvent,
        CanonicalAbstractProp, SemanticFingerprint,
    };

    #[test]
    fn a2ui_catalog_contains_component_props_and_events() {
        let comp = CanonicalAbstractComponent {
            id: "rust:button.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "abc".into(),
                normalized_prop_names: vec!["disabled".into(), "variant".into()],
            },
            props: vec![
                CanonicalAbstractProp {
                    canonical_name: "disabled".into(),
                    abstract_type: AbstractPropType::ControlFlag,
                    reactivity: AbstractReactivity::Static,
                    concrete_type: Some("bool".into()),
                    sources: vec![],
                    confidence: 1.0,
                    conflicts: vec![],
                },
                CanonicalAbstractProp {
                    canonical_name: "variant".into(),
                    abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                    reactivity: AbstractReactivity::Static,
                    concrete_type: Some("enum: Default, Destructive".into()),
                    sources: vec![],
                    confidence: 1.0,
                    conflicts: vec![],
                },
            ],
            events: vec![CanonicalAbstractEvent {
                canonical_name: "click".to_string(),
                abstract_payload: AbstractPropType::AsyncEventHandler(vec![]),
            }],
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
        export_a2ui(&output, "test-lib", "1.0.0", &tmp.path().to_string_lossy()).unwrap();
        let content = std::fs::read_to_string(tmp.path().join("a2ui-catalog.json")).unwrap();
        assert!(content.contains("Button"));
        assert!(content.contains("disabled"));
        assert!(content.contains("bool"));
        assert!(content.contains("variant"));
        assert!(content.contains("Default"));
        assert!(content.contains("Destructive"));
        assert!(content.contains("click"));
    }
}
