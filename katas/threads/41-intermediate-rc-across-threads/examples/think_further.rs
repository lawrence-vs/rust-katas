//! 41 intermediate: Rc across threads
//!
//! Think further, from the kata README:
//! 1. The audit should also append a correction to the fills. What does `Arc<Vec<u64>>` stop you doing, what would you add, and why not `Rc<RefCell<_>>`?
//! 2. `Rc` is `!Sync` as well as `!Send`. What could go wrong if two threads each held a `&Rc<T>`?
//! 3. A worker panics. What does `thread::scope` do when it returns, and what happens to a panic in a `thread::spawn` thread whose `JoinHandle` is dropped?
//! 4. When does spawning a thread per chunk cost more than one loop? What do rayon's `par_iter` and its thread pool change?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::sync::Arc;
use std::thread;

fn main() {
    // Arc (not Rc) because the count is updated atomically; Rc would not compile across threads.
    let fills: Arc<Vec<u64>> = Arc::new(vec![10, 20, 30, 40]);

    // Think further 3: scoped threads join before the scope returns, so they may borrow the data.
    let sums: Vec<u64> = thread::scope(|s| {
        let halves: Vec<_> = fills
            .chunks(2)
            .map(|chunk| s.spawn(move || chunk.iter().sum::<u64>()))
            .collect();
        halves.into_iter().filter_map(|h| h.join().ok()).collect()
    });
    println!("chunk sums: {sums:?}");

    // Think further 1: a shared Vec is read-only. A correction needs its own copy or a lock.
    let mut corrected = (*fills).clone();
    corrected.push(5);
    println!(
        "original {} fills, corrected {}",
        fills.len(),
        corrected.len()
    );

    println!("strong count: {}", Arc::strong_count(&fills));
}
