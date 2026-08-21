use chassis::{config::Config, db::DbPool};

#[derive(Clone)]
pub struct AppState {
    pub cfg: Config,
    pub pool: DbPool,
}
