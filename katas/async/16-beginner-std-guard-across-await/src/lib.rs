use std::sync::{Arc, Mutex};

async fn audit(_n: u64) {
    tokio::task::yield_now().await;
}

/// Adds one to the counter, then records the new value.
/// CLUE: What is `guard` still holding while `audit(..).await` is suspended?
/// CLUE: Is `std::sync::MutexGuard` `Send`? Where does `tokio::spawn` need `Send`?
pub async fn bump(counter: Arc<Mutex<u64>>) {
    let mut guard = counter.lock().unwrap();
    *guard += 1;
    audit(*guard).await;
}

/// Runs `n` bumps on the runtime's worker threads and returns the total.
pub async fn bump_many(n: u64) -> u64 {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..n)
        .map(|_| tokio::spawn(bump(counter.clone())))
        .collect();
    for h in handles {
        h.await.unwrap();
    }
    *counter.lock().unwrap()
}
