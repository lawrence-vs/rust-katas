//! 25 advanced: a lagging client
//!
//! Think further, from the kata README:
//! 1. The feed is order-book deltas, not chat. The client skipped 3 deltas. Is its book still right, and what would the client or server have to do next?
//! 2. Raise the capacity from 2 to 10,000 and `Lagged` mostly goes away. What does that cost, and who pays for it: the slow client or every client?
//! 3. `outbox` is bounded at 16 and the client stops reading. Where does `fan_out` wait, and what happens on the `broadcast` side while it waits? You now have two buffers. Which policy does each one have?
//! 4. Should the client be told it missed `n` messages? What would that look like on the wire?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Capacity 2: a receiver that falls more than 2 messages behind misses the oldest ones.
    let (tx, mut slow) = broadcast::channel::<u64>(2);
    let mut fast = tx.subscribe();

    for delta in 1..=5 {
        tx.send(delta).ok();
    }

    // Think further 1: the slow client sees Lagged(n) and keeps going from the newest message.
    loop {
        match slow.recv().await {
            Ok(delta) => println!("slow got delta {delta}"),
            Err(RecvError::Lagged(missed)) => println!("slow lagged, missed {missed}"),
            Err(RecvError::Closed) => break,
        }
        if slow.is_empty() {
            break;
        }
    }

    // A client that reads late gets Lagged first, then only the newest messages still in the buffer.
    let mut seen = Vec::new();
    loop {
        match fast.try_recv() {
            Ok(delta) => seen.push(delta),
            Err(broadcast::error::TryRecvError::Lagged(missed)) => println!("fast lagged, missed {missed}"),
            Err(_) => break,
        }
    }
    println!("fast client kept: {seen:?}");

    // Think further 4: telling the client it missed n messages is a message of its own on the wire.
    drop(tx);
    println!("closed: {}", matches!(slow.recv().await, Err(RecvError::Closed)));
}
