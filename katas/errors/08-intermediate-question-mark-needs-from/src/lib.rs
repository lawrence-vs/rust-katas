use std::collections::HashMap;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingKey(&'static str),
    BadNumber(ParseIntError),
}

/// Reads `timeout_ms` from a parsed config file.
/// CLUE: `?` converts the error before returning it. Which trait does it call to convert?
/// CLUE: Is there a conversion from `ParseIntError` into `ConfigError` yet?
pub fn read_timeout(cfg: &HashMap<String, String>) -> Result<u64, ConfigError> {
    let raw = cfg
        .get("timeout_ms")
        .ok_or(ConfigError::MissingKey("timeout_ms"))?;
    let ms = raw.parse::<u64>()?;
    Ok(ms)
}
