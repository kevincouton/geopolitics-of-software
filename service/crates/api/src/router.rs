use crate::{handlers::health, handlers::projects, state::AppState};
use axum::{routing::get, Router};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health::health))
        .route("/projects", get(projects::list))
        .route("/projects/:owner/:name", get(projects::detail))
        .with_state(state)
}
