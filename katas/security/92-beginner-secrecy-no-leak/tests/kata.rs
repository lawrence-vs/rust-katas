use kata_security_92::{ApiError, Config};

const ENDPOINT: &str = "https://api.example.com";
const KEY: &str = "hunter2-hunter2-hunter2";

fn config() -> Config {
    Config::new(ENDPOINT, KEY)
}

#[test]
fn debug_output_hides_the_api_key() {
    let cfg = config();
    let dbg = format!("{:?}", cfg);
    assert!(
        !dbg.contains(KEY),
        "the API key leaked into Debug output: {dbg}"
    );
    assert!(dbg.contains(ENDPOINT));
    assert!(
        dbg.contains("[REDACTED]"),
        "expected the key to show as [REDACTED]: {dbg}"
    );
}

#[test]
fn the_log_line_hides_the_api_key() {
    let line = config().log_line();
    assert!(
        !line.contains(KEY),
        "the API key leaked into the log line: {line}"
    );
}

#[test]
fn a_rejected_request_does_not_leak_the_key() {
    let err = config().check_status(401).unwrap_err();
    let shown = err.to_string();
    let debugged = format!("{err:?}");
    assert!(
        !shown.contains(KEY),
        "the API key leaked into an error: {shown}"
    );
    assert!(
        !debugged.contains(KEY),
        "the API key leaked into an error: {debugged}"
    );
    for text in [&shown, &debugged] {
        assert!(text.contains("401"));
        assert!(text.contains(ENDPOINT));
    }
    assert!(matches!(err, ApiError::Rejected { .. }));
}

#[test]
fn auth_header_still_carries_the_key() {
    assert_eq!(config().auth_header(), format!("Bearer {KEY}"));
}

#[test]
fn a_2xx_status_is_ok() {
    let cfg = config();
    assert!(cfg.check_status(200).is_ok());
    assert!(cfg.check_status(204).is_ok());
    assert!(cfg.check_status(199).is_err());
    assert!(cfg.check_status(300).is_err());
}
