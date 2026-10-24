use std::fmt;
use std::time::Duration;

use tower::{BoxError, Service, ServiceExt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookup {
    pub id: u64,
}

/// What the upstream can fail with.
/// CLUE: Which of these variants would a second attempt change?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpstreamError {
    /// Transient: try again later.
    Unavailable,
    /// Permanent: retrying cannot help.
    NotFound,
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpstreamError::Unavailable => write!(f, "upstream unavailable"),
            UpstreamError::NotFound => write!(f, "record not found"),
        }
    }
}

impl std::error::Error for UpstreamError {}

/// CLUE: Which tower layer reads each field?
#[derive(Debug, Clone)]
pub struct Config {
    pub per_try_timeout: Duration,
    /// Retries after the first attempt: total calls <= 1 + max_retries.
    pub max_retries: u32,
    /// Wait before the first retry; doubles for each later retry.
    pub first_backoff: Duration,
}

/// CLUE: The upstream answers NotFound. How many times will this ask again?
/// CLUE: The upstream never answers. What ends this await?
pub async fn fetch<S>(mut svc: S, req: Lookup, cfg: &Config) -> Result<String, BoxError>
where
    S: Service<Lookup, Response = String, Error = UpstreamError> + Clone,
{
    loop {
        match svc.ready().await?.call(req.clone()).await {
            Ok(body) => return Ok(body),
            Err(_) => continue,
        }
    }
}
