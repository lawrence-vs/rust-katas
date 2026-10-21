/// The mean of `prices`, rounded down.
///
/// # Panics
///
/// Panics with `no prices` if `prices` is empty.
///
/// CLUE: Both tests below pass. Which inputs did nobody try?
/// CLUE: What is the biggest total two `u64` prices can add up to, and which type can hold it?
/// CLUE: `empty_panics` passes today. Is the panic it catches the one the doc comment promises?
pub fn mean_price(prices: &[u64]) -> u64 {
    let total: u64 = prices.iter().sum();
    total / prices.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_of_two() {
        assert_eq!(mean_price(&[100, 200]), 150);
    }

    #[test]
    #[should_panic]
    fn empty_panics() {
        mean_price(&[]);
    }
}
