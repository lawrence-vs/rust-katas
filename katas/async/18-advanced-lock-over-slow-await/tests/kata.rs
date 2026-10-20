use std::sync::Arc;
use std::time::Duration;

use kata_async_18::Cache;
use tokio::task::JoinSet;
use tokio::time::Instant;

#[tokio::test(start_paused = true)]
async fn different_keys_fetch_concurrently() {
    let cache = Arc::new(Cache::default());
    let start = Instant::now();
    let mut set = JoinSet::new();
    for i in 0..10 {
        let cache = cache.clone();
        set.spawn(async move { cache.get(&format!("k{i}")).await });
    }
    while let Some(v) = set.join_next().await {
        assert!(v.unwrap().starts_with("value-k"));
    }
    assert!(
        start.elapsed() < Duration::from_millis(200),
        "gets ran one at a time: {:?}",
        start.elapsed()
    );
}

#[tokio::test(start_paused = true)]
async fn second_get_is_a_hit() {
    let cache = Cache::default();
    cache.get("a").await;
    let start = Instant::now();
    assert_eq!(cache.get("a").await, "value-a");
    assert_eq!(start.elapsed(), Duration::ZERO);
}
