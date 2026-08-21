use chassis::{
    connectors::github, db::DbPool, error::ApiError, projects, scoring, snapshots,
};

pub async fn run(pool: &DbPool, github_token: Option<&str>) -> Result<(), ApiError> {
    let client = github::Client::new(github_token.map(|s| s.to_string()));
    let repos = client.list_trending("rust").await?;
    for repo in repos {
        let project = projects::upsert(pool, &repo).await?;
        let score = scoring::score(&project, &[]);
        snapshots::record(pool, project.id, project.stars, project.forks, &score).await?;
    }
    Ok(())
}
