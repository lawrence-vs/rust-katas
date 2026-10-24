use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// The key has no value and no usable default.
    Missing { key: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Missing { key } => write!(f, "missing config key {key}"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Builds fallbacks and counts how many it has built.
pub struct Defaults {
    built: Cell<usize>,
}

impl Defaults {
    pub fn new() -> Self {
        Defaults {
            built: Cell::new(0),
        }
    }

    /// Builds the default value for `key`; every call is counted.
    pub fn value_for(&self, key: &str) -> String {
        self.built.set(self.built.get() + 1);
        format!("default-{key}")
    }

    /// Builds the error for a missing `key`; every call is counted.
    pub fn error_for(&self, key: &str) -> ConfigError {
        self.built.set(self.built.get() + 1);
        ConfigError::Missing {
            key: key.to_string(),
        }
    }

    pub fn built(&self) -> usize {
        self.built.get()
    }
}

impl Default for Defaults {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Config {
    values: HashMap<String, String>,
}

impl Config {
    pub fn new(pairs: &[(&str, &str)]) -> Self {
        Config {
            values: pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }

    /// CLUE: Which arguments does Rust evaluate before `unwrap_or` is called?
    pub fn get_or_default(&self, key: &str, defaults: &Defaults) -> String {
        self.values
            .get(key)
            .cloned()
            .unwrap_or(defaults.value_for(key))
    }

    /// CLUE: `ok_or` takes a finished error value, not a way to make one.
    pub fn require(&self, key: &str, defaults: &Defaults) -> Result<String, ConfigError> {
        self.values.get(key).cloned().ok_or(defaults.error_for(key))
    }
}
