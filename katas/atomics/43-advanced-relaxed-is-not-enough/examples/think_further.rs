//! 43 advanced: Relaxed is not enough
//!
//! Think further, from the kata README:
//! 1. `publish` is called twice, (101, 5) then (102, 7). Can a reader get (102, 5)? What design (a seqlock, or swapping an `Arc` pointer) prevents a torn read?
//! 2. Rewrite the reader with a `Relaxed` load followed by `fence(Acquire)`. When is a fence better than an `Acquire` load?
//! 3. How would you let this library build with std atomics normally and loom atomics under `RUSTFLAGS="--cfg loom"`? What does `check-cfg` need?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering, fence};
use std::thread;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    // Release on the store and Acquire on the load order the data write before the flag.
    let writer = thread::spawn(|| {
        DATA.store(101, Ordering::Relaxed);
        READY.store(true, Ordering::Release);
    });
    let reader = thread::spawn(|| {
        while !READY.load(Ordering::Acquire) {
            thread::yield_now();
        }
        println!("data after acquire: {}", DATA.load(Ordering::Relaxed));
    });
    writer.join().ok();
    reader.join().ok();

    // Think further 2: a Relaxed load then fence(Acquire) gives the same ordering for this read.
    DATA.store(202, Ordering::Relaxed);
    READY.store(true, Ordering::Release);
    if READY.load(Ordering::Relaxed) {
        fence(Ordering::Acquire);
        println!("data after fence: {}", DATA.load(Ordering::Relaxed));
    }
}
