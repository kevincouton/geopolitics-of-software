use crate::{db::DbPool, error::ApiError, scoring::Score};
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct DailySnapshot {
    pub id: Uuid,
    pub project_id: Uuid,
    pub snapshot_date: NaiveDate,
    pub stars: i32,
    pub forks: i32,
    pub asia_readiness_score: i32,
    pub docs_score: i32,
    pub platform_score: i32,
    pub social_score: i32,
    pub community_score: i32,
}

pub async fn record(
    pool: &DbPool,
    project_id: Uuid,
    stars: i32,
    forks: i32,
    score: &Score,
) -> Result<DailySnapshot, ApiError> {
    sqlx::query_as::<_, DailySnapshot>(
        "INSERT INTO daily_snapshots (project_id, snapshot_date, stars, forks, asia_readiness_score, docs_score, platform_score, social_score, community_score)
         VALUES ($1, CURRENT_DATE, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (project_id, snapshot_date)
         DO UPDATE SET stars = EXCLUDED.stars, forks = EXCLUDED.forks,
                       asia_readiness_score = EXCLUDED.asia_readiness_score,
                       docs_score = EXCLUDED.docs_score, platform_score = EXCLUDED.platform_score,
                       social_score = EXCLUDED.social_score, community_score = EXCLUDED.community_score
         RETURNING *"
    )
    .bind(project_id)
    .bind(stars)
    .bind(forks)
    .bind(score.total)
    .bind(score.docs)
    .bind(score.platform)
    .bind(score.social)
    .bind(score.community)
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::Internal)
}

pub async fn list_for_project(
    pool: &DbPool,
    project_id: Uuid,
) -> Result<Vec<DailySnapshot>, ApiError> {
    sqlx::query_as::<_, DailySnapshot>(
        "SELECT * FROM daily_snapshots
         WHERE project_id = $1
         ORDER BY snapshot_date ASC",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
    .map_err(|_| ApiError::Internal)
}
