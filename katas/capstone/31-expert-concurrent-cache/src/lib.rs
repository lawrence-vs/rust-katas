use std::collections::HashMap;
use std::future::Future;

use tokio::sync::Mutex;

/// An async read-through cache shared by many tasks.
pub struct Cache<V> {
    capacity: usize,
    map: Mutex<HashMap<String, V>>,
}

impl<V: Clone> Cache<V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: Mutex::new(HashMap::new()),
        }
    }

    /// Number of cached keys.
    /// CLUE: Stage 1. `capacity` is stored and never read. Which entry should leave when a new one arrives?
    pub async fn cached_keys(&self) -> usize {
        self.map.lock().await.len()
    }

    /// Returns the cached value, or runs `load` once and caches its result.
    /// CLUE: Stage 2. Which tasks wait while one task's `load` is sleeping?
    /// CLUE: Stage 3 passes today only because of the stage 2 bug. When you release the lock during `load`, what stops ten tasks loading the same key?
    /// CLUE: Look at `tokio::sync::OnceCell::get_or_init`, and at what an `Arc` of one lets you hold without the map's lock.
    pub async fn get_or_load<F, Fut>(&self, key: &str, load: F) -> V
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = V>,
    {
        let mut map = self.map.lock().await;
        if let Some(v) = map.get(key) {
            return v.clone();
        }
        let v = load().await;
        map.insert(key.to_string(), v.clone());
        v
    }
}
