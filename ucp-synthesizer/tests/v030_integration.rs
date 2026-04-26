use tempfile::TempDir;
use ucp_core::cam::*;
use ucp_synthesizer::export::w3c::export_w3c;
use ucp_synthesizer::generate::dioxus::generate_dioxus;
use ucp_synthesizer::generate::leptos::generate_leptos;
use ucp_synthesizer::generate::registry::generate_registry;
use ucp_synthesizer::pipeline::{PipelineStats, SynthesisOutput};

fn make_test_component(name: &str) -> CanonicalAbstractComponent {
    CanonicalAbstractComponent {
        id: format!("rust:{}.rs:{}", name.to_lowercase(), name),
        semantic_fingerprint: SemanticFingerprint {
            purpose_hash: format!("{:016x}", name.len()),
            normalized_prop_names: vec!["disabled".into(), "label".into()],
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
                canonical_name: "label".into(),
                abstract_type: AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                reactivity: AbstractReactivity::Static,
                concrete_type: Some("String".into()),
                sources: vec![],
                confidence: 1.0,
                conflicts: vec![],
            },
        ],
        events: vec![],
        extracted_state_machine: None,
        extracted_parts: vec![],
        source_repos: vec![SourceAttribution {
            repo_url: "local".into(),
            file_path: format!("{}.rs", name.to_lowercase()),
            line_start: 1,
        }],
        provided_context: None,
        consumed_contexts: vec![],
    }
}

#[test]
fn full_v030_pipeline_dioxus() {
    let tmp = TempDir::new().unwrap();
    let comp = make_test_component("Button");
    let manifest = PackageManifest {
        name: "test-v030".into(),
        version: "0.1.0".into(),
        frameworks: vec!["dioxus".into()],
        components: vec![comp],
        global_styles: None,
        generated_by: "test".into(),
        generated_at: "now".into(),
    };

    // Generate Dioxus
    generate_dioxus(&manifest, &tmp.path().join("dioxus").to_string_lossy()).unwrap();
    assert!(tmp.path().join("dioxus/src/button.rs").exists());

    // Generate Leptos
    generate_leptos(&manifest, &tmp.path().join("leptos").to_string_lossy()).unwrap();
    assert!(tmp.path().join("leptos/src/button.rs").exists());
}

#[test]
fn full_v030_registry_and_w3c() {
    let tmp = TempDir::new().unwrap();
    let comp = make_test_component("Input");
    let manifest = PackageManifest {
        name: "test-v030-reg".into(),
        version: "0.1.0".into(),
        frameworks: vec!["dioxus".into()],
        components: vec![comp],
        global_styles: None,
        generated_by: "test".into(),
        generated_at: "now".into(),
    };

    // Generate registry (corrected interface)
    generate_registry(
        &manifest,
        &tmp.path().join("registry").to_string_lossy(),
        None,
        None,
        None,
    )
    .unwrap();
    assert!(tmp.path().join("registry/registry.json").exists());
    assert!(tmp
        .path()
        .join("registry/registry-item-input.json")
        .exists());

    // Export W3C
    let output = SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![make_test_component("Form")],
        stats: PipelineStats {
            files_scanned: 0,
            files_parsed: 0,
            components_found: 1,
            conflicts_detected: 0,
            llm_enriched: false,
        },
    };
    export_w3c(&output, &tmp.path().join("w3c").to_string_lossy()).unwrap();
    assert!(tmp.path().join("w3c/ucp-spec.w3c.json").exists());
}
