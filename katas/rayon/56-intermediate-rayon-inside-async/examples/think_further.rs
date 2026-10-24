//! 56 intermediate: rayon inside async
//!
//! Think further, from the kata README:
//! 1. 1,000 requests arrive at once. Each one parks a blocking-pool thread on rayon. What bounds them (a `Semaphore`, kata 21)?
//! 2. The caller times out (kata 22). Does the rayon work stop? What would a cancel flag checked between levels cost?
//! 3. With no `panic_handler` set, a panic inside `rayon::spawn` aborts the process. Read the docs. How does that change your choice of fix?
//! 4. Why does this kata measure real time instead of using `start_paused`?
//! 5. Each level takes 20 µs, not 300 ms. Is hopping threads still worth it?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use rayon::prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Think further 1 and 5: the CPU work hops to a blocking thread, so the async runtime keeps serving others.
    let levels: Vec<u64> = (1..=10_000).collect();
    let total =
        tokio::task::spawn_blocking(move || levels.par_iter().map(|q| q * 2).sum::<u64>()).await;
    match total {
        Ok(sum) => println!("depth total: {sum}"),
        Err(err) => println!("worker failed: {err}"),
    }

    // Think further 2: a blocking task keeps running after its caller gives up; only its result is dropped.
    let abandoned = tokio::task::spawn_blocking(|| (0..1_000u64).sum::<u64>());
    abandoned.abort();
    println!("abort returned; the blocking work finishes on its own");
}
