use kata_errors_123::{Config, ConfigError, Defaults};

fn config() -> Config {
    Config::new(&[("region", "eu-west"), ("mode", "fast")])
}

#[test]
fn fallback_is_not_run_when_value_is_present() {
    let defaults = Defaults::new();
    let got = config().get_or_default("region", &defaults);
    assert_eq!(got, "eu-west");
    assert_eq!(
        defaults.built(),
        0,
        "fallback is not run when value is present, but it was built eagerly"
    );
}

#[test]
fn error_is_not_built_when_value_is_present() {
    let defaults = Defaults::new();
    let got = config().require("mode", &defaults);
    assert_eq!(got, Ok("fast".to_string()));
    assert_eq!(defaults.built(), 0, "error built although the key exists");
}

#[test]
fn fallback_runs_once_when_value_is_missing() {
    let defaults = Defaults::new();
    let got = config().get_or_default("zone", &defaults);
    assert_eq!(got, "default-zone");
    assert_eq!(defaults.built(), 1);
}

#[test]
fn error_is_built_once_when_value_is_missing() {
    let defaults = Defaults::new();
    let got = config().require("zone", &defaults);
    assert_eq!(
        got,
        Err(ConfigError::Missing {
            key: "zone".to_string()
        })
    );
    assert_eq!(defaults.built(), 1);
}

#[test]
fn present_value_is_returned_unchanged() {
    let defaults = Defaults::new();
    let cfg = Config::new(&[("name", "")]);
    assert_eq!(cfg.get_or_default("name", &defaults), "");
}

#[test]
fn error_message_names_the_key() {
    let defaults = Defaults::new();
    let err = config().require("zone", &defaults).unwrap_err();
    assert_eq!(err.to_string(), "missing config key zone");
}
