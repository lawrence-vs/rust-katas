//! 47 intermediate: a stale quote accepted
//!
//! Think further, from the kata README:
//! 1. Asymmetric last look: reject only moves against the house. Is that fair?
//! 2. A quote accepted twice: single-use quote IDs (see kata 50).
//! 3. `now` earlier than the issue time, because a clock stepped backwards or a client sent the timestamp.
//! 4. A tolerance in ticks instead of bps.

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

#[derive(Debug, PartialEq, Eq)]
enum QuoteError {
    Stale,
    ClockBackwards,
}

// Think further 3: a timestamp earlier than the issue time is a clock problem, reported as its own error.
fn check_fresh(issued_ms: u64, now_ms: u64, max_age_ms: u64) -> Result<(), QuoteError> {
    if now_ms < issued_ms {
        return Err(QuoteError::ClockBackwards);
    }
    if now_ms - issued_ms > max_age_ms {
        return Err(QuoteError::Stale);
    }
    Ok(())
}

// Think further 1: asymmetric last look rejects only when the market moved against the house.
fn accept_at_price(quoted: u64, market: u64, client_buys: bool) -> bool {
    if client_buys {
        market <= quoted
    } else {
        market >= quoted
    }
}

fn main() {
    println!("fresh: {:?}", check_fresh(1_000, 1_200, 500));
    println!("stale: {:?}", check_fresh(1_000, 2_000, 500));
    println!("clock stepped back: {:?}", check_fresh(1_000, 900, 500));

    println!("buy, market fell: {}", accept_at_price(100, 99, true));
    println!("buy, market rose: {}", accept_at_price(100, 101, true));
}
