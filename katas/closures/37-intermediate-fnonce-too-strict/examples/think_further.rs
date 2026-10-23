//! 37 intermediate: fnonce-too-strict
//!
//! Think further, from the kata README:
//! 1. Write a function that calls its closure parameter twice but declares the weakest bound. Which error do you get, and which rule does it name?
//! 2. `for_each` on an iterator takes `FnMut`. Why not `Fn`? What would break if it took `Fn`?
//! 3. A thread-spawning API takes `F: FnOnce() + Send`. Why `FnOnce` and not `FnMut`? What does `Send` have to do with the closure's captures?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::thread;

// Think further 1: calling the closure twice needs FnMut at least; declaring FnOnce gives E0382.
fn call_twice<F: FnMut() -> u32>(mut f: F) -> u32 {
    f() + f()
}

// Think further 2: for_each needs FnMut because the closure may mutate what it captured.
fn for_each_demo(values: &[u32]) -> u32 {
    let mut total = 0;
    values.iter().for_each(|v| total += v);
    total
}

fn main() {
    let mut calls = 0;
    let sum = call_twice(|| {
        calls += 1;
        calls
    });
    println!("call_twice sum {sum}, calls {calls}");
    println!("for_each total {}", for_each_demo(&[1, 2, 3]));

    // Think further 3: spawn runs the closure once on another thread, so it owns what it captures.
    let name = String::from("worker");
    let handle = thread::spawn(move || format!("{name} done"));
    match handle.join() {
        Ok(msg) => println!("{msg}"),
        Err(_) => println!("worker panicked"),
    }
}
