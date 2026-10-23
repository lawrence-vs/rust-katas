//! 17 intermediate: spawn, then send results back
//!
//! Think further, from the kata README:
//! 1. One task panics. What does its `h.await` return, and what happens to every other length when the loop calls `unwrap()` on it?
//! 2. `words` has a million entries. How many tasks run at once, and what does each one cost? Where would you put a limit? (kata 21)
//! 3. The caller gives up halfway (a `timeout` around `word_lengths`, or the request is dropped). Do the spawned tasks stop? Would the answer change if they lived in a `JoinSet`?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Think further 1: one task panics; its JoinHandle returns an error, the others still finish.
    let words = ["ant", "bee", "crab"];
    let mut handles = Vec::new();
    for (i, word) in words.iter().enumerate() {
        let word = word.to_string();
        handles.push(tokio::spawn(async move {
            if i == 1 {
                panic!("bad word");
            }
            word.len()
        }));
    }
    for h in handles {
        match h.await {
            Ok(len) => println!("length {len}"),
            Err(err) => println!("task failed, panic? {}", err.is_panic()),
        }
    }

    // In the wild: an mpsc channel carries results from many tasks back to one owner.
    let (tx, mut rx) = mpsc::channel::<usize>(8);
    for word in ["hello", "tokio"] {
        let tx = tx.clone();
        tokio::spawn(async move {
            let _ = tx.send(word.len()).await;
        });
    }
    drop(tx);
    while let Some(len) = rx.recv().await {
        println!("received {len}");
    }
}
