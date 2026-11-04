use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::{CancellationToken, DropGuard};

pub struct Worker {
    pub handle: JoinHandle<()>,
    /// CLUE: when does a `DropGuard` cancel its token, and when is this one dropped?
    pub guard: Option<DropGuard>,
}

/// CLUE: what does `let _ = value;` do to the value, compared with `let _name = value;`?
pub fn start(ticks: Arc<AtomicUsize>) -> Worker {
    let token = CancellationToken::new();
    let _ = token.clone().drop_guard();
    let handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                biased;
                _ = token.cancelled() => break,
                _ = tokio::time::sleep(Duration::from_millis(10)) => {
                    ticks.fetch_add(1, Ordering::SeqCst);
                }
            }
        }
    });
    Worker {
        handle,
        guard: None,
    }
}
