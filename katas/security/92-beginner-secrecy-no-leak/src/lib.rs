use std::fmt;

/// CLUE: #[derive(Debug)] prints every field. Which of these fields should never be printed?
#[derive(Debug)]
pub struct Config {
    endpoint: String,
    api_key: String,
}

impl Config {
    pub fn new(endpoint: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            api_key: api_key.into(),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// The one place the key is read. Returns "Bearer <key>".
    ///
    /// CLUE: This is the one caller that needs the raw key. How could every other read become impossible to write by accident?
    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    /// What the client logs at startup: built from `{self:?}`.
    pub fn log_line(&self) -> String {
        format!("loaded config: {self:?}")
    }

    /// 2xx is Ok(()); anything else is Err(ApiError::Rejected { .. }).
    pub fn check_status(&self, status: u16) -> Result<(), ApiError> {
        if (200..300).contains(&status) {
            Ok(())
        } else {
            Err(ApiError::Rejected {
                endpoint: self.endpoint.clone(),
                status,
                api_key: self.api_key.clone(),
            })
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    /// CLUE: Who reads this error, and do they need the key to act on it?
    Rejected {
        endpoint: String,
        status: u16,
        api_key: String,
    },
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Rejected {
                endpoint,
                status,
                api_key,
            } => write!(
                f,
                "request to {endpoint} rejected with {status} (key {api_key})"
            ),
        }
    }
}

impl std::error::Error for ApiError {}
