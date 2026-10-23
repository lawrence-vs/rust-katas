//! 62 advanced: out-of-order and duplicate events
//!
//! Think further, from the kata README:
//! 1. Two different messages with the same version.
//! 2. A gap that never fills: a timeout, then a resync from the order-status endpoint (kata 52).
//! 3. Buffering by distance as well as by count.
//! 4. Total memory across many orders (kata 21).
//! 5. `CumQty`-style fills, where a replayed fill is not harmless.
//! 6. Restart and replay (kata 50).
//! 7. A version at `u64::MAX`.

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
enum Applied {
    Now(u64),
    Buffered,
    Duplicate,
}

// Buffer events that arrive early, keyed by version so they can be replayed in order.
struct Order {
    version: u64,
    pending: BTreeMap<u64, &'static str>,
}

impl Order {
    // Think further 2: a duplicate (version at or below what we have) is ignored, not applied twice.
    fn receive(&mut self, version: u64, event: &'static str) -> Applied {
        if version <= self.version {
            return Applied::Duplicate;
        }
        if version > self.version + 1 {
            self.pending.insert(version, event);
            return Applied::Buffered;
        }
        self.version = version;
        // Think further 6: after a gap fills, the buffered events that now line up apply too.
        while let Some(_next) = self.pending.remove(&(self.version + 1)) {
            self.version += 1;
        }
        Applied::Now(self.version)
    }
}

fn main() {
    let mut order = Order { version: 0, pending: BTreeMap::new() };
    println!("v2 first: {:?}", order.receive(2, "partial fill"));
    println!("v1 fills the gap: {:?}", order.receive(1, "new"));
    println!("v2 again: {:?}", order.receive(2, "partial fill"));

    // Think further 1: two different events with the same version are a data error, not a duplicate.
    println!("v3 cancel: {:?}", order.receive(3, "cancel"));
}
