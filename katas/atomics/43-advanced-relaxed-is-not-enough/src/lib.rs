//! A one-shot price snapshot: one thread publishes it, others read it.
//!
//! The atomics come from `loom`, so `tests/kata.rs` can try every
//! interleaving and every value the memory model lets a load return.

use loom::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// One price and quantity, published once by a writer thread.
pub struct Snapshot {
    price: AtomicU64,
    qty: AtomicU64,
    ready: AtomicBool,
}

impl Default for Snapshot {
    fn default() -> Self {
        Self {
            price: AtomicU64::new(0),
            qty: AtomicU64::new(0),
            ready: AtomicBool::new(false),
        }
    }
}

impl Snapshot {
    /// Writes the data, then raises the flag.
    /// CLUE: `Relaxed` keeps each atomic consistent with itself. What does it promise about the other atomics?
    pub fn publish(&self, price: u64, qty: u64) {
        self.price.store(price, Ordering::Relaxed);
        self.qty.store(qty, Ordering::Relaxed);
        self.ready.store(true, Ordering::Relaxed);
    }

    /// Returns the data once the flag is up.
    /// CLUE: Seeing `ready == true` proves the store to `ready` happened. What makes the stores before it visible too?
    /// CLUE: Ch. 3 of Rust Atomics and Locks: which pair of operations makes a happens-before edge between two threads?
    pub fn read(&self) -> Option<(u64, u64)> {
        if !self.ready.load(Ordering::Relaxed) {
            return None;
        }
        Some((
            self.price.load(Ordering::Relaxed),
            self.qty.load(Ordering::Relaxed),
        ))
    }
}
