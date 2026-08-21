use crate::{extract::Query, state::AppState};
use axum::{extract::Path, extract::State, Json};
use chassis::{error::ApiError, projects, snapshots};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default = "default_offset")]
    offset: i64,
}

fn default_limit() -> i64 {
    20
}

fn default_offset() -> i64 {
    0
}

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Vec<projects::Project>>, ApiError> {
    let projects = projects::list_paginated(&state.pool, q.limit, q.offset).await?;
    Ok(Json(projects))
}

pub async fn detail(
    State(state): State<AppState>,
    Path((owner, name)): Path<(String, String)>,
) -> Result<Json<projects::Project>, ApiError> {
    projects::by_owner_name(&state.pool, &owner, &name)
        .await?
        .ok_or(ApiError::NotFound)
        .map(Json)
}

pub async fn snapshots(
    State(state): State<AppState>,
    Path((owner, name)): Path<(String, String)>,
) -> Result<Json<Vec<snapshots::DailySnapshot>>, ApiError> {
    let project = projects::by_owner_name(&state.pool, &owner, &name)
        .await?
        .ok_or(ApiError::NotFound)?;
    let snaps = snapshots::list_for_project(&state.pool, project.id).await?;
    Ok(Json(snaps))
}
