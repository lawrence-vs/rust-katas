use rust_decimal::Decimal;

/// Decimal places in a cent amount.
pub const CENT_DP: u32 = 2;
/// Basis points in 100%.
pub const BPS_PER_UNIT: u32 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoneyError {
    /// The input is not a plain decimal number ("", "abc", "NaN", "inf", "1,5").
    Parse,
    /// The exact result does not fit in a Decimal.
    Overflow,
}

/// CLUE: What is 1.005 as an f64, exactly? And what does `.round()` do with exactly 12.5?
/// CLUE: `as i64` never fails. What should 7.9e28 x 2 be?
fn to_cents(value: f64) -> Decimal {
    Decimal::new((value * 100.0).round() as i64, 2)
}

/// The fee on `notional` at `fee_bps`, rounded to the cent.
/// Rounding: half-up. A tie goes away from zero, as the published fee schedule states.
///
/// CLUE: "NaN" and "inf" parse as f64. What does `Decimal::from_str` do with them?
pub fn fee(notional: &str, fee_bps: u32) -> Result<Decimal, MoneyError> {
    let notional: f64 = notional.parse().map_err(|_| MoneyError::Parse)?;
    Ok(to_cents(notional * fee_bps as f64 / BPS_PER_UNIT as f64))
}

/// What `qty` at `price` settles for, rounded to the cent.
/// Rounding: half-to-even (banker's), so ties across many fills don't bias the total.
///
/// CLUE: `round_dp` already rounds one way. Which way, and is it the one each doc comment promises?
pub fn settle(price: &str, qty: &str) -> Result<Decimal, MoneyError> {
    let price: f64 = price.parse().map_err(|_| MoneyError::Parse)?;
    let qty: f64 = qty.parse().map_err(|_| MoneyError::Parse)?;
    Ok(to_cents(price * qty))
}
