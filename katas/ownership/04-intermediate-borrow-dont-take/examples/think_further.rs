//! 04 intermediate: borrow, don't take
//!
//! Think further, from the kata README:
//! 1. After it passes, call the function with a `Vec<String>` you build inline, and then with a fixed-size array of `String`. Which calls compile, and what does each tell you about what the parameter accepts?
//! 2. The caller holds the names behind an `Arc<Vec<String>>`, shared with another thread. What does it need to write at the call site, and does anything get copied?
//! 3. Now the function also needs to sort the names for its own use. What are your options, and who pays for each?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::sync::Arc;

// Borrowing a slice takes any sequence of strings and copies nothing.
fn count_matching(names: &[String], needle: &str) -> usize {
    names.iter().filter(|n| n.contains(needle)).count()
}

// Think further 3: sorting needs a copy or a mutable borrow; this one copies only the refs it sorts.
fn sorted_refs(names: &[String]) -> Vec<&String> {
    let mut refs: Vec<&String> = names.iter().collect();
    refs.sort();
    refs
}

fn main() {
    // Think further 1: a Vec and an array both coerce to a slice.
    let built: Vec<String> = ["ada", "grace"].iter().map(|s| s.to_string()).collect();
    println!("vec: {}", count_matching(&built, "a"));

    let array = [String::from("linus"), String::from("ken")];
    println!("array: {}", count_matching(&array, "k"));

    // Think further 2: a shared Arc is read through a deref; nothing is cloned.
    let shared = Arc::new(built);
    println!("arc: {}", count_matching(&shared, "a"));

    println!("sorted: {:?}", sorted_refs(&shared));
}
