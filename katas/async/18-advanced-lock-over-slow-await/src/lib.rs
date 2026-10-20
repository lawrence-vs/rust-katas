use std::collections::HashMap;
use std::time::Duration;

use tokio::sync::Mutex;

/// Stands in for a slow backend call.
pub async fn fetch(key: &str) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("value-{key}")
}

#[derive(Default)]
pub struct Cache {
    map: Mutex<HashMap<String, String>>,
}

impl Cache {
    /// Returns the cached value, fetching it on a miss.
    /// CLUE: How long is `map` locked when the key is missing?
    /// CLUE: Which work actually needs the lock: the lookup, the fetch, or the insert?
    pub async fn get(&self, key: &str) -> String {
        let mut map = self.map.lock().await;
        if let Some(v) = map.get(key) {
            return v.clone();
        }
        let v = fetch(key).await;
        map.insert(key.to_string(), v.clone());
        v
    }
}
