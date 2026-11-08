use std::time::Duration;

use tokio::sync::mpsc::Receiver;
use tokio::time::interval;

/// Why `run` returned.
#[derive(Debug, PartialEq, Eq)]
pub enum End {
    Idle,
    Closed,
}

/// Reads frames until the sender closes or no frame arrives for `idle`, pinging every `heartbeat`.
/// CLUE: a heartbeat tick is not progress, yet it re-enters the loop. What does the loop do to the idle deadline each pass?
/// CLUE: `Sleep::reset(deadline)` moves a timer you already hold; call it only when a real frame arrives.
pub async fn run(mut frames: Receiver<String>, idle: Duration, heartbeat: Duration) -> End {
    let mut ping = interval(heartbeat);
    loop {
        tokio::select! {
            frame = frames.recv() => if frame.is_none() { return End::Closed },
            _ = ping.tick() => {}
            _ = tokio::time::sleep(idle) => return End::Idle,
        }
    }
}
