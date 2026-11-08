use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use kata_async_219::{Fetch, TokenCache};

fn slow_fetch(fetches: Arc<AtomicUsize>) -> impl Fn() -> Fetch {
    move || {
        let fetches = fetches.clone();
        Box::pin(async move {
            fetches.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(50)).await;
            "tok".to_string()
        })
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cold_burst_fetches_once() {
    let cache = Arc::new(TokenCache::new());
    let fetches = Arc::new(AtomicUsize::new(0));
    let tasks: Vec<_> = (0..8)
        .map(|_| {
            let (cache, fetches) = (cache.clone(), fetches.clone());
            tokio::spawn(async move { cache.get(slow_fetch(fetches)).await })
        })
        .collect();
    for task in tasks {
        assert_eq!(task.await.unwrap(), "tok");
    }
    let n = fetches.load(Ordering::SeqCst);
    assert_eq!(n, 1, "upstream fetched {n} times");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn warm_cache_does_not_fetch_again() {
    let cache = TokenCache::new();
    let fetches = Arc::new(AtomicUsize::new(0));
    cache.get(slow_fetch(fetches.clone())).await;
    cache.get(slow_fetch(fetches.clone())).await;
    assert_eq!(fetches.load(Ordering::SeqCst), 1);
}
