//! 09 advanced: `Option` loses the why
//!
//! Think further, from the kata README:
//! 1. What does your parser return for `"ada, 36, admin"`, `"ada,36"` and `" , 36"`? Decide which should be errors, and whether your variants can say so.
//! 2. What does `load_user("")` return? Trace `split(',')` on an empty string: was the name really there?
//! 3. For `"ada, old"`, loop on `source()` and print each error in the chain. Now suppose the top-level message also included the parse error's text. What would a log line that prints the whole chain look like?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
enum UserError {
    #[error("missing name")]
    MissingName,
    #[error("bad age")]
    BadAge(#[source] ParseIntError),
}

// Think further 1: each failure has its own variant, so the caller learns why.
fn parse_user(line: &str) -> Result<(String, u32), UserError> {
    let mut parts = line.split(',');
    let name = parts.next().unwrap_or("").trim();
    if name.is_empty() {
        return Err(UserError::MissingName);
    }
    let age = parts.next().unwrap_or("").trim();
    let age = age.parse().map_err(UserError::BadAge)?;
    Ok((name.to_string(), age))
}

fn main() {
    for line in ["ada, 36", "ada, old", " , 36", ""] {
        match parse_user(line) {
            Ok(user) => println!("{line:?} => {user:?}"),
            Err(err) => {
                // Think further 3: walk source() so the whole chain is in the log line.
                let mut chain = err.to_string();
                let mut cause = err.source();
                while let Some(c) = cause {
                    chain.push_str(&format!(": {c}"));
                    cause = c.source();
                }
                println!("{line:?} => error: {chain}");
            }
        }
    }
}
