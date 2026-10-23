//! 06 advanced: one lifetime too many
//!
//! Think further, from the kata README:
//! 1. `until_char`'s signature has no lifetimes written. Use the [elision rules](https://doc.rust-lang.org/reference/lifetime-elision.html) to write them out. Which input does the output borrow from?
//! 2. The haystack lives for the whole program, but each delimiter is a `String` built inside a loop. After your fix, which split items can you keep once the loop ends?
//! 3. Edge cases: what does `next` yield for `"a b "` split on `" "`, and for `"abc"` split on `""`? Run it, then decide what the type should promise.

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

// Think further 1: the output borrows from `haystack` only, so the delimiter gets its own lifetime.
fn until<'a>(haystack: &'a str, delimiter: &str) -> &'a str {
    haystack.split(delimiter).next().unwrap_or(haystack)
}

// Think further 2: the items borrow the haystack, not the delimiter, so they outlive the loop.
fn split_keep<'a>(haystack: &'a str, delimiter: &str) -> Vec<&'a str> {
    haystack.split(delimiter).collect()
}

fn main() {
    println!("until: {:?}", until("ada=36", "="));

    let kept = {
        let delimiter = String::from(",");
        split_keep("a,b,c", &delimiter)
    };
    println!("kept after the delimiter was dropped: {kept:?}");

    // Think further 3: the edge cases the type should promise.
    println!("'a b ' split on ' ': {:?}", "a b ".split(' ').collect::<Vec<_>>());
    println!("'abc' split on '': {:?}", "abc".split("").collect::<Vec<_>>());
}
