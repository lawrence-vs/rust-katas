//! 32 beginner: `&str`, not `String`
//!
//! Think further, from the kata README:
//! 1. `"e\u{301}"` is an `e` followed by a combining accent: one letter on screen. How many `char`s is it, and what does your `truncate(.., 1)` return? What would you need to count what a user calls a letter?
//! 2. A database column holds at most 32 **bytes**. Which `str` method tells you whether a byte offset is safe to cut at, and how would you find the largest safe offset at or below 32?
//! 3. When is taking `String` the right signature? Think of a struct field, or a function that stores or returns the text. What does `Cow<'_, str>` give a function that only sometimes changes its input?
//! 4. Where is `len()` the right answer? Think of buffer sizes and length prefixes on the wire.

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::borrow::Cow;

// Think further 2: find the largest char boundary at or below a byte limit, so the cut is valid UTF-8.
fn cut_at_bytes(s: &str, limit: usize) -> &str {
    if s.len() <= limit {
        return s;
    }
    let mut end = limit;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

// Think further 3: Cow borrows when nothing changes and owns when a rewrite is needed.
fn normalise(input: &str) -> Cow<'_, str> {
    if input.contains('\t') {
        Cow::Owned(input.replace('\t', " "))
    } else {
        Cow::Borrowed(input)
    }
}

fn main() {
    // Think further 1: one visible letter, two chars: "e" plus a combining accent.
    let accented = "e\u{301}";
    println!(
        "bytes {}, chars {}",
        accented.len(),
        accented.chars().count()
    );
    println!(
        "truncate to 1 char: {:?}",
        accented.chars().take(1).collect::<String>()
    );

    println!("cut 'héllo' at 2 bytes: {:?}", cut_at_bytes("héllo", 2));

    // Think further 4: len() is the byte count, right for a length prefix or buffer size.
    println!("len of 'héllo' in bytes: {}", "héllo".len());

    println!("{:?}", normalise("no tabs"));
    println!("{:?}", normalise("a\tb"));
}
