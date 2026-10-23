//! 20 advanced: a sync driver on the async runtime
//!
//! Think further, from the kata README:
//! 1. 10,000 lookups arrive at once. Tokio's blocking pool has a thread limit (`max_blocking_threads`). What happens to lookup 10,000, and what bounds memory while it waits?
//! 2. The caller's future is dropped (the client disconnects, or a timeout fires) while the driver call is running. Does the database work stop? Would your answer worry you more for a write than for a lookup?
//! 3. `tokio::task::block_in_place` also exists. Would it work on the test's runtime? Read its docs before you answer.
//! 4. The driver call takes 2 µs, not 100 ms. Is moving it to another thread still worth it? Where would you draw the line, and how would you measure it?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further
// A sleep stands in for the synchronous driver call.

use std::time::Duration;

fn sync_lookup(id: u64) -> String {
    std::thread::sleep(Duration::from_millis(20));
    format!("user-{id}")
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Think further 2: a dropped caller doesn't stop the blocking call; the result is simply unused.
    let dropped = tokio::task::spawn_blocking(|| sync_lookup(0));
    dropped.abort();

    // The lookup runs off the async worker, so other tasks still make progress meanwhile.
    let ticker = tokio::spawn(async {
        for _ in 0..3 {
            tokio::task::yield_now().await;
            println!("tick");
        }
    });
    let found = tokio::task::spawn_blocking(|| sync_lookup(42)).await;
    println!("lookup: {:?}", found);
    ticker.await.ok();

    // Think further 4: a call of a few µs can cost less than the thread hop, so measure before you move it.
    let started = std::time::Instant::now();
    let _ = sync_lookup(7);
    println!("direct call took {} ms", started.elapsed().as_millis());
}
