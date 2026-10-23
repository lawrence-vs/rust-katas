//! 45 expert: capstone: a mini order book
//!
//! Think further, from the kata README:
//! 1. Self-trade: the same account rests a sell and then sends a crossing buy. Where in `submit` would you check it, and what should happen: cancel the old order, the new one, or both?
//! 2. An immediate-or-cancel order must never rest. A market order has no limit price. What do the two need from `submit` that a limit order doesn't?
//! 3. A caller submits an `OrderId` that is already resting. What does your index do with it today, and what should `submit` return instead?
//! 4. `best_bid` sums the whole queue on every call, and that sum could overflow `u64`. How would you keep a running total per level, with `checked_add` (kata 42), so it costs O(1)?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Default)]
struct Level {
    orders: VecDeque<(u64, u64)>, // (order id, qty), oldest first
    total: u64,                   // running total, so best depth is O(1)
}

#[derive(Debug, Default)]
struct Book {
    bids: BTreeMap<u64, Level>,
}

impl Book {
    fn rest_bid(&mut self, price: u64, id: u64, qty: u64) {
        let level = self.bids.entry(price).or_default();
        level.orders.push_back((id, qty));
        level.total += qty;
    }

    // In the wild: last_entry gives the best bid, and remove drops the level once it is empty.
    fn best_bid(&self) -> Option<(u64, u64)> {
        self.bids.last_key_value().map(|(p, l)| (*p, l.total))
    }

    fn pop_best_bid(&mut self) -> Option<(u64, u64, u64)> {
        let mut entry = self.bids.last_entry()?;
        let price = *entry.key();
        let level = entry.get_mut();
        let (id, qty) = level.orders.pop_front()?;
        level.total -= qty;
        if level.orders.is_empty() {
            entry.remove();
        }
        Some((price, id, qty))
    }
}

fn main() {
    let mut book = Book::default();
    book.rest_bid(100, 1, 5);
    book.rest_bid(101, 2, 3);
    book.rest_bid(101, 3, 4);

    // Think further 4: the level total is kept current, so best depth needs no sum over the queue.
    println!("best bid: {:?}", book.best_bid());

    // Time priority: order 2 rested before 3 at the same price, so it fills first.
    println!("filled: {:?}", book.pop_best_bid());
    println!("filled: {:?}", book.pop_best_bid());
    println!("best bid now: {:?}", book.best_bid());
    println!("empty book: {:?}", Book::default().best_bid());
}
