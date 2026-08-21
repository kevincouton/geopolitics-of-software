use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chassis::{error::ApiError, tracked};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

const SESSION_COOKIE: &str = "session_id";

fn ensure_user_id(cookies: &Cookies) -> String {
    if let Some(cookie) = cookies.get(SESSION_COOKIE) {
        let value = cookie.value().to_string();
        if !value.is_empty() {
            return value;
        }
    }
    let id = Uuid::new_v4().to_string();
    cookies.add(Cookie::new(SESSION_COOKIE, id.clone()));
    id
}

#[derive(Deserialize)]
pub struct TrackRequest {
    project_id: Uuid,
}

pub async fn list(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<Json<Vec<tracked::TrackedProjectWithDetails>>, ApiError> {
    let user_id = ensure_user_id(&cookies);
    let items = tracked::list_for_user(&state.pool, &user_id).await?;
    Ok(Json(items))
}

pub async fn track(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(body): Json<TrackRequest>,
) -> Result<Json<tracked::TrackedProjectWithDetails>, ApiError> {
    let user_id = ensure_user_id(&cookies);
    let item = tracked::track(&state.pool, &user_id, body.project_id).await?;
    Ok(Json(item))
}

pub async fn untrack(
    State(state): State<AppState>,
    cookies: Cookies,
    Path(project_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let user_id = ensure_user_id(&cookies);
    tracked::untrack(&state.pool, &user_id, project_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
