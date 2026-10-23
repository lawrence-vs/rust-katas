//! 18 advanced: a lock held over a slow await
//!
//! Think further, from the kata README:
//! 1. In the starting code, `fetch` panics or its task is aborted while it holds the lock. What happens to the `tokio::sync::Mutex` and to the tasks waiting on it? Is a std `Mutex` different here?
//! 2. Two tasks fetch the same key and the backend changes in between, so they get different values. Which value stays in the map? Does that matter for a tick size? For a price?
//! 3. Reads far outnumber misses. Would a `RwLock` help when the lock is held for microseconds? What about a sharded map like `DashMap`, and what happens if you keep one of its references across an `.await`?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Think further 1: an aborted task drops its tokio guard, so the lock is free again.
    let lock = Arc::new(Mutex::new(0u32));
    let held = Arc::clone(&lock);
    let task = tokio::spawn(async move {
        let _guard = held.lock().await;
        tokio::time::sleep(Duration::from_secs(60)).await;
    });
    tokio::time::sleep(Duration::from_millis(10)).await;
    task.abort();
    let _ = task.await;
    println!("lock free after abort: {}", lock.try_lock().is_ok());

    // The fix shape: lock for the lookup, release, do the slow work, lock again for the insert.
    let cache: Arc<Mutex<Vec<(u32, u32)>>> = Arc::new(Mutex::new(Vec::new()));
    let fetched = {
        let guard = cache.lock().await;
        guard.iter().find(|(k, _)| *k == 7).map(|(_, v)| *v)
    };
    if fetched.is_none() {
        tokio::time::sleep(Duration::from_millis(5)).await; // stands in for a slow fetch
        cache.lock().await.push((7, 42));
    }
    println!("cached: {:?}", cache.lock().await.first());
}
