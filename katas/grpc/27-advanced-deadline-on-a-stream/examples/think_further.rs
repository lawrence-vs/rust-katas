//! 27 advanced: a deadline on a stream
//!
//! Think further, from the kata README:
//! 1. The stream is slow, not stalled: one tick every 900 ms, and you want 5. What does your version return? Is that what a trading screen wants?
//! 2. The deadline fires after 3 of 5 ticks. You return `DeadlineExceeded` and throw away 3 good prices. When would returning the partial result be better, and how would the signature have to change to say so?
//! 3. The client gives up and drops the stream. What happens to the server's stream: does it keep producing, and how would it find out?
//! 4. Service A calls B with 2 s left, and B calls C. How long should B give C?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further
// std only: a channel stands in for the gRPC stream, so the deadline logic is visible.

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
enum StreamError {
    DeadlineExceeded,
    Closed,
}

// Think further 1: each read waits only for what is left of the overall deadline.
fn take_until_deadline(rx: &mpsc::Receiver<u32>, want: usize, budget: Duration) -> Result<Vec<u32>, StreamError> {
    let deadline = Instant::now() + budget;
    let mut got = Vec::new();
    while got.len() < want {
        let left = deadline.saturating_duration_since(Instant::now());
        match rx.recv_timeout(left) {
            Ok(tick) => got.push(tick),
            Err(mpsc::RecvTimeoutError::Timeout) => return Err(StreamError::DeadlineExceeded),
            Err(mpsc::RecvTimeoutError::Disconnected) => return Err(StreamError::Closed),
        }
    }
    Ok(got)
}

fn main() {
    // A slow producer: one tick every 90 ms, asked for 5 within 300 ms.
    let (tx, rx) = mpsc::channel();
    let producer = thread::spawn(move || {
        for tick in 1..=5 {
            thread::sleep(Duration::from_millis(90));
            if tx.send(tick).is_err() {
                // Think further 3: the send fails once the receiver is dropped, which is how the producer learns.
                println!("producer: receiver gone, stopping");
                return;
            }
        }
    });

    println!("{:?}", take_until_deadline(&rx, 5, Duration::from_millis(300)));
    drop(rx);
    producer.join().ok();
}
