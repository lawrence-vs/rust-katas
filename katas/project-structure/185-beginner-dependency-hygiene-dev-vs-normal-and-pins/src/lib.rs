//! Counts written as decimal text and read back.
use std::num::ParseIntError;

/// CLUE: `itoa = "*"` accepts which versions, including ones not released yet? What would a fresh clone build next year?
pub fn format_count(n: u64) -> String {
    let mut buf = itoa::Buffer::new();
    buf.format(n).to_owned()
}

/// CLUE: Run `cargo tree -e normal`. Which crates in that list does `src/` never name, and which does only `tests/` use?
pub fn parse_count(s: &str) -> Result<u64, ParseIntError> {
    s.trim().parse()
}
