//! 123 beginner: unwrap_or runs its fallback even when the value is there
//!
//! Think further, from the kata README:
//! 1. When is `unwrap_or(0)` fine, and when is it a bug?
//! 2. What does `unwrap_or_default` evaluate, and when?
//! 3. `a && b()` skips `b()` when `a` is false. Why can `unwrap_or` not do the same?
//! 4. Which of `or`, `or_else`, `map_or`, `map_or_else` are eager?

// Scratch file for the Think further items. Delete before the PR.
// Run with: cargo run --example think_further

use std::cell::Cell;

fn present() -> Option<i32> {
    std::hint::black_box(Some(1))
}

fn present_text() -> Option<String> {
    std::hint::black_box(Some("x".to_string()))
}

fn main() {
    let calls = Cell::new(0);
    let costly = || {
        calls.set(calls.get() + 1);
        7
    };

    // Think further 3: an argument is evaluated before the call.
    let _ = present().unwrap_or(costly());
    println!("after unwrap_or on Some: {} call(s)", calls.get());

    // Think further 4: map_or takes a value, map_or_else takes a closure.
    let _ = present().map_or(costly(), |v| v + 1);
    println!("after map_or on Some: {} call(s)", calls.get());
    let _ = present().map_or_else(costly, |v| v + 1);
    println!("after map_or_else on Some: {} call(s)", calls.get());

    // Think further 2: unwrap_or_default builds the default only on None.
    let s: String = present_text().unwrap_or_default();
    println!("{s}");
}
