//! Totals for amounts that arrive from outside as single bytes.

/// AmountError says why two amounts could not be added.
#[derive(Debug, PartialEq, Eq)]
pub enum AmountError {
    Overflow,
}

/// add_amounts adds two user-supplied amounts.
///
/// CLUE: `+` on a `u8` checks for overflow in one build profile and wraps in another. Which, and why can your caller not tell?
pub fn add_amounts(a: u8, b: u8) -> Result<u8, AmountError> {
    Ok(a + b)
}

/// total returns the sum of all amounts, or the first error.
///
/// CLUE: Which integer methods name their overflow behaviour instead of leaving it to the profile?
pub fn total(amounts: &[u8]) -> Result<u8, AmountError> {
    let mut sum = 0u8;
    for &amount in amounts {
        sum = add_amounts(sum, amount)?;
    }
    Ok(sum)
}
