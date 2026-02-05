//! Load skills from AWS S3.
//!
//! Allows loading skill definitions stored as markdown files in an S3 bucket.
//! Each object should be a markdown file with YAML frontmatter (same format
//! as [`Skill::from_markdown`]).
//!
//! # Usage
//!
//! ```rust,no_run
//! use spider_skills::s3::S3SkillSource;
//! use spider_skills::SkillRegistry;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let source = S3SkillSource::new("my-skills-bucket").await;
//! let mut registry = SkillRegistry::new();
//! source.load_into(&mut registry, "skills/").await?;
//! # Ok(())
//! # }
//! ```

use crate::SkillRegistry;

/// Source for loading skills from an S3 bucket.
pub struct S3SkillSource {
    client: aws_sdk_s3::Client,
    bucket: String,
}

impl S3SkillSource {
    /// Create a new S3 skill source using default AWS credentials.
    pub async fn new(bucket: impl Into<String>) -> Self {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        Self {
            client: aws_sdk_s3::Client::new(&config),
            bucket: bucket.into(),
        }
    }

    /// Create from an existing S3 client.
    pub fn from_client(client: aws_sdk_s3::Client, bucket: impl Into<String>) -> Self {
        Self {
            client,
            bucket: bucket.into(),
        }
    }

    /// Load all `.md` files under the given prefix into the registry.
    ///
    /// Returns the list of skill names that were successfully loaded.
    pub async fn load_into(
        &self,
        registry: &mut SkillRegistry,
        prefix: &str,
    ) -> Result<Vec<String>, S3SkillError> {
        let mut loaded = Vec::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let mut req = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix);

            if let Some(token) = &continuation_token {
                req = req.continuation_token(token);
            }

            let resp = req.send().await.map_err(S3SkillError::ListObjects)?;

            for obj in resp.contents() {
                let key = match obj.key() {
                    Some(k) if k.ends_with(".md") => k,
                    _ => continue,
                };

                match self.load_one(registry, key).await {
                    Ok(name) => loaded.push(name),
                    Err(e) => {
                        log::warn!("Failed to load skill from s3://{}/{}: {}", self.bucket, key, e);
                    }
                }
            }

            if resp.is_truncated() == Some(true) {
                continuation_token = resp.next_continuation_token().map(|s| s.to_string());
            } else {
                break;
            }
        }

        Ok(loaded)
    }

    /// Load a single skill from an S3 object key.
    pub async fn load_one(
        &self,
        registry: &mut SkillRegistry,
        key: &str,
    ) -> Result<String, S3SkillError> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(S3SkillError::GetObject)?;

        let bytes = resp
            .body
            .collect()
            .await
            .map_err(|e| S3SkillError::ReadBody(e.to_string()))?;

        let text = String::from_utf8(bytes.to_vec())
            .map_err(|e| S3SkillError::ReadBody(e.to_string()))?;

        registry
            .load_markdown(&text)
            .ok_or(S3SkillError::ParseFailed(key.to_string()))
    }
}

/// Load a [`SkillRegistry`] with built-in web challenge skills and S3 skills.
///
/// Combines the built-in skills from [`web_challenges::registry()`] with
/// additional skills loaded from S3.
#[cfg(feature = "web_challenges")]
pub async fn with_builtin_and_s3(
    bucket: &str,
    prefix: &str,
) -> Result<SkillRegistry, S3SkillError> {
    let mut registry = crate::web_challenges::registry();
    let source = S3SkillSource::new(bucket).await;
    let loaded = source.load_into(&mut registry, prefix).await?;
    log::info!("Loaded {} skills from S3", loaded.len());
    Ok(registry)
}

/// Errors that can occur when loading skills from S3.
#[derive(Debug)]
pub enum S3SkillError {
    /// Failed to list objects in the bucket.
    ListObjects(aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Error>),
    /// Failed to get an object from the bucket.
    GetObject(aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::get_object::GetObjectError>),
    /// Failed to read the object body.
    ReadBody(String),
    /// Failed to parse the markdown skill file.
    ParseFailed(String),
}

impl std::fmt::Display for S3SkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListObjects(e) => write!(f, "S3 list objects error: {}", e),
            Self::GetObject(e) => write!(f, "S3 get object error: {}", e),
            Self::ReadBody(e) => write!(f, "S3 read body error: {}", e),
            Self::ParseFailed(key) => write!(f, "failed to parse skill from S3 key: {}", key),
        }
    }
}

impl std::error::Error for S3SkillError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ListObjects(e) => Some(e),
            Self::GetObject(e) => Some(e),
            _ => None,
        }
    }
}
