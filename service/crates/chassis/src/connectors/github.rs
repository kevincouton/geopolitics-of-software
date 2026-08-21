use crate::error::ApiError;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubRepo {
    pub owner: Owner,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub open_issues_count: i64,
    pub topics: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Owner {
    pub login: String,
}

pub struct Client {
    http: reqwest::Client,
    token: Option<String>,
    base_url: String,
}

impl Client {
    pub fn new(token: Option<String>) -> Self {
        Self::with_base_url(token, "https://api.github.com")
    }

    pub fn with_base_url(token: Option<String>, base_url: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            token,
            base_url: base_url.into(),
        }
    }

    pub async fn list_trending(&self, _language: &str) -> Result<Vec<GithubRepo>, ApiError> {
        // MVP: use GitHub search API sorted by stars for "created:>2025-01-01".
        // In production, scrape github.com/trending or use a dedicated service.
        let url = format!("{}/search/repositories", self.base_url);
        let mut req = self
            .http
            .get(&url)
            .query(&[
                ("q", "stars:>100"),
                ("sort", "stars"),
                ("order", "desc"),
                ("per_page", "30"),
            ])
            .header("User-Agent", "geosoft-trendboard");
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }
        let resp = req.send().await.map_err(|_| ApiError::Internal)?;
        if !resp.status().is_success() {
            return Err(ApiError::Internal);
        }
        let body: SearchResponse = resp.json().await.map_err(|_| ApiError::Internal)?;
        Ok(body.items)
    }
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    items: Vec<GithubRepo>,
}
