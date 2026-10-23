//! 34 beginner: entry, not contains_key
//!
//! Think further, from the kata README:
//! 1. The key is a `String` you only have as `&str`. `entry` takes the key by value. What does that cost on every hit, and how would you avoid it on a hot path?
//! 2. A level should be removed once its last order is cancelled. What does `Entry::Occupied` let you do that `get_mut` doesn't?
//! 3. Order books keep price levels sorted. Does `BTreeMap` have the same `entry` API? What changes about the cost of one lookup?
//! 4. You want to count and also know whether this was the first fill at that price. Which `Entry` method or `match` gives you both from one lookup?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Entry;

fn main() {
    // In the wild: one lookup creates the level on first order, and counts it.
    let mut fills: HashMap<String, u32> = HashMap::new();
    for symbol in ["BTC", "ETH", "BTC"] {
        *fills.entry(symbol.to_string()).or_insert(0) += 1;
    }
    let mut counts: Vec<_> = fills.iter().collect();
    counts.sort();
    println!("fills per symbol: {counts:?}");

    // Think further 4: one lookup gives both the count and whether this was the first fill.
    let mut levels: HashMap<u64, u32> = HashMap::new();
    for price in [100, 101, 100] {
        match levels.entry(price) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
                println!("price {price}: fill {} (not first)", e.get());
            }
            Entry::Vacant(e) => {
                e.insert(1);
                println!("price {price}: first fill");
            }
        }
    }

    // Think further 2: Occupied can remove the level once its last order is cancelled.
    if let Entry::Occupied(e) = levels.entry(101) {
        let removed = e.remove();
        println!("removed level 101 with count {removed}");
    }

    // Think further 3: BTreeMap has the same entry API, and keeps the levels sorted.
    let mut book: BTreeMap<u64, u32> = BTreeMap::new();
    for price in [102, 100, 101] {
        *book.entry(price).or_default() += 1;
    }
    println!("sorted levels: {book:?}");
}
