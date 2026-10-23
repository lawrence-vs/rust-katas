//! 70 intermediate: parse a decimal string
//!
//! Think further, from the kata README:
//! 1. Negative prices (spreads, some futures): what changes in the type and the grammar?
//! 2. A scale per instrument (8 places for BTC, 0 for JPY): could it be `Price<const SCALE: u32>`?
//! 3. A `Display` that round-trips to `"12.3400"`.
//! 4. serde `#[serde(try_from = "&str")]` at the JSON boundary.
//! 5. A CSV of trades with one bad row: skip it or stop, and how do you report the line number?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Price {
    minor: i64,
    scale: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError(String);

// Think further 1: a sign is allowed here, which the grammar must say explicitly.
impl FromStr for Price {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || ParseError(s.to_string());
        let (negative, body) = match s.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, s),
        };
        let (whole, frac) = body.split_once('.').unwrap_or((body, ""));
        if frac.len() > 8 || !frac.bytes().all(|b| b.is_ascii_digit()) {
            return Err(err());
        }
        let scale = frac.len() as u32;
        let digits = format!("{whole}{frac}");
        let magnitude: i64 = digits.parse().map_err(|_| err())?;
        let minor = if negative { -magnitude } else { magnitude };
        Ok(Price { minor, scale })
    }
}

// Think further 3: Display prints the scale back, so "12.3400" round-trips.
impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.minor < 0 { "-" } else { "" };
        let digits = self.minor.unsigned_abs().to_string();
        let scale = self.scale as usize;
        let padded = format!("{digits:0>width$}", width = scale + 1);
        let (whole, frac) = padded.split_at(padded.len() - scale);
        if scale == 0 {
            write!(f, "{sign}{whole}")
        } else {
            write!(f, "{sign}{whole}.{frac}")
        }
    }
}

fn main() {
    for input in ["12.3400", "-0.5", "42", "1.2.3", "7.123456789"] {
        match input.parse::<Price>() {
            Ok(p) => println!("{input:>12} => {p:?} => {p}"),
            Err(e) => println!("{input:>12} => error {e:?}"),
        }
    }
}
