//! Registry HTTP server for serving component metadata in shadcn v4 format.
//!
//! Endpoints:
//! - `GET /registry.json` – full registry index
//! - `GET /r/{name}` or `GET /r/{name}.json` – individual component item
//!
//! Optional bearer token auth via `--token` flag.

use axum::{
    Router,
    extract::Path,
    extract::State,
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Json, Response},
    routing::get,
};
use serde_json::{Value, json};
use std::sync::Arc;
use ucp_synthesizer::pipeline::SynthesisOutput;

pub(crate) struct AppState {
    spec: SynthesisOutput,
    token: Option<String>,
}

/// Start the registry HTTP server on the given port.
pub async fn run_registry_server(spec: SynthesisOutput, port: u16) -> anyhow::Result<()> {
    run_registry_server_with_token(spec, port, None).await
}

/// Start the registry HTTP server with optional bearer token auth.
pub async fn run_registry_server_with_token(
    spec: SynthesisOutput,
    port: u16,
    token: Option<String>,
) -> anyhow::Result<()> {
    let state = Arc::new(AppState { spec, token: token.clone() });
    let app = build_router(state);
    let addr = format!("0.0.0.0:{}", port);
    if token.is_some() {
        eprintln!("Starting registry server on http://{}/ (auth enabled)", addr);
    } else {
        eprintln!("Starting registry server on http://{}/", addr);
    }
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn build_router(state: Arc<AppState>) -> Router {
    let has_token = state.token.is_some();
    let mut router = Router::new()
        .route("/registry.json", get(registry_index))
        .route("/r/{name}", get(component_by_name))
        .with_state(state.clone());

    if has_token {
        router = router.layer(axum::middleware::from_fn_with_state(
            Arc::clone(&state),
            auth_middleware,
        ));
    }
    router
}

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    req: axum::extract::Request,
    next: Next,
) -> Response {
    let token = match &state.token {
        Some(t) => t,
        None => return next.run(req).await,
    };
    let provided = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    match provided {
        Some(t) if t == token => next.run(req).await,
        _ => (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))).into_response(),
    }
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
            ucp_version: "0.13.0".into(), components: vec![],
            stats: PipelineStats { files_scanned: 0, files_parsed: 0, components_found: 0,
                conflicts_detected: 0, llm_enriched: false },
            provenance: None, curation_log: None,
        }
    }
    fn spec_with_button() -> SynthesisOutput {
        let mut s = empty_spec();
        s.components.push(CanonicalAbstractComponent {
            id: "rust:button.rs:Button".into(),
            semantic_fingerprint: SemanticFingerprint { purpose_hash: "x".into(), normalized_prop_names: vec![] },
            props: vec![], events: vec![], extracted_state_machine: None,
            extracted_parts: vec![], source_repos: vec![], provided_context: None, consumed_contexts: vec![],
        });
        s
    }
    fn no_auth(s: SynthesisOutput) -> Arc<AppState> { Arc::new(AppState { spec: s, token: None }) }
    fn with_auth(s: SynthesisOutput) -> Arc<AppState> { Arc::new(AppState { spec: s, token: Some("secret".into()) }) }

    #[tokio::test]
    async fn registry_index_ok() {
        let r = build_router(no_auth(empty_spec())).oneshot(
            Request::builder().uri("/registry.json").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(r.status(), AxumStatusCode::OK);
    }
    #[tokio::test]
    async fn component_found() {
        let r = component_by_name(State(no_auth(spec_with_button())), Path("button.json".into())).await;
        assert!(r.is_ok()); assert_eq!(r.unwrap().0["name"], "button");
    }
    #[tokio::test]
    async fn component_not_found() {
        let r = component_by_name(State(no_auth(spec_with_button())), Path("nope.json".into())).await;
        assert_eq!(r.err().unwrap(), StatusCode::NOT_FOUND);
    }
    #[tokio::test]
    async fn auth_rejects_no_token() {
        let r = build_router(with_auth(spec_with_button())).oneshot(
            Request::builder().uri("/registry.json").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(r.status(), AxumStatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn auth_rejects_bad_token() {
        let r = build_router(with_auth(spec_with_button())).oneshot(
            Request::builder().uri("/registry.json").header("Authorization", "Bearer wrong").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(r.status(), AxumStatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn auth_accepts_good_token() {
        let r = build_router(with_auth(spec_with_button())).oneshot(
            Request::builder().uri("/registry.json").header("Authorization", "Bearer secret").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(r.status(), AxumStatusCode::OK);
    }
    #[tokio::test]
    async fn no_auth_when_unset() {
        let r = build_router(no_auth(spec_with_button())).oneshot(
            Request::builder().uri("/registry.json").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(r.status(), AxumStatusCode::OK);
    }
}
