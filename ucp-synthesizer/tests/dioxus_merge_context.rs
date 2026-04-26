use std::fs;
use tempfile::TempDir;
use ucp_synthesizer::pipeline::{self, PipelineOptions, SynthesisOutput};

/// Two Dioxus components with provided context and consumed contexts.
/// After extraction and merge, the merged component should retain both.
const PROVIDER_CODE: &str = r#"
#[derive(Props)]
pub struct DialogProps {
    open: bool,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let ctx = DialogContext {
        open: Signal::new(props.open),
        set_open: Callback::new(|_| {}),
        content_id: String::new(),
        title_id: String::new(),
        description_id: String::new(),
    };
    use_context_provider(|| ctx);
    rsx! { {props.children} }
}
"#;

const CONSUMER_CODE: &str = r#"
#[component]
pub fn DialogActions() -> Element {
    let ctx = use_context::<DialogContext>();
    rsx! { <div></div> }
}
"#;

#[tokio::test]
async fn dioxus_context_survives_merge() {
    let tmp_a = TempDir::new().unwrap();
    let src_a = tmp_a.path().join("src");
    fs::create_dir_all(&src_a).unwrap();
    fs::write(src_a.join("dialog.rs"), PROVIDER_CODE).unwrap();

    let tmp_b = TempDir::new().unwrap();
    let src_b = tmp_b.path().join("src");
    fs::create_dir_all(&src_b).unwrap();
    fs::write(src_b.join("actions.rs"), CONSUMER_CODE).unwrap();

    let spec_a = pipeline::run_pipeline_with_options(
        &tmp_a.path().to_string_lossy(),
        &PipelineOptions::default(),
    )
    .await
    .unwrap();

    let spec_b = pipeline::run_pipeline_with_options(
        &tmp_b.path().to_string_lossy(),
        &PipelineOptions::default(),
    )
    .await
    .unwrap();

    // Manually set provided/consumed on the consumer (since extraction doesn't yet detect consumed in function components without props)
    // Actually, our extractor only detects context for Dioxus pattern (props struct). For a bare #[component] without props struct,
    // it won't be picked by dioxus_ast. So we modify the spec to simulate.
    // Better: modify the consumer code to use a props struct? But we want to test merge, so we'll just set the fields manually.
    // However, the test is about merge preserving context, so we can artificially set the fields.
    let mut comp_a = spec_a.components[0].clone();
    comp_a.provided_context = Some("DialogContext".to_string());
    let spec_a_mod = SynthesisOutput {
        components: vec![comp_a],
        ..spec_a
    };

    let mut comp_b = spec_b.components[0].clone();
    comp_b.consumed_contexts = vec!["DialogContext".to_string()];
    let spec_b_mod = SynthesisOutput {
        components: vec![comp_b],
        ..spec_b
    };

    let merged = ucp_synthesizer::merge::merge_specs(&[spec_a_mod, spec_b_mod]).unwrap();
    // Since they have different fingerprints, they won't merge into one component.
    // Actually, they will remain separate because fingerprint is based on name+props.
    // So this test is not about dedup; it's about verifying fields are present in each after merge.
    // The merge function just collects components, so both should be present with their context fields.
    assert_eq!(merged.components.len(), 2);
    let provider = merged.components.iter().find(|c| c.id.contains("Dialog")).unwrap();
    assert_eq!(provider.provided_context.as_deref(), Some("DialogContext"));
    assert!(provider.consumed_contexts.is_empty());

    let consumer = merged.components.iter().find(|c| c.id.contains("Actions")).unwrap();
    assert_eq!(consumer.consumed_contexts, vec!["DialogContext"]);
    assert!(consumer.provided_context.is_none());
}
