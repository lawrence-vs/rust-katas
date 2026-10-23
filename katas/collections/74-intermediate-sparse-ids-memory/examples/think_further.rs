//! 74 intermediate: sparse ids, a map not a Vec
//!
//! Think further, from the kata README:
//! 1. What does the starting code do with an id of `u32::MAX`? And what was the cost of `len`?
//! 2. Ids are dense but start at a large offset, or you can map them once to `0..n`. What does a `Vec` plus an id-to-index map give you?
//! 3. The default hasher is SipHash. For integer keys, `FxHashMap` or a no-op hasher is faster. What do you give up (HashDoS) and when is that fine?
//! 4. After a burst of accounts is closed, what does `shrink_to_fit` do, and when is it worth it?
//! 5. When does order matter enough to pay for a `BTreeMap`?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::collections::HashMap;
use std::mem::size_of;

fn main() {
    // Think further 1: a Vec indexed by id must hold every slot up to the largest id.
    let largest_id: u64 = u32::MAX as u64;
    let vec_bytes = (largest_id + 1) * size_of::<Option<i64>>() as u64;
    println!("Vec<Option<i64>> up to id u32::MAX would need about {} GiB", vec_bytes / (1 << 30));

    // In the wild: with_capacity reserves room once, so inserts don't rehash.
    let mut accounts: HashMap<u32, i64> = HashMap::with_capacity(4);
    let before = accounts.capacity();
    for id in [7u32, 1_000_000, u32::MAX] {
        accounts.insert(id, 100);
    }
    println!("capacity before {before}, after 3 sparse inserts {}", accounts.capacity());

    // Think further 4: shrink_to_fit gives memory back after a burst of closes.
    accounts.remove(&7);
    accounts.remove(&1_000_000);
    accounts.shrink_to_fit();
    println!("after closes and shrink, capacity {}", accounts.capacity());

    // Think further 2: a Vec plus an id-to-index map keeps the hot data dense.
    let mut index: HashMap<u32, usize> = HashMap::new();
    let mut balances: Vec<i64> = Vec::new();
    for id in [7u32, 1_000_000] {
        index.insert(id, balances.len());
        balances.push(100);
    }
    println!("dense balances {balances:?}, index {index:?}");
}
