//! 28 expert: unsafe to dodge the borrow checker
//!
//! Think further, from the kata README:
//! 1. The test uses `Vec::with_capacity(1)`. With `with_capacity(10)`, no `push` reallocates. Would Miri still complain about the original code? Is the code then correct, or only lucky? (Read `Vec::as_ptr`'s note on the aliasing model.)
//! 2. Make the element type `String` instead of `i32`. What stops `v.push(*first)` compiling even in your fixed version, and what does each way around it cost?
//! 3. `for x in v.iter() { v.push(*x); }` is the same bug as a loop. Safe Rust rejects it; C++ calls it iterator invalidation. What would Miri report if `unsafe` forced it through?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

// Think further 1 and 3: pushing while a reference into the Vec is alive is the bug that unsafe would hide.
// The safe version copies the value out before the push, so the borrow ends first.
fn push_first_again(v: &mut Vec<i32>) {
    let first = v[0];
    v.push(first);
}

// Think further 2: a String element can't be copied out with *first; clone it instead, which costs an allocation.
fn push_first_string_again(v: &mut Vec<String>) {
    let first = v[0].clone();
    v.push(first);
}

fn main() {
    let mut nums = vec![1, 2, 3];
    push_first_again(&mut nums);
    println!("nums: {nums:?}");

    let mut words = vec![String::from("a")];
    push_first_string_again(&mut words);
    println!("words: {words:?}");

    // Think further 3: a loop that pushes while iterating is rejected by the borrow checker, not undefined behaviour.
    // for x in nums.iter() { nums.push(*x); }   // E0502: cannot borrow `nums` as mutable
    println!("the loop version is a compile error, so it never runs");
}
