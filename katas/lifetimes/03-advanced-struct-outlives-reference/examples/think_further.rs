//! 03 advanced: a struct outliving its reference
//!
//! Think further, from the kata README:
//! 1. Change `reconnect` to return the `Connection` itself instead of a `String`. With your fix, what must the return type say, and where does the URL's data have to live?
//! 2. Use `db` inside a `std::thread::spawn` closure. What does the compiler require of the URL? What changes with [`std::thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html)?
//! 3. Sometimes the URL is a literal and sometimes it's built at runtime. What would a `Cow<'a, str>` field buy, and what would it cost?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::borrow::Cow;

// Think further 3: Cow lets the URL be a literal (borrowed) or built at runtime (owned).
struct Connection<'a> {
    url: Cow<'a, str>,
}

impl<'a> Connection<'a> {
    fn new(url: impl Into<Cow<'a, str>>) -> Self {
        Connection { url: url.into() }
    }

    // Think further 1: returning the connection ties its URL's lifetime to the caller's input.
    fn reconnect(&self) -> Connection<'_> {
        Connection { url: Cow::Borrowed(&self.url) }
    }
}

fn main() {
    // A literal is borrowed; no allocation.
    let literal = Connection::new("db://localhost");
    println!("literal: {} (owned? {})", literal.url, matches!(literal.url, Cow::Owned(_)));

    // A runtime-built URL is owned by the Connection.
    let built = Connection::new(format!("db://{}", "replica"));
    println!("built: {} (owned? {})", built.url, matches!(built.url, Cow::Owned(_)));

    // Think further 2: a scoped thread may borrow the connection; a plain spawn may not.
    std::thread::scope(|s| {
        s.spawn(|| println!("thread sees {}", literal.url));
    });
    println!("reconnected: {}", literal.reconnect().url);
}
