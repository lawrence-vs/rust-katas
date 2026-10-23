//! 08 intermediate: `?` needs a From
//!
//! Think further, from the kata README:
//! 1. Add a second key, `retries`, parsed as a `u8`. Both parses fail with the same error type. With your fix, can a caller tell which key was bad? What would it take?
//! 2. The config now comes from a file, so reading it can fail with `std::io::Error`. What changes in `read_timeout`'s signature and in `ConfigError`? Then look at `#[derive(PartialEq)]`: does it still compile, and what would the tests compare instead?
//! 3. Try `timeout_ms` set to `"0"`, `"-5"` and `"18446744073709551616"`. Which variant does each land in? Should a zero timeout be an error of its own?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    BadTimeout(ParseIntError),
    BadRetries(ParseIntError),
}

// Think further 1: a separate variant per key tells the caller which key was bad.
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::BadTimeout(e) => write!(f, "timeout_ms: {e}"),
            ConfigError::BadRetries(e) => write!(f, "retries: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

// With From, `?` converts the error on the way out; without it, every call site needs map_err.
fn parse_timeout(s: &str) -> Result<u64, ConfigError> {
    s.parse::<u64>().map_err(ConfigError::BadTimeout)
}

fn parse_retries(s: &str) -> Result<u8, ConfigError> {
    s.parse::<u8>().map_err(ConfigError::BadRetries)
}

fn read_config(timeout: &str, retries: &str) -> Result<(u64, u8), ConfigError> {
    Ok((parse_timeout(timeout)?, parse_retries(retries)?))
}

fn main() {
    // Think further 3: which variant does each input land in?
    for input in ["0", "-5", "18446744073709551616", "250"] {
        match parse_timeout(input) {
            Ok(v) => println!("{input:>22} => ok {v}"),
            Err(e) => println!("{input:>22} => {e}"),
        }
    }

    match read_config("500", "300") {
        Ok(v) => println!("config {v:?}"),
        Err(e) => println!("config error: {e}"),
    }
}
