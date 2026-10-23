//! 01 beginner: mutable variable vs mutable reference
//!
//! Think further, from the kata README:
//! 1. Once it passes, make `replace` generic over `T`. Which line stops compiling when `T` is `String`, and which rule from kata 02 does it break?
//! 2. Write a call that passes the same `x` as both arguments. Does it compile? Why does the order in which arguments are evaluated matter?
//! 3. Try to hold two exclusive references to `x` at once. What does the compiler say, and why does that rule make a function that writes through a reference safe to call?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

// Writes through a reference: the signature says the caller's value changes.
fn bump(counter: &mut u32, by: u32) {
    *counter += by;
}

// Think further 1: a generic version works for any T, because it only moves values in and out.
fn replace<T>(slot: &mut T, value: T) -> T {
    std::mem::replace(slot, value)
}

fn main() {
    let mut x = 1;
    bump(&mut x, 5);
    println!("x after bump: {x}");

    // `let mut p = &x` only re-points p; writing through p needs a &mut reference.
    let mut p = &mut x;
    *p += 1;
    println!("x after writing through p: {x}");

    let mut name = String::from("old");
    println!("replaced: {}", replace(&mut name, String::from("new")));
    println!("now: {name}");

    // Think further 2 and 3 are compile errors, so they stay as comments:
    // bump(&mut x, &mut x)  -> E0499, two mutable borrows of x in one call
    // let a = &mut x; let b = &mut x; a.clone_from(&1); -> E0499 if a is used after b is taken
}
