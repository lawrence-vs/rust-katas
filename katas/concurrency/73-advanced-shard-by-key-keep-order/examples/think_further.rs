//! 73 advanced: shard by key, keep per-key order
//!
//! Think further, from the kata README:
//! 1. Hot keys and skew: one account is 90% of the stream. What does sharding buy you now?
//! 2. Resharding and consistent hashing: how do you change N without breaking per-key order?
//! 3. The same design with tokio: one `spawn` per shard and `mpsc::channel(capacity)`. What changes about `send` and about a dead worker?
//! 4. An event touches two keys, such as a transfer. Which worker owns it?
//! 5. Blocking on `send` versus `try_send` and dropping (kata 21): which is right for an untrusted producer?
//! 6. Batching per shard: what does it cost in latency and gain in throughput?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::mpsc;
use std::thread;

const SHARDS: usize = 3;

fn shard_of(key: &str) -> usize {
    let mut h = DefaultHasher::new();
    key.hash(&mut h);
    (h.finish() as usize) % SHARDS
}

fn main() {
    // The same key always lands on the same shard, so one worker sees that key's events in order.
    let events = [
        ("alice", 1),
        ("bob", 1),
        ("alice", 2),
        ("carol", 1),
        ("alice", 3),
        ("bob", 2),
    ];

    let mut senders = Vec::new();
    let mut workers = Vec::new();
    for _ in 0..SHARDS {
        let (tx, rx) = mpsc::channel::<(&str, u32)>();
        senders.push(tx);
        workers.push(thread::spawn(move || {
            // Each worker returns what it saw, so the output is in order, not interleaved.
            rx.into_iter().collect::<Vec<_>>()
        }));
    }

    for (key, seq) in events {
        senders[shard_of(key)].send((key, seq)).ok();
    }
    drop(senders);

    for (i, worker) in workers.into_iter().enumerate() {
        match worker.join() {
            Ok(seen) => println!("shard {i}: {seen:?}"),
            Err(_) => println!("shard {i} panicked"),
        }
    }

    // Think further 1: a hot key always lands on one shard, so sharding can't spread its load.
    println!("every alice event goes to shard {}", shard_of("alice"));
}
