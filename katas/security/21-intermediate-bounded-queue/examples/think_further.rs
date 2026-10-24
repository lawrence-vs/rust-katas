//! 21 intermediate: a bounded queue for untrusted input
//!
//! Think further, from the kata README:
//! 1. `capacity` counts messages, not bytes. The test sends 1 KiB messages; an attacker sends eight 16 MiB ones. What bounds memory now, and what else would you have to limit?
//! 2. Every client shares one queue. One noisy client keeps it full. What do the quiet clients see, and how could you make it fair?
//! 3. What does `ingest(0)` do? Should a caller be able to ask for that?
//! 4. The worker panics and drops `rx`. What does `accept` return now, and is `Rejected` the right thing to tell the client in that case?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use tokio::sync::mpsc;

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Accepted,
    Rejected,
}

// Think further 1: a bounded queue caps messages, not bytes, so a message size limit must sit beside it.
const MAX_MESSAGE_BYTES: usize = 4 * 1024;

// Think further 4: a dropped receiver makes send fail, and the client gets Rejected, not a hang.
fn accept(tx: &mpsc::Sender<Vec<u8>>, msg: Vec<u8>) -> Outcome {
    if msg.len() > MAX_MESSAGE_BYTES {
        return Outcome::Rejected;
    }
    match tx.try_send(msg) {
        Ok(()) => Outcome::Accepted,
        Err(_) => Outcome::Rejected,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Think further 3: a capacity of 0 would panic in channel(0); the caller shouldn't be able to ask for it.
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(2);

    println!("first: {:?}", accept(&tx, vec![1; 10]));
    println!("second: {:?}", accept(&tx, vec![2; 10]));
    println!("third, queue full: {:?}", accept(&tx, vec![3; 10]));
    println!(
        "oversized: {:?}",
        accept(&tx, vec![4; MAX_MESSAGE_BYTES + 1])
    );

    // Draining the queue frees room; the next message is accepted.
    rx.recv().await;
    println!("after one recv: {:?}", accept(&tx, vec![5; 10]));

    // Think further 4: once the consumer is gone, every send is rejected instead of waiting forever.
    drop(rx);
    println!("receiver gone: {:?}", accept(&tx, vec![6; 10]));
}
