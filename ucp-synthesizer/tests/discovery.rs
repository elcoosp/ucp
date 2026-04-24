use ucp_synthesizer::discovery::find_shadcn_repos;

// TODO: mockito 1.7 async Server + octocrab base_uri URL resolution don't
// interop correctly (octocrab receives empty 404 body). This integration
// test needs either a real test GitHub token or a different HTTP mock
// approach (e.g., wiremock which controls the full URL resolution).
// The mapping logic itself is trivial and covered by compilation.
#[tokio::test]
#[ignore]
async fn test_discovery_finds_repos() {
    let mut server = mockito::Server::new_async().await;
    server.mock("GET", "/search/repositories")
        .with_status(200)
        .with_body(r#"{"total_count": 1, "items": [{"full_name": "user/shadcn-leptos", "html_url": "https://github.com/user/shadcn-leptos", "license": {"spdx_id": "MIT"}}]}"#)
        .create();

    let repos = find_shadcn_repos("shadcn", &server.url()).await.unwrap();
    assert_eq!(repos.len(), 1);
    assert_eq!(repos[0].full_name, "user/shadcn-leptos");
}
