use std::collections::HashMap;

use kata_errors_08::{ConfigError, read_timeout};

fn cfg(value: Option<&str>) -> HashMap<String, String> {
    value
        .map(|v| ("timeout_ms".to_string(), v.to_string()))
        .into_iter()
        .collect()
}

#[test]
fn reads_a_number() {
    assert_eq!(read_timeout(&cfg(Some("250"))), Ok(250));
}

#[test]
fn missing_key() {
    assert_eq!(
        read_timeout(&cfg(None)),
        Err(ConfigError::MissingKey("timeout_ms"))
    );
}

#[test]
fn bad_number_keeps_the_parse_error() {
    assert!(matches!(
        read_timeout(&cfg(Some("soon"))),
        Err(ConfigError::BadNumber(_))
    ));
}
