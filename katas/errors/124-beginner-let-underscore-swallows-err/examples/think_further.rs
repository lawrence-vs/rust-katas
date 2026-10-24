//! 124 beginner: `let _ =` swallows the error
//!
//! Think further, from the kata README:
//! 1. After the fix, a failed `read_to_string` can leave part of the text in the buffer. Who sees it?
//! 2. `let _ = x;` and `let _unused = x;` differ in when the value is dropped. Which one matters for a lock guard?
//! 3. Sometimes ignoring an error is right (a best-effort cleanup). How do you make that visible to a reader?
//! 4. `#[must_use]` is on `Result`. Which lint or clippy check would catch a discarded `Result` in a statement?

// Scratch file for the Think further items. Delete before the PR.
// Run with: cargo run --example think_further

use std::io::Read;
use std::sync::Mutex;

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    // Think further 1: bytes read before the error stay in the buffer.
    let mut buf = String::new();
    let mut src = "ok".as_bytes().chain(&[0xff][..]);
    let res = src.read_to_string(&mut buf);
    println!("result {res:?}, buffer {buf:?}");

    // Think further 2: `_` drops at once; `_name` lives to the end of the scope.
    let _ = Noisy("underscore");
    let _kept = Noisy("named");
    println!("end of the first part");

    // Think further 3: a visible, deliberate ignore.
    let lock = Mutex::new(0);
    let _ = lock.lock().map(|mut n| *n += 1); // best effort: a poisoned counter is fine here
    println!("counter {:?}", lock.lock().map(|n| *n));
}
