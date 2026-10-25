//! 161 intermediate: a wallet invariant that table tests miss
//!
//! Think further, from the kata README:
//! 1. A seed is fixed here (`RngSeed::Fixed(161)`) and `failure_persistence` is `None`. What could a fixed seed keep hiding forever, and when would a committed `proptest-regressions/` file serve you better?
//! 2. How would you state "money is conserved" for a `transfer` between two wallets as a property?
//! 3. If `reserve` returned a hold id and `release(id)` freed exactly that hold, what new invariant joins `total == free + reserved`?
//! 4. In a small clean-code review, would you add proptest, or table tests and a "property tests later" note? What does each give the reviewer?

// Scratch file for the Think further items, as in kata 53. Run with: cargo run --example think_further

use std::collections::HashMap;

// Think further 3: a hold has an id, so a second release is a lookup miss.
#[derive(Debug, Default)]
struct Holds {
    free: u64,
    holds: HashMap<u32, u64>,
    next: u32,
}

impl Holds {
    fn reserve(&mut self, amount: u64) -> Option<u32> {
        self.free = self.free.checked_sub(amount)?;
        let id = self.next;
        self.next += 1;
        self.holds.insert(id, amount);
        Some(id)
    }

    fn release(&mut self, id: u32) -> Option<()> {
        let amount = self.holds.remove(&id)?;
        self.free = self.free.checked_add(amount)?;
        Some(())
    }
}

fn main() {
    let mut w = Holds {
        free: 100,
        ..Holds::default()
    };
    if let Some(id) = w.reserve(30) {
        println!("first release: {:?}", w.release(id));
        println!("second release: {:?}", w.release(id));
    }
    println!("free {} holds {}", w.free, w.holds.len());
}
