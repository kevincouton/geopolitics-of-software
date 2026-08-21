use api::{router, state::AppState};
use axum::{
    body::{to_bytes, Body},
    http::Request,
};
use chassis::{
    config::Config,
    connectors::github::{GithubRepo, Owner},
    projects,
};
use sqlx::PgPool;
use tower::ServiceExt;

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
    AppState {
        cfg: Config {
            database_url: "postgres://localhost/test".into(),
            api_port: 0,
            github_token: None,
        },
        pool,
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn list_projects_endpoint(pool: PgPool) {
    let repo = sample_repo();
    projects::upsert(&pool, &repo).await.unwrap();

    let app = router::app(app_state(pool));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/projects")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["github_name"], "hello");
}

#[sqlx::test(migrations = "../../migrations")]
async fn detail_project_endpoint(pool: PgPool) {
    let repo = sample_repo();
    projects::upsert(&pool, &repo).await.unwrap();

    let app = router::app(app_state(pool));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/projects/octocat/hello")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let project: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(project["github_owner"], "octocat");
    assert_eq!(project["github_name"], "hello");
}

#[sqlx::test(migrations = "../../migrations")]
async fn detail_missing_project_returns_404(pool: PgPool) {
    let app = router::app(app_state(pool));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/projects/nobody/nothing")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
}
