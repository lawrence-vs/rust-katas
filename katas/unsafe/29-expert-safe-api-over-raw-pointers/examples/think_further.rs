//! 29 expert: a safe API over raw pointers
//!
//! Think further, from the kata README:
//! 1. `Buffer::new(0)`: what does the allocation look like, and what may `get(0)` do?
//! 2. Someone adds `#[derive(Clone)]` to your fixed `Buffer`. With fix A, what happens? With fix B, what happens when both copies drop?
//! 3. Is your `Buffer` `Send` and `Sync`? Why do raw pointers change the answer, and what would `unsafe impl Send` have to promise?
//! 4. A hot loop wants to skip the bounds check. When is a `get_unchecked`-style `unsafe fn` worth it, and what would you measure first?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

// A safe wrapper over a Vec: the same shape as Vec's own internals, with the bounds check in one place.
pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    // Think further 1: an empty buffer is a valid value; get on it returns None, never reads memory.
    pub fn new(len: usize) -> Self {
        Buffer { data: vec![0; len] }
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        self.data.get(index).copied()
    }

    pub fn set(&mut self, index: usize, value: u8) -> bool {
        match self.data.get_mut(index) {
            Some(slot) => {
                *slot = value;
                true
            }
            None => false,
        }
    }
}

fn main() {
    let mut buf = Buffer::new(4);
    println!("set in range: {}", buf.set(2, 9));
    println!("set out of range: {}", buf.set(10, 9));
    println!("get 2: {:?}, get 10: {:?}", buf.get(2), buf.get(10));

    let empty = Buffer::new(0);
    println!("empty get(0): {:?}", empty.get(0));

    // Think further 3: Vec<u8> is Send and Sync automatically, so this wrapper is too.
    fn assert_send<T: Send>(_: &T) {}
    assert_send(&buf);
    println!("Buffer is Send");
}
