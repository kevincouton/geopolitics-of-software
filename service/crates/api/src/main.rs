use api::{router, state::AppState};
use chassis::{config::Config, db};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cfg = Config::from_env();
    let pool = db::connect(&cfg.database_url).await?;
    db::migrate(&pool).await?;
    let state = AppState {
        cfg: cfg.clone(),
        pool,
    };
    let app = router::app(state);
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", cfg.api_port)).await?;
    tracing::info!("API listening on :{}", cfg.api_port);
    axum::serve(listener, app).await?;
    Ok(())
}
