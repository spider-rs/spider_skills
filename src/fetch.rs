//! Fetch skills from remote URLs.
//!
//! Allows loading skill definitions from HTTP endpoints at runtime.
//! Skills are expected to be in markdown format with YAML frontmatter.

use crate::SkillRegistry;

/// Fetch a skill from a URL and add it to the registry.
///
/// The URL should return markdown with YAML frontmatter.
/// Returns the skill name if successfully loaded.
pub async fn fetch_skill(registry: &mut SkillRegistry, url: &str) -> Result<String, FetchError> {
    let response = reqwest::get(url).await.map_err(FetchError::Http)?;
    let text = response.text().await.map_err(FetchError::Http)?;
    registry
        .load_markdown(&text)
        .ok_or(FetchError::ParseFailed)
}

/// Fetch multiple skills from URLs.
///
/// Returns a list of (url, result) pairs.
pub async fn fetch_skills(
    registry: &mut SkillRegistry,
    urls: &[&str],
) -> Vec<(String, Result<String, FetchError>)> {
    let mut results = Vec::with_capacity(urls.len());
    for url in urls {
        let result = fetch_skill(registry, url).await;
        results.push((url.to_string(), result));
    }
    results
}

/// Error type for skill fetching.
#[derive(Debug)]
pub enum FetchError {
    /// HTTP request failed.
    Http(reqwest::Error),
    /// Markdown parsing failed (invalid frontmatter or missing name).
    ParseFailed,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP error: {}", e),
            Self::ParseFailed => write!(f, "failed to parse skill markdown"),
        }
    }
}

impl std::error::Error for FetchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Http(e) => Some(e),
            Self::ParseFailed => None,
        }
    }
}
