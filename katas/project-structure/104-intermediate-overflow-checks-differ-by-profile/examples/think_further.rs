//! 104 intermediate: overflow checks differ by profile
//!
//! Think further, from the kata README:
//! 1. What do `wrapping_add`, `saturating_add` and `overflowing_add` return for 200 + 100? When is each the right answer?
//! 2. Setting `overflow-checks = true` under `[profile.release]` makes `+` panic everywhere. Why is that not the fix for input you do not control?
//! 3. Which profile do `cargo test` and `cargo test --release` use, and what would a release-only bug need to hide?
//! 4. Why is widening to `u16` before adding a different design from `checked_add`?

// Scratch file for the Think further items. Delete before the PR.
// Run with: cargo run --example think_further
// And:      cargo run --release --example think_further

fn main() {
    println!("debug_assertions on: {}", cfg!(debug_assertions));

    // Think further 1: four named behaviours for the same sum.
    println!("wrapping:    {}", 200u8.wrapping_add(100));
    println!("saturating:  {}", 200u8.saturating_add(100));
    println!("overflowing: {:?}", 200u8.overflowing_add(100));
    println!("checked:     {:?}", 200u8.checked_add(100));

    // Think further 4: widening cannot overflow, but the result no longer fits a u8.
    println!("widened:     {}", 200u16 + 100u16);
}
