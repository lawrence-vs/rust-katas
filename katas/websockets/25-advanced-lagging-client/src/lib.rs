use tokio::sync::{broadcast, mpsc};

/// The per-client write task of a chat server: copies room messages to one client's outbox.
/// Returns how many messages this client missed because it fell behind.
/// CLUE: `recv` has two errors: `Closed` and `Lagged(n)`. Which one ends this loop today?
/// CLUE: A slow client fell behind. Should that disconnect it, or skip it ahead?
pub async fn fan_out(mut room: broadcast::Receiver<String>, outbox: mpsc::Sender<String>) -> u64 {
    let missed = 0;
    while let Ok(msg) = room.recv().await {
        if outbox.send(msg).await.is_err() {
            break;
        }
    }
    missed
}
