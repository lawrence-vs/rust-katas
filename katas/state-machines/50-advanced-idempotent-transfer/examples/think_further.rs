//! 50 advanced: an idempotent transfer
//!
//! Think further, from the kata README:
//! 1. Rebuilding `Transfers` from a stored log after a restart: replay must not call `send`. How do you split deciding (the transition) from doing (the side effect)?
//! 2. An `Unknown` that never resolves: reconciling by asking the rail about the key, or a `NeedsReview` state.
//! 3. The key map grows forever: key retention (TTL), and what a retry after eviction does.
//! 4. Two threads applying the same `Requested` at once (kata 51).
//! 5. Single-use quote IDs from kata 47 as the same pattern.

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    Requested,
    Sent,
    Unknown,
    Settled,
}

// Think further 1: decide() only computes the next state; the caller does the side effect once.
fn decide(current: &State, rail_said_ok: bool) -> State {
    match (current, rail_said_ok) {
        (State::Requested, true) => State::Sent,
        (State::Requested, false) => State::Unknown,
        (State::Sent, _) => State::Settled,
        (other, _) => other.clone(),
    }
}

struct Transfers {
    by_key: HashMap<String, State>,
}

impl Transfers {
    // Think further 3 and in the wild: the same key returns the stored state, so a retry never sends twice.
    fn request(&mut self, key: &str) -> State {
        self.by_key.entry(key.to_string()).or_insert(State::Requested).clone()
    }
}

fn main() {
    let mut t = Transfers { by_key: HashMap::new() };

    let first = t.request("tx-1");
    println!("first request: {first:?}");
    let next = decide(&first, true);
    t.by_key.insert("tx-1".to_string(), next.clone());
    println!("after rail ok: {next:?}");

    // A retry with the same key does not start a second transfer.
    println!("retry same key: {:?}", t.request("tx-1"));

    // Think further 2: a rail that timed out leaves the transfer Unknown, not failed.
    let unknown = decide(&State::Requested, false);
    println!("rail timed out: {unknown:?}");
    println!("settled stays settled: {:?}", decide(&State::Settled, true));
}
