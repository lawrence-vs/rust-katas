//! 36 beginner: test the edges
//!
//! Think further, from the kata README:
//! 1. You add a new edge (a slice of one `u64::MAX`). In a table test, what does it cost to add it, compared with a new `#[test]` function per case?
//! 2. When one row fails, how does the `"case: {name}"` message help? What happens to the rows after it?
//! 3. Property testing (e.g. `proptest`) generates the rows for you. What property would you state for `mean_price`? (Hint: compare it with the smallest and largest price.)
//! 4. `cargo test --release` turns off overflow checks. What would the starting code have returned for the overflow row there, and would any test have noticed?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

// Average price over fills, widening to u128 so the sum can't overflow for realistic inputs.
fn mean_price(prices: &[u64]) -> Option<u64> {
    if prices.is_empty() {
        return None;
    }
    let sum: u128 = prices.iter().map(|p| u128::from(*p)).sum();
    u64::try_from(sum / prices.len() as u128).ok()
}

// Think further 1 and 2: a table of named edge cases; a failure prints which row broke.
fn main() {
    let cases: [(&str, &[u64], Option<u64>); 4] = [
        ("empty", &[], None),
        ("one price", &[7], Some(7)),
        ("two huge prices", &[u64::MAX, u64::MAX], Some(u64::MAX)),
        ("uneven", &[1, 2], Some(1)),
    ];
    for (name, input, expected) in cases {
        let got = mean_price(input);
        println!("{}: {} ({got:?})", name, if got == expected { "pass" } else { "FAIL" });
    }

    // Think further 3: a property to state instead of rows: the mean lies between the smallest and largest price.
    let prices = [10u64, 20, 30];
    let mean = mean_price(&prices).unwrap_or(0);
    let (lo, hi) = (prices.iter().min().copied().unwrap_or(0), prices.iter().max().copied().unwrap_or(0));
    println!("property holds: {}", lo <= mean && mean <= hi);
}
