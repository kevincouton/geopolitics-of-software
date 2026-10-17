use api::{router, state::AppState};
use chassis::{config::Config, db};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_env();
    let _sentry = chassis::observability::init(
        &cfg.sentry_dsn,
        Some(concat!("geopolitics-of-software-api@", env!("CARGO_PKG_VERSION")).to_string()),
    );
    // With Sentry enabled, forward ERROR-level tracing events (and sampled
    // spans) to Sentry; without it, keep the plain fmt subscriber.
    use tracing_subscriber::prelude::*;
    if _sentry.is_some() {
        tracing_subscriber::fmt()
            .finish()
            .with(sentry::integrations::tracing::layer())
            .init();
    } else {
        tracing_subscriber::fmt().init();
    }
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
