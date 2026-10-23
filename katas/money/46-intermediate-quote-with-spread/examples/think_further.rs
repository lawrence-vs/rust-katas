//! 46 intermediate: quote with a spread
//!
//! Think further, from the kata README:
//! 1. An asset with 18 decimals (wei), or a quote currency with 0 decimals: do the scale constants and `u128` still fit?
//! 2. A fee on top of the spread: do you round once or twice, and where does the remainder go?
//! 3. The quote shows an ask price on a tick but charges a total. Must the two agree?
//! 4. Minimum notional: should a quote that rounds to 0 cents be an error?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

// Think further 2: round the ask up and the bid down, so the desk never gives away the spread.
fn quote(mid_cents: u64, half_spread_bps: u64) -> Option<(u64, u64)> {
    let half = u128::from(mid_cents).checked_mul(u128::from(half_spread_bps))?;
    let delta = half.div_ceil(10_000);
    let mid = u128::from(mid_cents);
    let bid = mid.checked_sub(delta)?;
    let ask = mid.checked_add(delta)?;
    Some((u64::try_from(bid).ok()?, u64::try_from(ask).ok()?))
}

fn main() {
    // Think further 1: u128 intermediates keep the product exact, even for a big mid.
    println!("mid 50_000_00 with 25 bps: {:?}", quote(5_000_000, 25));
    println!("mid u64::MAX with 25 bps: {:?}", quote(u64::MAX, 25));

    // Think further 4: a tiny mid makes the bid floor to 0, which should be flagged, not sent.
    match quote(1, 1) {
        Some((bid, ask)) => println!("tiny mid: bid {bid} ask {ask}"),
        None => println!("tiny mid: no quote"),
    }
}
