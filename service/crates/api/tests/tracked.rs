use api::{router, state::AppState};
use axum::{
    body::{to_bytes, Body},
    http::Request,
};
use chassis::{
    connectors::github::{GithubRepo, Owner},
    projects, scoring, snapshots,
};
use sqlx::PgPool;
use tower::ServiceExt;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn sample_repo() -> GithubRepo {
    GithubRepo {
        owner: Owner {
            login: "octocat".into(),
        },
        name: "hello".into(),
        description: Some("demo".into()),
        language: Some("Rust".into()),
        stargazers_count: 100,
        forks_count: 10,
        open_issues_count: 2,
        topics: vec!["demo".into()],
    }
}

fn app_state(pool: PgPool) -> AppState {
    app_state_with(pool, "https://api.github.com", false)
}

fn app_state_with(pool: PgPool, github_base_url: &str, cookie_secure: bool) -> AppState {
    AppState {
        cfg: chassis::config::Config {
            database_url: "postgres://localhost/test".into(),
            api_port: 0,
            github_token: None,
            github_base_url: github_base_url.into(),
            cookie_secure,
            cors_origins: vec!["http://localhost:3000".into()],
        },
        pool,
    }
}

fn extract_session_cookie(response: &axum::response::Response) -> Option<String> {
    response
        .headers()
        .get_all("set-cookie")
        .iter()
        .find_map(|value| {
            let cookie = value.to_str().ok()?;
            cookie
                .split(';')
                .next()
                .filter(|part| part.starts_with("session_id="))
                .map(|part| part.to_string())
        })
}

fn cookie_has_secure_flag(response: &axum::response::Response) -> bool {
    response
        .headers()
        .get_all("set-cookie")
        .iter()
        .filter_map(|value| value.to_str().ok())
        .any(|cookie| {
            cookie.starts_with("session_id=")
                && cookie.split(';').any(|part| part.trim() == "Secure")
        })
}

#[sqlx::test(migrations = "../../migrations")]
async fn track_and_list_project(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();
    let score = scoring::score(&project, &[]);
    snapshots::record(&pool, project.id, project.stars, project.forks, &score)
        .await
        .unwrap();

    let app = router::app(app_state(pool));

    let track_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(format!("{{\"project_id\":\"{}\"}}", project.id)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(track_response.status(), 200);

    let session_cookie = extract_session_cookie(&track_response).expect("session cookie set");

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .header("cookie", &session_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(list_response.status(), 200);

    let body = to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["github_name"], "hello");
    assert!(!items[0]["score"].is_null());
}

#[sqlx::test(migrations = "../../migrations")]
async fn untrack_project(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();

    let app = router::app(app_state(pool));

    let track_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(format!("{{\"project_id\":\"{}\"}}", project.id)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(track_response.status(), 200);

    let session_cookie = extract_session_cookie(&track_response).expect("session cookie set");

    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/me/tracked/{}", project.id))
                .method("DELETE")
                .header("cookie", &session_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(delete_response.status(), 204);

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .header("cookie", &session_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(list_response.status(), 200);

    let body = to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(items.is_empty());
}

#[sqlx::test(migrations = "../../migrations")]
async fn tracked_projects_are_isolated_by_session(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();

    let app = router::app(app_state(pool));

    let first_session = {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/me/tracked")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(format!("{{\"project_id\":\"{}\"}}", project.id)))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        extract_session_cookie(&response).expect("session cookie set")
    };

    let second_session = {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/me/tracked")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(format!("{{\"project_id\":\"{}\"}}", project.id)))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        extract_session_cookie(&response).expect("session cookie set")
    };

    assert_ne!(first_session, second_session);

    let other_list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .header("cookie", &second_session)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(other_list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(items.len(), 1);

    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/me/tracked/{}", project.id))
                .method("DELETE")
                .header("cookie", &first_session)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(delete_response.status(), 204);

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .header("cookie", &first_session)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(items.is_empty());
}

#[sqlx::test(migrations = "../../migrations")]
async fn track_unknown_project_returns_404(pool: PgPool) {
    let app = router::app(app_state(pool));
    let unknown_project_id = uuid::Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"project_id\":\"{}\"}}",
                    unknown_project_id
                )))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
}

#[sqlx::test(migrations = "../../migrations")]
async fn cors_preflight_returns_allowed_headers(pool: PgPool) {
    let app = router::app(app_state(pool));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .method("OPTIONS")
                .header("origin", "http://localhost:3000")
                .header("access-control-request-method", "POST")
                .header("access-control-request-headers", "content-type")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(
        response
            .headers()
            .get("access-control-allow-origin")
            .and_then(|v| v.to_str().ok()),
        Some("http://localhost:3000")
    );
    assert_eq!(
        response
            .headers()
            .get("access-control-allow-credentials")
            .and_then(|v| v.to_str().ok()),
        Some("true")
    );
    assert!(response
        .headers()
        .get("access-control-allow-methods")
        .is_some());
}

#[sqlx::test(migrations = "../../migrations")]
async fn track_by_owner_name_fetches_and_tracks_project(pool: PgPool) {
    let server = MockServer::start().await;
    let repo = sample_repo();
    let response_body = serde_json::json!({
        "owner": { "login": repo.owner.login },
        "name": repo.name,
        "description": repo.description,
        "language": repo.language,
        "stargazers_count": repo.stargazers_count,
        "forks_count": repo.forks_count,
        "open_issues_count": repo.open_issues_count,
        "topics": repo.topics,
    });

    Mock::given(method("GET"))
        .and(path("/repos/octocat/hello"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&server)
        .await;

    let app = router::app(app_state_with(pool, &server.uri(), false));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/me/tracked/octocat/hello")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let item: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(item["github_owner"], "octocat");
    assert_eq!(item["github_name"], "hello");
}

#[sqlx::test(migrations = "../../migrations")]
async fn track_by_owner_name_missing_repo_returns_404(pool: PgPool) {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/repos/nobody/nothing"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let app = router::app(app_state_with(pool, &server.uri(), false));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/me/tracked/nobody/nothing")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
}

#[sqlx::test(migrations = "../../migrations")]
async fn invalid_json_body_returns_json_400(pool: PgPool) {
    let app = router::app(app_state(pool));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from("not valid json"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 400);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("error").is_some());
}

#[sqlx::test(migrations = "../../migrations")]
async fn unknown_route_returns_json_404(pool: PgPool) {
    let app = router::app(app_state(pool));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/this-route-does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("error").is_some());
}

#[sqlx::test(migrations = "../../migrations")]
async fn session_cookie_is_secure_when_configured(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();

    let app = router::app(app_state_with(pool, "https://api.github.com", true));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(format!("{{\"project_id\":\"{}\"}}", project.id)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    assert!(cookie_has_secure_flag(&response));
}

#[sqlx::test(migrations = "../../migrations")]
async fn session_cookie_is_not_secure_by_default(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();

    let app = router::app(app_state_with(pool, "https://api.github.com", false));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/me/tracked")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(format!("{{\"project_id\":\"{}\"}}", project.id)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    assert!(!cookie_has_secure_flag(&response));
}
