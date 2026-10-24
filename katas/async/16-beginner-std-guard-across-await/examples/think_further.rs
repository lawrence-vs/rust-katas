//! 16 beginner: a std MutexGuard across `.await`
//!
//! Think further, from the kata README:
//! 1. `audit` only needs the new count. What changes if it needed something that isn't `Copy`, like a `String` built from data inside the lock?
//! 2. `lock().unwrap()` panics if the mutex is poisoned. What poisons it, which task would see the panic, and what should `bump_many` do then?
//! 3. Ten thousand tasks bump one counter. Is `Mutex<u64>` still the right tool? What would `AtomicU64` change, and what would it not give you? (kata 43 goes into orderings)

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut tasks = Vec::new();
    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        tasks.push(tokio::spawn(async move {
            // The guard is dropped at the end of this block, before any .await.
            {
                let mut n = counter.lock().unwrap_or_else(|e| e.into_inner());
                *n += 1;
            }
            tokio::task::yield_now().await;
            *counter.lock().unwrap_or_else(|e| e.into_inner())
        }));
    }
    for task in tasks {
        match task.await {
            Ok(seen) => println!("task saw {seen}"),
            Err(err) => println!("task failed: {err}"),
        }
    }

    // Think further 2: a poisoned mutex means a holder panicked; into_inner recovers the value.
    let poisoned = Arc::new(Mutex::new(5u64));
    let p = Arc::clone(&poisoned);
    let _ = std::thread::spawn(move || {
        let _guard = p.lock();
        panic!("holder panicked");
    })
    .join();
    println!("poisoned: {}", poisoned.is_poisoned());
    println!(
        "value still readable: {}",
        *poisoned.lock().unwrap_or_else(|e| e.into_inner())
    );
}
