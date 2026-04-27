use ucp_maintainer::registry::SpecStore;
use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};
use tempfile::TempDir;

fn empty_spec() -> SynthesisOutput {
    SynthesisOutput {
        ucp_version: "4.0.0".into(),
        components: vec![],
        stats: PipelineStats {
            files_scanned: 0, files_parsed: 0, components_found: 0,
            conflicts_detected: 0, llm_enriched: false,
        },
        provenance: None,
        curation_log: None,
    }
}

#[test]
fn integration_persistent_store_across_connections() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("specs.db");
    let path_str = db_path.to_string_lossy().to_string();

    let id = {
        let store = SpecStore::open(&path_str).unwrap();
        store.store("first", &empty_spec()).unwrap()
    };

    // Re-open and verify
    let store = SpecStore::open(&path_str).unwrap();
    let retrieved = store.get(id).unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().ucp_version, "4.0.0");
}

#[test]
fn integration_list_after_multiple_inserts() {
    let store = SpecStore::open(":memory:").unwrap();
    store.store("alpha", &empty_spec()).unwrap();
    store.store("beta", &empty_spec()).unwrap();
    store.store("gamma", &empty_spec()).unwrap();
    let list = store.list().unwrap();
    assert_eq!(list.len(), 3);
    let names: Vec<&str> = list.iter().map(|(_, n)| n.as_str()).collect();
    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
    assert!(names.contains(&"gamma"));
}

#[test]
fn integration_delete_nonexistent_returns_false() {
    let store = SpecStore::open(":memory:").unwrap();
    assert!(!store.delete(1).unwrap());
}
