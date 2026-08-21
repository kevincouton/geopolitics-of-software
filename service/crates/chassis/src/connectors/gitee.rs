use crate::error::ApiError;

/// Locate a potential Gitee mirror for a GitHub repository.
///
/// MVP: this is a no-op stub to avoid external API calls in tests and
/// development. A real implementation can query
/// `https://gitee.com/api/v5/search/repositories?q={owner}%2F{name}` and
/// return the first matching `(owner, name)` tuple.
pub async fn find_mirror(_owner: &str, _name: &str) -> Result<Option<(String, String)>, ApiError> {
    Ok(None)
}
