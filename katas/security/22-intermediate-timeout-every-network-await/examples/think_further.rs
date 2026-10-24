//! 22 intermediate: a timeout on every network await
//!
//! Think further, from the kata README:
//! 1. The peer sends one byte every 4 seconds and never a newline. Does a deadline on the whole greeting behave like a deadline on each read? Which one stops this peer?
//! 2. The peer sends 1 GiB with no newline, fast. How big does `line` get before any deadline fires? (Kata 23 meets the same problem.)
//! 3. The deadline fires halfway through a line. What happens to the bytes already in the `BufReader`? Could the caller try again on the same connection?
//! 4. Who should choose the timeout: a constant in this function, config, or the caller? What breaks when a slow mobile client and a co-located bot share one value?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::timeout;

// Think further 1: a timeout on each read stops a peer that drips bytes forever only if the read is bounded too.
// Think further 2: a line with no newline grows without a bound; cap it, as kata 23 does for frames.
async fn read_line_with_deadline<R: AsyncBufReadExt + Unpin>(
    reader: &mut R,
    limit: Duration,
) -> Result<String, String> {
    let mut line = String::new();
    timeout(limit, reader.read_line(&mut line))
        .await
        .map_err(|_| "timed out".to_string())?
        .map_err(|e| e.to_string())?;
    Ok(line)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // A peer that sends a line promptly.
    let (mut client, server) = tokio::io::duplex(64);
    let mut reader = BufReader::new(server);
    client.write_all(b"hello\n").await.ok();
    println!(
        "prompt: {:?}",
        read_line_with_deadline(&mut reader, Duration::from_millis(100)).await
    );

    // A peer that never finishes its line.
    client.write_all(b"partial").await.ok();
    println!(
        "silent: {:?}",
        read_line_with_deadline(&mut reader, Duration::from_millis(50)).await
    );

    // Think further 3: after a timeout, the bytes already read stay in the BufReader; the next read continues.
    client.write_all(b" rest\n").await.ok();
    println!(
        "continued: {:?}",
        read_line_with_deadline(&mut reader, Duration::from_millis(100)).await
    );
}
