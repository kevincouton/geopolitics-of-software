use crate::{error::ApiError, social::Mention};

/// Stub social-platform scraper.
///
/// Real implementations for Juejin, Zhihu, V2EX and Bilibili will be added
/// behind feature flags or dedicated adapters later. For now this avoids
/// making any outbound scraping calls.
pub async fn search_mentions(platform: &str, query: &str) -> Result<Vec<Mention>, ApiError> {
    let _ = (platform, query);
    Ok(Vec::new())
}
