use ucp_synthesizer::discovery::find_shadcn_repos;
use mockito::mock;

#[tokio::test]
async fn test_discovery_finds_repos() {
    let mut server = mockito::mock_server!();
    server.mock("GET", "/search/repositories")
        .with_status(200)
        .with_body(r#"{"total_count": 1, "items": [{"full_name": "user/shadcn-leptos", "html_url": "https://github.com/user/shadcn-leptos", "license": {"spdx_id": "MIT"}}]}"#)
        .create();

    let repos = find_shadcn_repos("shadcn", &server.url()).await.unwrap();
    assert_eq!(repos.len(), 1);
    assert_eq!(repos[0].full_name, "user/shadcn-leptos");
}
