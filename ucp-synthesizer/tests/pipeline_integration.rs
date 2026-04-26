use std::fs;
use tempfile::TempDir;
use ucp_core::cam::{AbstractPropType, AbstractReactivity};
use ucp_synthesizer::pipeline::{self, PipelineOptions};

/// Helper: create a temp source tree with a `src/` dir (required by is_path_safe_to_parse).
fn setup_source_dir() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let src_dir = tmp.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();

    fs::write(
        src_dir.join("card.rs"),
        r#"
#[component]
pub fn Card(
    #[prop(default)] elevated: bool,
    title: String,
    #[prop(default = 0)] padding: usize,
) -> impl IntoView {
    view! { <div>{}</div> }
}
"#,
    )
    .unwrap();

    tmp
}

#[tokio::test]
async fn pipeline_scans_and_parses_rust_components() {
    let tmp = setup_source_dir();
    let source_dir = tmp.path().to_string_lossy().to_string();

    let output = pipeline::run_pipeline_with_options(
        &source_dir,
        &PipelineOptions {
            ollama_url: None,
            llm_model: "unused".to_string(),
            dry_run: true,
        },
    )
    .await
    .unwrap();

    assert_eq!(output.ucp_version, "4.0.0");
    assert_eq!(output.stats.files_scanned, 1);
    assert_eq!(output.stats.files_parsed, 1);
    assert_eq!(output.stats.components_found, 1);
    assert_eq!(output.stats.conflicts_detected, 0);
    assert!(!output.stats.llm_enriched);

    // SourceAttribution should have real line numbers, not hardcoded 0
    let card = &output.components[0];
    assert!(
        card.source_repos[0].line_start > 0,
        "Expected line_start > 0, got {}",
        card.source_repos[0].line_start
    );
}

#[tokio::test]
async fn pipeline_extracts_correct_prop_types() {
    let tmp = setup_source_dir();
    let source_dir = tmp.path().to_string_lossy().to_string();

    let output = pipeline::run_pipeline_with_options(&source_dir, &PipelineOptions::default())
        .await
        .unwrap();

    let comp = &output.components[0];
    assert!(
        comp.id.contains("Card"),
        "Expected component ID to contain 'Card', got: {}",
        comp.id
    );

    let props_by_name: std::collections::HashMap<_, _> = comp
        .props
        .iter()
        .map(|p| (p.canonical_name.as_str(), p))
        .collect();

    // elevated: bool → ControlFlag
    let elevated = props_by_name["elevated"];
    assert_eq!(elevated.abstract_type, AbstractPropType::ControlFlag);
    assert_eq!(elevated.reactivity, AbstractReactivity::Static);

    // title: String → StaticValue(Any)
    let title = props_by_name["title"];
    assert!(matches!(
        title.abstract_type,
        AbstractPropType::StaticValue(_)
    ));

    // padding: usize → StaticValue(Any)
    let padding = props_by_name["padding"];
    assert!(matches!(
        padding.abstract_type,
        AbstractPropType::StaticValue(_)
    ));
    assert!(padding.conflicts.is_empty());
}

#[tokio::test]
async fn pipeline_skips_files_outside_src() {
    let tmp = TempDir::new().unwrap();
    // Write a file directly in tmp root (no src/ in path → rejected by is_path_safe_to_parse)
    fs::write(
        tmp.path().join("secret_component.rs"),
        r#"
#[component]
pub fn Secret(visible: bool) -> impl IntoView { view! { <div></div> } }
"#,
    )
    .unwrap();

    let output = pipeline::run_pipeline_with_options(
        &tmp.path().to_string_lossy(),
        &PipelineOptions::default(),
    )
    .await
    .unwrap();

    assert_eq!(output.stats.files_scanned, 0);
    assert_eq!(output.stats.components_found, 0);
}

#[tokio::test]
async fn pipeline_skips_credential_files() {
    let tmp = TempDir::new().unwrap();
    let src_dir = tmp.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();

    // .env extension → rejected by is_path_safe_to_parse
    fs::write(src_dir.join(".env"), "SECRET=token").unwrap();

    let output = pipeline::run_pipeline_with_options(
        &tmp.path().to_string_lossy(),
        &PipelineOptions::default(),
    )
    .await
    .unwrap();

    assert_eq!(output.stats.files_scanned, 0);
}

#[tokio::test]
async fn pipeline_returns_empty_for_nonexistent_dir() {
    let output = pipeline::run_pipeline_with_options(
        "/nonexistent/path/that/does/not/exist",
        &PipelineOptions::default(),
    )
    .await
    .unwrap();

    assert_eq!(output.stats.files_scanned, 0);
    assert!(output.components.is_empty());
}

#[tokio::test]
async fn pipeline_parses_tsx_components() {
    let tmp = TempDir::new().unwrap();
    let components_dir = tmp.path().join("components");
    fs::create_dir_all(&components_dir).unwrap();

    fs::write(
        components_dir.join("badge.tsx"),
        r#"
export interface BadgeProps {
  variant?: "default" | "secondary";
  label: string;
  onClick?: () => void;
}
export const Badge = (props: BadgeProps) => {
  return <span>{props.label}</span>;
};
"#,
    )
    .unwrap();

    let output = pipeline::run_pipeline_with_options(
        &tmp.path().to_string_lossy(),
        &PipelineOptions::default(),
    )
    .await
    .unwrap();

    assert_eq!(output.stats.files_scanned, 1);
    assert_eq!(output.stats.files_parsed, 1);
    assert_eq!(output.stats.components_found, 1);

    let comp = &output.components[0];
    assert!(comp.id.contains("Badge"));
    assert_eq!(comp.props.len(), 3);

    // TSX source attribution should also have real line numbers
    assert!(
        comp.source_repos[0].line_start > 0,
        "Expected TSX line_start > 0, got {}",
        comp.source_repos[0].line_start
    );

    let props_by_name: std::collections::HashMap<_, _> = comp
        .props
        .iter()
        .map(|p| (p.canonical_name.as_str(), p))
        .collect();

    // onClick: () => void → AsyncEventHandler
    let on_click = props_by_name["onClick"];
    assert!(matches!(
        on_click.abstract_type,
        AbstractPropType::AsyncEventHandler(_)
    ));
}

#[tokio::test]
async fn pipeline_extracts_struct_props_components() {
    let tmp = TempDir::new().unwrap();
    let src_dir = tmp.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();
    let fixtures = std::path::Path::new("tests/leptos_shadcn_fixtures");
    for entry in fs::read_dir(fixtures).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), src_dir.join(entry.file_name())).unwrap();
    }
    let src = tmp.path().to_string_lossy().to_string();
    let output = pipeline::run_pipeline_with_options(
        &src,
        &PipelineOptions::default(),
    )
    .await
    .unwrap();
    assert_eq!(output.stats.components_found, 2);
    for comp in &output.components {
        assert!(comp.id.contains("StandardizedButton") || comp.id.contains("Badge"));
    }
    let button = output
        .components
        .iter()
        .find(|c| c.id.contains("StandardizedButton"))
        .unwrap();
    assert!(button.props.len() >= 4);
    assert!(button.props.iter().any(|p| p.canonical_name == "disabled"));
    assert!(button.props.iter().any(|p| p.canonical_name == "onclick"));
}
