use crate::{db::DbPool, error::ApiError};
use chrono::Utc;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct TrackedProjectWithDetails {
    pub id: Uuid,
    pub github_owner: String,
    pub github_name: String,
    pub gitee_owner: Option<String>,
    pub gitee_name: Option<String>,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub description: Option<String>,
    pub stars: i32,
    pub forks: i32,
    pub open_issues: i32,
    pub has_chinese_readme: bool,
    pub has_gitee_mirror: bool,
    #[sqlx(default)]
    pub score: Option<i32>,
    pub tracked_at: chrono::DateTime<Utc>,
}

pub async fn list_for_user(
    pool: &DbPool,
    user_id: &str,
) -> Result<Vec<TrackedProjectWithDetails>, ApiError> {
    sqlx::query_as::<_, TrackedProjectWithDetails>(
        "SELECT p.*, ds.asia_readiness_score AS score, t.created_at AS tracked_at
         FROM tracked_projects t
         JOIN projects p ON p.id = t.project_id
         LEFT JOIN LATERAL (
             SELECT asia_readiness_score
             FROM daily_snapshots
             WHERE project_id = p.id
             ORDER BY snapshot_date DESC
             LIMIT 1
         ) ds ON true
         WHERE t.user_id = $1
         ORDER BY t.created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|_| ApiError::Internal)
}

pub async fn track(
    pool: &DbPool,
    user_id: &str,
    project_id: Uuid,
) -> Result<TrackedProjectWithDetails, ApiError> {
    sqlx::query_as::<_, TrackedProjectWithDetails>(
        "WITH inserted AS (
             INSERT INTO tracked_projects (user_id, project_id)
             VALUES ($1, $2)
             ON CONFLICT (user_id, project_id) DO UPDATE SET created_at = EXCLUDED.created_at
             RETURNING project_id, created_at
         )
         SELECT p.*, ds.asia_readiness_score AS score, i.created_at AS tracked_at
         FROM inserted i
         JOIN projects p ON p.id = i.project_id
         LEFT JOIN LATERAL (
             SELECT asia_readiness_score
             FROM daily_snapshots
             WHERE project_id = p.id
             ORDER BY snapshot_date DESC
             LIMIT 1
         ) ds ON true",
    )
    .bind(user_id)
    .bind(project_id)
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::Internal)
}

pub async fn untrack(pool: &DbPool, user_id: &str, project_id: Uuid) -> Result<(), ApiError> {
    let result = sqlx::query("DELETE FROM tracked_projects WHERE user_id = $1 AND project_id = $2")
        .bind(user_id)
        .bind(project_id)
        .execute(pool)
        .await
        .map_err(|_| ApiError::Internal)?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(())
}
