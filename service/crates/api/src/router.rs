use crate::{
    handlers::{health, projects, tracked},
    state::AppState,
};
use axum::{
    http::{header, Method},
    routing::{delete, get, post},
    Router,
};
use chassis::error::ApiError;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

async fn fallback() -> ApiError {
    ApiError::NotFound
}

async fn method_not_allowed() -> ApiError {
    ApiError::MethodNotAllowed
}

pub fn app(state: AppState) -> Router {
    let allowed_origins: Vec<_> = state
        .cfg
        .cors_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    let allow_origin = if allowed_origins.is_empty() {
        AllowOrigin::exact("http://localhost:3000".parse().unwrap())
    } else {
        AllowOrigin::list(allowed_origins)
    };

    let cors = CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE]);

    Router::new()
        .route("/healthz", get(health::health))
        .route("/projects", get(projects::list))
        .route("/projects/:owner/:name", get(projects::detail))
        .route("/projects/:owner/:name/snapshots", get(projects::snapshots))
        .route("/me/tracked", get(tracked::list).post(tracked::track))
        .route(
            "/me/tracked/:owner/:name",
            post(tracked::track_by_owner_name),
        )
        .route("/me/tracked/:id", delete(tracked::untrack))
        .fallback(fallback)
        .method_not_allowed_fallback(method_not_allowed)
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .with_state(state)
}
