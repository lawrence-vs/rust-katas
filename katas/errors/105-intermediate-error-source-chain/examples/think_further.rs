//! 105 intermediate: an error source chain
//!
//! Think further, from the kata README:
//! 1. What does a `Box<dyn Error>` field do to `downcast_ref` on the wrapper, and what does it do to `source()`?
//! 5. How many errors are in the chain, and which one does `downcast_ref::<io::Error>` find first?

// Scratch file for the Think further items. Delete before the PR.
// Run with: cargo run --example think_further

use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
struct Outer(Box<dyn Error>);

impl fmt::Display for Outer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "outer")
    }
}

impl Error for Outer {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0.as_ref())
    }
}

fn main() {
    let boxed: Box<dyn Error> = Box::new(io::Error::from(io::ErrorKind::WouldBlock));
    let outer = Outer(boxed);

    println!(
        "downcast on the wrapper finds io::Error: {}",
        (&outer as &dyn Error).downcast_ref::<io::Error>().is_some()
    );

    let mut depth = 0;
    let mut current: Option<&(dyn Error + 'static)> = Some(&outer);
    while let Some(e) = current {
        println!("level {depth}: {e} (is io::Error: {})", e.is::<io::Error>());
        depth += 1;
        current = e.source();
    }
}
