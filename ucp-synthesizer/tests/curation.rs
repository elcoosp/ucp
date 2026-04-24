use ucp_synthesizer::curation::generate_curation_html;
use ucp_core::cam::{Conflict, ResolutionStrategy};

#[test]
fn generate_html_contains_3_columns() {
    let conflicts = vec![Conflict {
        id: "c1".to_string(),
        field: "loading".to_string(),
        present_in: vec!["react".to_string()],
        absent_in: vec!["leptos".to_string()],
        confidence: 0.5,
        resolution_suggestion: ResolutionStrategy::IncludeMajority,
    }];

    let html = generate_curation_html(&conflicts, "react src", "leptos src", "{}").unwrap();
    assert!(html.contains("column-source-a"));
    assert!(html.contains("column-source-b"));
    assert!(html.contains("column-canon"));
}
