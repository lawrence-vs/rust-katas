use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use kata_capstone_31::Cache;
use tokio::task::JoinSet;
use tokio::time::{Instant, sleep};

async fn slow(v: u32) -> u32 {
    sleep(Duration::from_millis(100)).await;
    v
}

#[tokio::test(start_paused = true)]
async fn stage1_capacity_is_bounded() {
    let cache = Cache::new(2);
    for (i, k) in ["a", "b", "c"].into_iter().enumerate() {
        cache.get_or_load(k, || slow(i as u32)).await;
    }
    assert_eq!(cache.cached_keys().await, 2);
    let reloaded = cache.get_or_load("a", || slow(99)).await;
    assert_eq!(reloaded, 99, "the oldest key should have been evicted");
}

#[tokio::test(start_paused = true)]
async fn stage2_distinct_keys_load_concurrently() {
    let cache = Arc::new(Cache::new(100));
    let start = Instant::now();
    let mut set = JoinSet::new();
    for i in 0..10 {
        let cache = cache.clone();
        set.spawn(async move { cache.get_or_load(&format!("k{i}"), || slow(i)).await });
    }
    while set.join_next().await.is_some() {}
    assert!(
        start.elapsed() < Duration::from_millis(200),
        "loads ran one at a time: {:?}",
        start.elapsed()
    );
}

#[tokio::test(start_paused = true)]
async fn stage3_same_key_loads_once() {
    let cache = Arc::new(Cache::new(100));
    let calls = Arc::new(AtomicUsize::new(0));
    let mut set = JoinSet::new();
    for _ in 0..10 {
        let (cache, calls) = (cache.clone(), calls.clone());
        set.spawn(async move {
            cache
                .get_or_load("hot", || {
                    calls.fetch_add(1, Ordering::SeqCst);
                    slow(7)
                })
                .await
        });
    }
    while let Some(v) = set.join_next().await {
        assert_eq!(v.unwrap(), 7);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
