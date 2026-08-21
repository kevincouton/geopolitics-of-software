use crate::{extract::Json as JsonBody, state::AppState};
use axum::{extract::Path, extract::State, Json as AxumJson};
use chassis::{connectors::github::Client, error::ApiError, projects, tracked};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

const SESSION_COOKIE: &str = "session_id";

fn ensure_user_id(cookies: &Cookies, secure: bool) -> String {
    if let Some(cookie) = cookies.get(SESSION_COOKIE) {
        let value = cookie.value().to_string();
        if !value.is_empty() {
            return value;
        }
    }
    let id = Uuid::new_v4().to_string();
    let cookie = Cookie::build((SESSION_COOKIE, id.clone()))
        .http_only(true)
        .same_site(tower_cookies::cookie::SameSite::Lax)
        .path("/")
        .secure(secure)
        .build();
    cookies.add(cookie);
    id
}

#[derive(Deserialize)]
pub struct TrackRequest {
    project_id: Uuid,
}

pub async fn list(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<AxumJson<Vec<tracked::TrackedProjectWithDetails>>, ApiError> {
    let user_id = ensure_user_id(&cookies, state.cfg.cookie_secure);
    let items = tracked::list_for_user(&state.pool, &user_id).await?;
    Ok(AxumJson(items))
}

pub async fn track(
    State(state): State<AppState>,
    cookies: Cookies,
    JsonBody(body): JsonBody<TrackRequest>,
) -> Result<AxumJson<tracked::TrackedProjectWithDetails>, ApiError> {
    let user_id = ensure_user_id(&cookies, state.cfg.cookie_secure);
    let item = tracked::track(&state.pool, &user_id, body.project_id).await?;
    Ok(AxumJson(item))
}

pub async fn track_by_owner_name(
    State(state): State<AppState>,
    cookies: Cookies,
    Path((owner, name)): Path<(String, String)>,
) -> Result<AxumJson<tracked::TrackedProjectWithDetails>, ApiError> {
    let user_id = ensure_user_id(&cookies, state.cfg.cookie_secure);
    let client = Client::with_base_url(
        state.cfg.github_token.clone(),
        state.cfg.github_base_url.clone(),
    );
    let repo = client.get_repo(&owner, &name).await?;
    let project = projects::upsert(&state.pool, &repo).await?;
    let item = tracked::track(&state.pool, &user_id, project.id).await?;
    Ok(AxumJson(item))
}

pub async fn untrack(
    State(state): State<AppState>,
    cookies: Cookies,
    Path(project_id): Path<Uuid>,
) -> Result<AxumJson<serde_json::Value>, ApiError> {
    let user_id = ensure_user_id(&cookies, state.cfg.cookie_secure);
    tracked::untrack(&state.pool, &user_id, project_id).await?;
    Ok(AxumJson(json!({"status":"ok"})))
}
