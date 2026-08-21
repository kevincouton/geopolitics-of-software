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
        cfg: chassis::config::Config {
            database_url: "postgres://localhost/test".into(),
            api_port: 0,
            github_token: None,
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
