/// Decimal places in a price.
pub const SCALE: u32 = 4;
/// Minor units in one whole unit (10^SCALE).
pub const UNITS_PER_WHOLE: u64 = 10_000;

/// A price in ten-thousandths: `Price(123_400)` is 12.3400.
/// CLUE: What is 0.57 as an `f64`, exactly? What does `as u64` do to 5699.999999999999?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("price is empty")]
    Empty,
    #[error("price must be digits with at most one '.' and digits on both sides of it")]
    Malformed,
    #[error("price may contain only ASCII digits and one '.'")]
    InvalidDigit,
    #[error("price has more than 4 decimal places")]
    TooManyDecimals,
    #[error("price is too large")]
    Overflow,
}

impl std::str::FromStr for Price {
    type Err = ParseError;

    /// Parses a decimal string such as `" 12.3400 "` into ten-thousandths.
    /// CLUE: "1e3" and "NaN" both parse as `f64`. Which characters can a price actually contain?
    /// CLUE: `split_once('.')` gives two `&str`s. How many digits may the right one have, and what do you multiply a short one by?
    /// CLUE: "+5".parse::<u64>() is `Ok(5)`. Is `+` a digit?
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: f64 = s.trim().parse().map_err(|_| ParseError::InvalidDigit)?;
        Ok(Price((value * 10_000.0) as u64))
    }
}
