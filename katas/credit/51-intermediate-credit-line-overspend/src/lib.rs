//! A credit line that open orders reserve against.
//!
//! The atomics come from `loom`, so `tests/kata.rs` can try every
//! interleaving of the threads.

use loom::sync::atomic::{AtomicU64, Ordering};

/// Quote-currency cents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cents(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditError {
    Insufficient { available: Cents, requested: Cents },
    Overfill { reserved: Cents, filled: Cents },
}

/// Credit held for one open order. Only `CreditLine::reserve` makes one; `fill` or `cancel` uses it up.
#[derive(Debug, PartialEq, Eq)]
pub struct Reservation {
    amount: Cents,
}

impl Reservation {
    pub fn amount(&self) -> Cents {
        self.amount
    }
}

/// A limit and the credit committed against it (reserved plus filled).
pub struct CreditLine {
    limit: Cents,
    committed: AtomicU64,
}

impl CreditLine {
    pub fn new(limit: Cents) -> Self {
        Self {
            limit,
            committed: AtomicU64::new(0),
        }
    }

    pub fn available(&self) -> Cents {
        Cents(self.limit.0 - self.committed.load(Ordering::Relaxed))
    }

    /// Holds `amount` of credit for an order, or says how much is left.
    /// CLUE: Between the load and the store, what can another thread do? How many atomic operations is this?
    /// CLUE: Rust Atomics and Locks ch. 2: which one operation reads, decides and writes, and fails if the value changed?
    pub fn reserve(&self, amount: Cents) -> Result<Reservation, CreditError> {
        let available = self.available();
        if amount > available {
            return Err(CreditError::Insufficient {
                available,
                requested: amount,
            });
        }
        let used = self.committed.load(Ordering::Relaxed);
        self.committed.store(used + amount.0, Ordering::Relaxed);
        Ok(Reservation { amount })
    }

    /// Settles an order for `filled`; the rest of the reservation goes back to the line.
    /// CLUE: Who gives the unfilled credit back? Would a load then a store here have the same bug as reserve?
    pub fn fill(&self, reservation: Reservation, filled: Cents) -> Result<(), CreditError> {
        let _ = (reservation, filled);
        Ok(())
    }

    /// Drops an order; its whole reservation goes back to the line.
    pub fn cancel(&self, reservation: Reservation) {
        let _ = reservation;
    }
}
