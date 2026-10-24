//! 39 intermediate: an RAII guard, not a manual release
//!
//! Think further, from the kata README:
//! 1. `std::mem::forget(slot)` is safe Rust. What happens to the count, and why does Rust treat a leak as safe rather than undefined behaviour?
//! 2. A `Drop` impl panics while another panic is already unwinding. What does the process do?
//! 3. Suppose `in_flight` were a plain `usize` and the methods took `&mut self`. Could the guard hold `&mut Gateway` while `submit` keeps using `self`? What would you move onto the guard?
//! 4. Two threads share one `Gateway`. What replaces the `Cell`, and which tokio type already is this guard?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::cell::Cell;

struct Gateway {
    in_flight: Cell<usize>,
}

// The guard gives the slot back on drop, so an early return or a panic can't leak it.
struct Slot<'a> {
    gateway: &'a Gateway,
}

impl Gateway {
    fn begin(&self) -> Slot<'_> {
        self.in_flight.set(self.in_flight.get() + 1);
        Slot { gateway: self }
    }
}

impl Drop for Slot<'_> {
    fn drop(&mut self) {
        self.gateway.in_flight.set(self.gateway.in_flight.get() - 1);
    }
}

fn submit(gateway: &Gateway, fail: bool) -> Result<(), &'static str> {
    let _slot = gateway.begin();
    if fail {
        return Err("rejected");
    }
    Ok(())
}

fn main() {
    let gateway = Gateway {
        in_flight: Cell::new(0),
    };

    println!("ok: {:?}", submit(&gateway, false));
    println!("rejected: {:?}", submit(&gateway, true));
    println!("in flight after both: {}", gateway.in_flight.get());

    // Think further 1: mem::forget is safe Rust, and it skips Drop, so the count leaks.
    let slot = gateway.begin();
    std::mem::forget(slot);
    println!("in flight after forget: {}", gateway.in_flight.get());

    // In the wild: MutexGuard and SemaphorePermit do the same job, unlocking or releasing on drop.
}
