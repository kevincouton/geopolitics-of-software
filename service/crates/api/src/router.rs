use crate::{
    handlers::{health, projects, tracked},
    state::AppState,
};
use axum::{
    routing::{delete, get},
    Router,
};
use tower_cookies::CookieManagerLayer;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health::health))
        .route("/projects", get(projects::list))
        .route("/projects/:owner/:name", get(projects::detail))
        .route("/projects/:owner/:name/snapshots", get(projects::snapshots))
        .route("/me/tracked", get(tracked::list).post(tracked::track))
        .route("/me/tracked/:id", delete(tracked::untrack))
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
