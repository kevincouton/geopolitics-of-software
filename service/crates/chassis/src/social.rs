use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Mention {
    pub platform: String,
    pub url: String,
    pub title: Option<String>,
    pub engagement_score: i32,
    pub sentiment: Option<String>,
}
