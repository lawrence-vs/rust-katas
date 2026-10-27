use std::fmt;

use sha2::{Digest, Sha256};

/// The deepest nesting of arrays and objects a body may have.
/// CLUE: Where must a check of this run so that 100_000 `[` never reach the stack?
pub const MAX_DEPTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FingerprintError {
    Invalid,
    DuplicateKey(String),
    TooDeep,
}

impl fmt::Display for FingerprintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => f.write_str("invalid JSON"),
            Self::DuplicateKey(key) => write!(f, "duplicate key {key:?}"),
            Self::TooDeep => write!(f, "nesting deeper than {MAX_DEPTH}"),
        }
    }
}

impl std::error::Error for FingerprintError {}

/// The body in one canonical form: sorted keys, one escape form, no whitespace.
/// CLUE: What does `Value` hold for a number that fits neither `i64` nor `u64`?
/// CLUE: What does `Value` keep when one object has two "a" keys?
pub fn canonical_json(body: &[u8]) -> Result<String, FingerprintError> {
    let value: serde_json::Value =
        serde_json::from_slice(body).map_err(|_| FingerprintError::Invalid)?;
    serde_json::to_string(&value).map_err(|_| FingerprintError::Invalid)
}

/// SHA-256 of the method, the path and the canonical body.
/// CLUE: Which two different (method, path) pairs feed the hasher the same bytes?
pub fn fingerprint(method: &str, path: &str, body: &[u8]) -> Result<[u8; 32], FingerprintError> {
    let canonical = canonical_json(body)?;
    let mut hasher = Sha256::new();
    hasher.update(method.as_bytes());
    hasher.update(path.as_bytes());
    hasher.update(canonical.as_bytes());
    Ok(hasher.finalize().into())
}
