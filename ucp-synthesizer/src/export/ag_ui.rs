use std::fs;
use std::path::Path;
use serde::Serialize;
use crate::pipeline::SynthesisOutput;
use ucp_core::Result;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgUiSchema {
    pub protocol: String,       // "ag-ui/v1"
    pub events: Vec<AgUiEvent>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgUiEvent {
    pub component: String,
    pub event: String,
    pub event_type: String,     // e.g., "component.click"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

pub fn export_ag_ui(spec: &SynthesisOutput, output_dir: &str) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir).map_err(ucp_core::UcpError::Io)?;

    let mut events = Vec::new();

    for comp in &spec.components {
        let comp_name = comp.id.rsplit(':').next().unwrap_or(&comp.id).to_string();
        for ev in &comp.events {
            events.push(AgUiEvent {
                component: comp_name.clone(),
                event: ev.canonical_name.clone(),
                event_type: format!("component.{}", ev.canonical_name),
                payload: Some(format!("{:?}", ev.abstract_payload)),
            });
        }
    }

    let schema = AgUiSchema {
        protocol: "ag-ui/v1".to_string(),
        events,
    };

    let json = serde_json::to_string_pretty(&schema)
        .map_err(ucp_core::UcpError::Json)?;
    fs::write(dir.join("ag-ui-events.json"), json)
        .map_err(ucp_core::UcpError::Io)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucp_core::cam::*;
    use crate::pipeline::PipelineStats;

    #[test]
    fn ag_ui_export_contains_all_component_events() {
        let comp1 = CanonicalAbstractComponent {
            id: "rust:button.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint { purpose_hash: "a".into(), normalized_prop_names: vec![] },
            props: vec![],
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
        let comp2 = CanonicalAbstractComponent {
            id: "rust:dialog.rs:Dialog".into(),
            semantic_fingerprint: SemanticFingerprint { purpose_hash: "b".into(), normalized_prop_names: vec![] },
            props: vec![],
            events: vec![
                CanonicalAbstractEvent { canonical_name: "open".into(), abstract_payload: AbstractPropType::AsyncEventHandler(vec![]) },
                CanonicalAbstractEvent { canonical_name: "close".into(), abstract_payload: AbstractPropType::AsyncEventHandler(vec![]) },
            ],
            extracted_state_machine: None,
            extracted_parts: vec![],
            source_repos: vec![],
            provided_context: None,
            consumed_contexts: vec![],
        };
        let output = SynthesisOutput {
            ucp_version: "4.0.0".into(),
            components: vec![comp1, comp2],
            stats: PipelineStats {
                files_scanned: 0, files_parsed: 0, components_found: 2,
                conflicts_detected: 0, llm_enriched: false,
            },
        };
        let tmp = tempfile::TempDir::new().unwrap();
        export_ag_ui(&output, &tmp.path().to_string_lossy()).unwrap();
        let content = std::fs::read_to_string(tmp.path().join("ag-ui-events.json")).unwrap();
        assert!(content.contains("component.click"));
        assert!(content.contains("component.open"));
        assert!(content.contains("component.close"));
        assert!(content.contains("Button"));
        assert!(content.contains("Dialog"));
    }
}
