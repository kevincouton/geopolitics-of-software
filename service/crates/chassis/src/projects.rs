use crate::{connectors::github::GithubRepo, db::DbPool, error::ApiError};
use chrono::Utc;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Project {
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
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

pub async fn upsert(pool: &DbPool, repo: &GithubRepo) -> Result<Project, ApiError> {
    sqlx::query_as::<_, Project>(
        "INSERT INTO projects (github_owner, github_name, description, language, stars, forks, open_issues, topics)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (github_owner, github_name)
         DO UPDATE SET description = EXCLUDED.description, language = EXCLUDED.language,
                       stars = EXCLUDED.stars, forks = EXCLUDED.forks, open_issues = EXCLUDED.open_issues,
                       topics = EXCLUDED.topics, updated_at = NOW()
         RETURNING id, github_owner, github_name, gitee_owner, gitee_name, language, topics, description,
                   stars, forks, open_issues, has_chinese_readme, has_gitee_mirror, created_at, updated_at",
    )
    .bind(&repo.owner.login)
    .bind(&repo.name)
    .bind(&repo.description)
    .bind(&repo.language)
    .bind(repo.stargazers_count as i32)
    .bind(repo.forks_count as i32)
    .bind(repo.open_issues_count as i32)
    .bind(&repo.topics)
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::Internal)
}

pub async fn list(pool: &DbPool, limit: i64) -> Result<Vec<Project>, ApiError> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY stars DESC LIMIT $1")
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|_| ApiError::Internal)
}

pub async fn by_owner_name(
    pool: &DbPool,
    owner: &str,
    name: &str,
) -> Result<Option<Project>, ApiError> {
    sqlx::query_as::<_, Project>(
        "SELECT * FROM projects WHERE github_owner = $1 AND github_name = $2",
    )
    .bind(owner)
    .bind(name)
    .fetch_optional(pool)
    .await
    .map_err(|_| ApiError::Internal)
}
