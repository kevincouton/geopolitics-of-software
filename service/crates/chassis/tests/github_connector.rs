use chassis::connectors::github;

#[tokio::test]
async fn github_client_returns_repos() {
    // Prefer a real, token-authenticated call so we exercise the wire format.
    // Unauthenticated requests are heavily rate-limited, so skip the test when
    // no token is available (e.g. local dev / CI without secrets).
    let token = std::env::var("GITHUB_TOKEN").ok();
    if token.is_none() {
        eprintln!("GITHUB_TOKEN not set; skipping real GitHub API test");
        return;
    }

    let client = github::Client::new(token);
    let repos = client.list_trending("rust").await.unwrap();
    assert!(!repos.is_empty());
}
