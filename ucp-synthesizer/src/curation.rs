use askama::Template;
use ucp_core::cam::Conflict;
use ucp_core::Result;

#[derive(Template)]
#[template(path = "conflict_review.html")]
struct CurationHtml {
    source_a: String,
    source_b: String,
    conflicts: Vec<ConflictRow>,
}

struct ConflictRow {
    id: String,
    field: String,
    present_in_joined: String,
    absent_in_joined: String,
    confidence: f32,
    proposed_json: String,
}

/// Generate a static 3-column HTML page for human conflict curation.
///
/// - `conflicts`: list of detected CAM conflicts
/// - `source_a_code`: raw source text from the first codebase
/// - `source_b_code`: raw source text from the second codebase
/// - `proposed_canon_json`: the proposed unified UCP spec as JSON
pub fn generate_curation_html(
    conflicts: &[Conflict],
    source_a_code: &str,
    source_b_code: &str,
    proposed_canon_json: &str,
) -> Result<String> {
    let rows: Vec<ConflictRow> = conflicts
        .iter()
        .map(|c| ConflictRow {
            id: c.id.clone(),
            field: c.field.clone(),
            present_in_joined: c.present_in.join(", "),
            absent_in_joined: c.absent_in.join(", "),
            confidence: c.confidence,
            proposed_json: proposed_canon_json.to_string(),
        })
        .collect();

    let html = CurationHtml {
        source_a: source_a_code.to_string(),
        source_b: source_b_code.to_string(),
        conflicts: rows,
    };

    html.render()
        .map_err(|e| ucp_core::UcpError::Parsing(format!("Askama template error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucp_core::cam::ResolutionStrategy;

    #[test]
    fn html_contains_three_column_classes() {
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

    #[test]
    fn html_contains_conflict_details() {
        let conflicts = vec![Conflict {
            id: "conf_042".to_string(),
            field: "props.variant".to_string(),
            present_in: vec!["react".to_string(), "vue".to_string()],
            absent_in: vec!["gpui".to_string()],
            confidence: 0.75,
            resolution_suggestion: ResolutionStrategy::ScopeToProfile("web".to_string()),
        }];

        let html = generate_curation_html(&conflicts, "", "", "{}").unwrap();

        assert!(html.contains("conf_042"));
        assert!(html.contains("props.variant"));
        assert!(html.contains("0.75"));
        assert!(html.contains("react, vue"));
        assert!(html.contains("gpui"));
    }

    #[test]
    fn html_contains_source_code() {
        let conflicts = vec![];
        let html = generate_curation_html(&conflicts, "fn main() {}", "pub fn run() {}", "{}").unwrap();
        assert!(html.contains("fn main()"));
        assert!(html.contains("pub fn run()"));
    }

    #[test]
    fn empty_conflicts_still_valid_html() {
        let html = generate_curation_html(&[], "", "", "{}").unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("</html>"));
    }
}
