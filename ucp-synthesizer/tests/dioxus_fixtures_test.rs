use std::fs;
use ucp_core::cam::AbstractPropType;
use ucp_synthesizer::pipeline::{self, PipelineOptions};

/// Run the pipeline against the real Dioxus fixture components and check
/// that every component is detected and that spread attributes are mapped.
#[tokio::test]
async fn dioxus_fixtures_all_components_extracted() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixtures = manifest_dir.join("tests/fixtures/dioxus");
    let output = pipeline::run_pipeline_with_options(
        &fixtures.to_string_lossy(),
        &PipelineOptions::default(),
    )
    .await
    .expect("Pipeline should succeed");

    let comps = output.components;
    // We expect at least the four components: Button, Dialog, Accordion, AccordionItem
    assert!(comps.len() >= 4, "Expected at least 4 components, got {}", comps.len());

    let mut button_found = false;
    let mut dialog_found = false;
    let mut accordion_found = false;
    let mut accordion_item_found = false;

    for comp in &comps {
        if comp.id.contains("Button") {
            button_found = true;
            // Button should have spread attributes
            let attr = comp.props.iter().find(|p| p.canonical_name == "attributes");
            assert!(attr.is_some(), "Button component should have 'attributes' prop");
            assert_eq!(
                attr.unwrap().abstract_type,
                AbstractPropType::SpreadAttributes,
                "Button 'attributes' should be SpreadAttributes"
            );
        } else if comp.id.contains("Dialog") {
            dialog_found = true;
            assert!(comp.props.iter().any(|p| p.canonical_name == "open"));
            assert!(comp.props.iter().any(|p| p.canonical_name == "default_open"));
        } else if comp.id.contains("AccordionItem") {
            accordion_item_found = true;
            let attr = comp.props.iter().find(|p| p.canonical_name == "attributes");
            assert!(attr.is_some(), "AccordionItem should have spread attributes");
            assert_eq!(
                attr.unwrap().abstract_type,
                AbstractPropType::SpreadAttributes,
                "AccordionItem 'attributes' should be SpreadAttributes"
            );
        } else if comp.id.contains("Accordion") {
            accordion_found = true;
            assert!(comp.props.iter().any(|p| p.canonical_name == "multiple"));
        }
    }

    assert!(button_found, "Should have found Button component");
    assert!(dialog_found, "Should have found Dialog component");
    assert!(accordion_found, "Should have found Accordion component");
    assert!(accordion_item_found, "Should have found AccordionItem component");
}
