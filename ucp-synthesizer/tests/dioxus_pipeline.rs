use std::fs;
use tempfile::TempDir;
use ucp_core::cam::AbstractPropType;
use ucp_synthesizer::pipeline::{self, PipelineOptions};

/// A minimal Dioxus component that uses spread attributes.
const DIOXUS_BUTTON_CODE: &str = r#"
#[derive(Props)]
pub struct ButtonProps {
    #[props(default)]
    disabled: bool,
    label: String,
    variant: ButtonVariant,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! { <button>{props.label}</button> }
}
"#;

#[tokio::test]
async fn dioxus_spread_attributes_appears_in_cam_output() {
    let tmp = TempDir::new().unwrap();
    let src_dir = tmp.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();

    fs::write(src_dir.join("button.rs"), DIOXUS_BUTTON_CODE).unwrap();

    let output = pipeline::run_pipeline_with_options(
        &tmp.path().to_string_lossy(),
        &PipelineOptions::default(),
    )
    .await
    .unwrap();

    assert_eq!(output.stats.components_found, 1);
    let comp = &output.components[0];
    assert!(comp.id.contains("Button"));

    // Check that the attributes prop is of type SpreadAttributes
    let attr_prop = comp
        .props
        .iter()
        .find(|p| p.canonical_name == "attributes")
        .expect("Should have an 'attributes' prop");
    assert_eq!(
        attr_prop.abstract_type,
        AbstractPropType::SpreadAttributes,
        "Expected SpreadAttributes, got {:?}",
        attr_prop.abstract_type
    );
}
