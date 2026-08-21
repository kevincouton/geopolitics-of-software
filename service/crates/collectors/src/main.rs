mod scoring_job;

use chassis::{config::Config, db};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cfg = Config::from_env();
    let pool = db::connect(&cfg.database_url).await?;
    db::migrate(&pool).await?;
    scoring_job::run(&pool, cfg.github_token.as_deref()).await?;
    Ok(())
}
