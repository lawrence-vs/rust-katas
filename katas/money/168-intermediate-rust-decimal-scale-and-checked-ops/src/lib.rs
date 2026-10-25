use rust_decimal::Decimal;
use serde::Deserialize;

/// Decimal places on every printed amount.
pub const AMOUNT_DP: u32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoneyError {
    /// The JSON is malformed, a field is missing or unknown, or an amount is not a decimal string.
    Parse,
    /// The exact result does not fit in a Decimal.
    Overflow,
}

/// One statement line: `qty` units at `unit_price`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Line {
    /// CLUE: A test sends `0.1` with no quotes. Which serde visitor receives it, and what type holds it on the way in?
    pub unit_price: Decimal,
    pub qty: Decimal,
}

/// Parses one line from JSON. Amounts are JSON strings ("0.1"), never JSON numbers.
pub fn parse_line(json: &str) -> Result<Line, MoneyError> {
    serde_json::from_str(json).map_err(|_| MoneyError::Parse)
}

/// What one line costs: `unit_price` x `qty`, exact.
pub fn line_total(line: &Line) -> Result<Decimal, MoneyError> {
    Ok(line.unit_price * line.qty)
}

/// The sum of every line total; zero for no lines.
///
/// CLUE: What does `+=` on a `Decimal` do when the sum does not fit? Read its `Add` impl.
pub fn statement_total(lines: &[Line]) -> Result<Decimal, MoneyError> {
    let mut total = Decimal::ZERO;
    for line in lines {
        total += line_total(line)?;
    }
    Ok(total)
}

/// Prints `amount` with exactly `AMOUNT_DP` decimal places.
/// Rounding: half-to-even when `amount` has more places, as settlement does in kata 87.
///
/// CLUE: `1.5 == 1.50`. What does `scale()` return for each, and which one does `to_string` print?
/// CLUE: `rescale` and `{:.4}` both change the places shown. What does each do with `Decimal::MAX`, and with `0.12346`?
pub fn format_amount(amount: Decimal) -> String {
    amount.to_string()
}
