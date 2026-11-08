use std::future::Future;
use std::pin::Pin;

use tokio::sync::Mutex;

pub type Fetch = Pin<Box<dyn Future<Output = String> + Send>>;

#[derive(Default)]
pub struct TokenCache {
    inner: Mutex<Option<String>>,
}

impl TokenCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// CLUE: where is the guard dropped relative to the fetch `.await`, and who can slip in between?
    pub async fn get(&self, fetch: impl Fn() -> Fetch) -> String {
        {
            let cached = self.inner.lock().await;
            if let Some(token) = cached.as_ref() {
                return token.clone();
            }
        }
        let token = fetch().await;
        *self.inner.lock().await = Some(token.clone());
        token
    }
}
