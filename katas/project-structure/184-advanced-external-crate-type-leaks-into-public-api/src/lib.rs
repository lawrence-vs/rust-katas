//! Prices for a shop's catalogue, parsed from text and summed.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

use rust_decimal::Decimal;

/// A price, as written: "1.50" stays "1.50".
///
/// CLUE: Who owns this type? When `rust_decimal` 2.0 ships, whose callers must change their code?
pub type Price = Decimal;

/// Why a price was refused.
///
/// CLUE: To name what is inside `Invalid`, which crate must a caller add to their own Cargo.toml?
#[derive(Debug)]
pub enum PriceError {
    /// The text is not a number.
    Invalid(rust_decimal::Error),
    /// The price is below zero.
    Negative,
    /// The total does not fit.
    Overflow,
}

impl fmt::Display for PriceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriceError::Invalid(e) => write!(f, "{e}"),
            PriceError::Negative => write!(f, "negative price"),
            PriceError::Overflow => write!(f, "total overflows"),
        }
    }
}

/// CLUE: Display prints the parser's message itself. What does `source()` return, and what should a caller walking the chain see?
impl Error for PriceError {}

impl From<rust_decimal::Error> for PriceError {
    fn from(e: rust_decimal::Error) -> Self {
        PriceError::Invalid(e)
    }
}

/// Parses a price written like "1.50". Refuses text that is not a number and prices below zero.
pub fn parse_price(input: &str) -> Result<Price, PriceError> {
    let value = Decimal::from_str(input)?;
    if value < Decimal::ZERO {
        return Err(PriceError::Negative);
    }
    Ok(value)
}

/// Adds prices. An empty list totals 0.
pub fn total(prices: &[Price]) -> Result<Price, PriceError> {
    prices.iter().try_fold(Decimal::ZERO, |sum, p| {
        sum.checked_add(*p).ok_or(PriceError::Overflow)
    })
}
