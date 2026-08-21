use chassis::connectors::github::{Client, GithubRepo, Owner};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn github_client_queries_mock_server_and_parses_repos() {
    let server = MockServer::start().await;
    let token = "test-token-123";

    let stub_repo = GithubRepo {
        owner: Owner {
            login: "octocat".to_string(),
        },
        name: "Hello-World".to_string(),
        description: Some("A test repository".to_string()),
        language: Some("Rust".to_string()),
        stargazers_count: 1500,
        forks_count: 100,
        open_issues_count: 42,
        topics: vec!["rust".to_string(), "test".to_string()],
    };
    let response_body = serde_json::json!({
        "items": [{
            "owner": { "login": "octocat" },
            "name": "Hello-World",
            "description": "A test repository",
            "language": "Rust",
            "stargazers_count": 1500,
            "forks_count": 100,
            "open_issues_count": 42,
            "topics": ["rust", "test"]
        }]
    });

    Mock::given(method("GET"))
        .and(path("/search/repositories"))
        .and(query_param("q", "stars:>100"))
        .and(query_param("sort", "stars"))
        .and(query_param("order", "desc"))
        .and(query_param("per_page", "30"))
        .and(header("User-Agent", "geosoft-trendboard"))
        .and(header("Authorization", format!("Bearer {token}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&server)
        .await;

    let client = Client::with_base_url(Some(token.to_string()), server.uri());
    let repos = client.list_trending("rust").await.unwrap();

    assert_eq!(repos.len(), 1);
    assert_eq!(repos[0], stub_repo);
}
