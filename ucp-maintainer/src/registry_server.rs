//! Registry HTTP server for serving component metadata in shadcn v4 format.
//!
//! Endpoints:
//! - `GET /registry.json` – full registry index
//! - `GET /r/{name}` or `GET /r/{name}.json` – individual component item

use axum::{
    Router,
    extract::Path,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
};
use serde_json::{Value, json};
use std::sync::Arc;
use ucp_synthesizer::pipeline::SynthesisOutput;

pub(crate) struct AppState {
    spec: SynthesisOutput,
}

/// Start the registry HTTP server on the given port.
pub async fn run_registry_server(spec: SynthesisOutput, port: u16) -> anyhow::Result<()> {
    let state = Arc::new(AppState { spec });
    let app = build_router(state);
    let addr = format!("0.0.0.0:{}", port);
    eprintln!("Starting registry server on http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/registry.json", get(registry_index))
        .route("/r/{name}", get(component_by_name))
        .with_state(state)
}

async fn registry_index(State(state): State<Arc<AppState>>) -> Json<Value> {
    let spec = &state.spec;
    let items: Vec<Value> = spec.components.iter().map(|comp| {
        let short_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let snake_name = short_name.to_lowercase().replace(' ', "-");
        json!({
            "name": snake_name,
            "type": "registry:ui",
            "$schema": "https://ui.shadcn.com/schema/registry-item.json",
        })
    }).collect();

    Json(json!({
        "$schema": "https://ui.shadcn.com/schema/registry.json",
        "name": "ucp-registry",
        "homepage": "",
        "items": items,
    }))
}

async fn component_by_name(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let name = name.strip_suffix(".json").unwrap_or(&name);
    let spec = &state.spec;

    let component = spec.components.iter().find(|comp| {
        let short_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        short_name.to_lowercase().replace(' ', "-") == name
    });

    match component {
        Some(comp) => {
            let short_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
            let snake_name = short_name.to_lowercase().replace(' ', "-");
            let code = ucp_synthesizer::generate::dioxus::generate_component_code_for_registry(comp);
            Ok(Json(json!({
                "$schema": "https://ui.shadcn.com/schema/registry-item.json",
                "name": snake_name,
                "type": "registry:ui",
                "title": short_name,
                "files": [{"name": format!("{}.rs", snake_name), "type": "registry:ui", "content": code}],
                "registryDependencies": [],
            })))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode as AxumStatusCode};
    use tower::ServiceExt;
    use ucp_core::cam::*;
    use ucp_synthesizer::pipeline::PipelineStats;

    fn empty_spec() -> SynthesisOutput {
        SynthesisOutput {
            ucp_version: "0.13.0".into(),
            components: vec![],
            stats: PipelineStats {
                files_scanned: 0, files_parsed: 0, components_found: 0,
                conflicts_detected: 0, llm_enriched: false,
            },
            provenance: None, curation_log: None,
        }
    }

    fn spec_with_button() -> SynthesisOutput {
        let mut spec = empty_spec();
        spec.components.push(CanonicalAbstractComponent {
            id: "rust:button.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "abc123".into(),
                normalized_prop_names: vec!["label".into(), "onclick".into()],
            },
            props: vec![], events: vec![], extracted_state_machine: None,
            extracted_parts: vec![], source_repos: vec![],
            provided_context: None, consumed_contexts: vec![],
        });
        spec
    }

    fn spec_with_two() -> SynthesisOutput {
        let mut spec = empty_spec();
        for id in &["rust:button.rs:Button", "rust:dialog.rs:Dialog"] {
            spec.components.push(CanonicalAbstractComponent {
                id: id.to_string(),
                semantic_fingerprint: SemanticFingerprint {
                    purpose_hash: "x".into(), normalized_prop_names: vec![],
                },
                props: vec![], events: vec![], extracted_state_machine: None,
                extracted_parts: vec![], source_repos: vec![],
                provided_context: None, consumed_contexts: vec![],
            });
        }
        spec
    }

    fn make_state(spec: SynthesisOutput) -> Arc<AppState> {
        Arc::new(AppState { spec })
    }

    // === Route test (oneshot works for literal routes) ===

    #[tokio::test]
    async fn registry_index_route() {
        let app = build_router(make_state(empty_spec()));
        let resp = app.oneshot(
            Request::builder().uri("/registry.json").body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(resp.status(), AxumStatusCode::OK);
    }

    // === Handler tests (direct invocation - reliable) ===

    #[tokio::test]
    async fn registry_index_empty_spec() {
        let state = make_state(empty_spec());
        let body = registry_index(State(state)).await.0;
        assert_eq!(body["items"].as_array().unwrap().len(), 0);
        assert_eq!(body["$schema"], "https://ui.shadcn.com/schema/registry.json");
        assert_eq!(body["name"], "ucp-registry");
    }

    #[tokio::test]
    async fn registry_index_with_components() {
        let state = make_state(spec_with_two());
        let body = registry_index(State(state)).await.0;
        let items = body["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
        let names: Vec<&str> = items.iter().map(|i| i["name"].as_str().unwrap()).collect();
        assert!(names.contains(&"button"));
        assert!(names.contains(&"dialog"));
    }

    #[tokio::test]
    async fn component_by_name_with_json_extension() {
        let state = make_state(spec_with_button());
        let result = component_by_name(State(state), Path("button.json".into())).await;
        assert!(result.is_ok());
        let body = result.unwrap().0;
        assert_eq!(body["name"], "button");
        assert_eq!(body["type"], "registry:ui");
        assert_eq!(body["title"], "Button");
        assert!(!body["files"][0]["content"].as_str().unwrap().is_empty());
    }

    #[tokio::test]
    async fn component_by_name_without_extension() {
        let state = make_state(spec_with_button());
        let result = component_by_name(State(state), Path("button".into())).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0["name"], "button");
    }

    #[tokio::test]
    async fn component_not_found() {
        let state = make_state(spec_with_button());
        let result = component_by_name(State(state), Path("nonexistent.json".into())).await;
        assert_eq!(result.err().unwrap(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn component_not_found_no_extension() {
        let state = make_state(spec_with_button());
        let result = component_by_name(State(state), Path("nonexistent".into())).await;
        assert_eq!(result.err().unwrap(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn component_name_is_lowercased() {
        let mut spec = empty_spec();
        spec.components.push(CanonicalAbstractComponent {
            id: "rust:MyButton.rs:MyButton".into(),
            semantic_fingerprint: SemanticFingerprint {
                purpose_hash: "x".into(), normalized_prop_names: vec![],
            },
            props: vec![], events: vec![], extracted_state_machine: None,
            extracted_parts: vec![], source_repos: vec![],
            provided_context: None, consumed_contexts: vec![],
        });
        let state = make_state(spec);

        // lowercase should work
        let result = component_by_name(State(state.clone()), Path("mybutton.json".into())).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0["name"], "mybutton");

        // original case should 404
        let result = component_by_name(State(state), Path("MyButton.json".into())).await;
        assert_eq!(result.err().unwrap(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn index_names_match_handler_lookup() {
        let state = make_state(spec_with_two());
        let index = registry_index(State(state.clone())).await.0;

        // Every name in the index should be fetchable via the handler
        for item in index["items"].as_array().unwrap() {
            let name = item["name"].as_str().unwrap();
            let result = component_by_name(
                State(state.clone()),
                Path(format!("{}.json", name)),
            ).await;
            assert!(result.is_ok(), "Component '{}' from index should be fetchable", name);
        }
    }
}
