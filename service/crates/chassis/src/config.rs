#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub api_port: u16,
    pub github_token: Option<String>,
    pub github_base_url: String,
    pub cookie_secure: bool,
    pub cors_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let mut cors_origins = vec![
            "http://localhost:3000".into(),
            "http://localhost:8080".into(),
        ];
        if let Ok(extra) = std::env::var("NUXT_PUBLIC_API_URL") {
            cors_origins.push(extra);
        }

        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL"),
            api_port: std::env::var("API_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            github_token: std::env::var("GITHUB_TOKEN").ok(),
            github_base_url: std::env::var("GITHUB_API_URL")
                .unwrap_or_else(|_| "https://api.github.com".into()),
            cookie_secure: std::env::var("COOKIE_SECURE")
                .map(|v| v == "true")
                .unwrap_or(false),
            cors_origins,
        }
    }
}
