use std::path::Path;

#[test]
fn gpui_real_snapshot_has_expected_components() {
    let path = Path::new("tests/fixtures/gpui_real_snapshot.json");
    let spec = ucp_synthesizer::pipeline::SynthesisOutput::load_from_file(path)
        .expect("Failed to load real GPUI snapshot");
    assert!(
        spec.components.len() >= 10,
        "Snapshot should contain at least 10 components, found {}",
        spec.components.len()
    );
    let ids: Vec<&str> = spec.components.iter().map(|c| c.id.as_str()).collect();
    assert!(
        ids.iter().any(|id| id.contains("Button")),
        "Should contain a Button component"
    );
    assert!(
        ids.iter().any(|id| id.contains("Input")),
        "Should contain an Input component"
    );
}
