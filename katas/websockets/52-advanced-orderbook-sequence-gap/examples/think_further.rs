//! 52 advanced: an order book sequence gap
//!
//! Think further, from the kata README:
//! 1. Feeds that send `first..=last` update ids per message: what must the first delta after a snapshot straddle?
//! 2. The snapshot is a slow REST call while deltas keep arriving. Buffer them during the fetch: which do you replay, and what bounds the buffer (katas 21, 25)?
//! 3. A gap you can't see, such as a bug in `apply`: periodic checksums of the top N levels.
//! 4. A crossed book (best bid ≥ best ask) as another sign of lost state.
//! 5. A snapshot call that fails or is rate-limited: a fallible source, backoff, and what the book serves meanwhile.
//! 6. One connection, many symbols: per-symbol seq or per-connection seq?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
enum Apply {
    Ok,
    Gap { expected: u64, got: u64 },
    Stale,
}

struct Book {
    last_update: u64,
    bids: BTreeMap<u64, u64>,
}

impl Book {
    // Think further 1: a delta covering first..=last must straddle the snapshot id, or it is a gap.
    fn apply(&mut self, first: u64, last: u64, levels: &[(u64, u64)]) -> Apply {
        if last <= self.last_update {
            return Apply::Stale;
        }
        if first > self.last_update + 1 {
            return Apply::Gap { expected: self.last_update + 1, got: first };
        }
        for (price, qty) in levels {
            if *qty == 0 {
                self.bids.remove(price);
            } else {
                self.bids.insert(*price, *qty);
            }
        }
        self.last_update = last;
        Apply::Ok
    }

    // Think further 4: a crossed book is a sign of lost state, so check it after every apply.
    fn crossed(&self, best_ask: u64) -> bool {
        self.bids.keys().next_back().is_some_and(|best_bid| *best_bid >= best_ask)
    }
}

fn main() {
    let mut book = Book { last_update: 100, bids: BTreeMap::new() };

    println!("stale: {:?}", book.apply(95, 100, &[(50, 1)]));
    println!("contiguous: {:?}", book.apply(101, 102, &[(50, 5)]));
    println!("gap: {:?}", book.apply(105, 106, &[(51, 2)]));
    println!("bids: {:?}", book.bids);
    println!("crossed against ask 49: {}", book.crossed(49));
}
