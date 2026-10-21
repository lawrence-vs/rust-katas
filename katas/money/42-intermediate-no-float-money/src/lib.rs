/// An amount of money in dollars: `0.10` is ten cents.
/// CLUE: Can an `f64` hold one tenth exactly? Try `0.1 + 0.2 == 0.3`.
/// CLUE: What is the smallest amount this market deals in, and is every price a whole number of it?
pub type Money = f64;

/// Sums a list of prices.
/// CLUE: `u64` has `checked_add`. What does it return that `+` doesn't?
pub fn total(prices: &[Money]) -> Money {
    prices.iter().sum()
}

/// The cost of filling `qty` units at `price`.
pub fn fill_cost(price: Money, qty: u64) -> Money {
    price * qty as f64
}
