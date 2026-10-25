//! 162 advanced: a mutant your tests let live
//!
//! Think further, from the kata README:
//! 1. `cargo install --locked cargo-mutants`. Its baseline needs green tests, so `cargo mutants --dir <this kata>` stops on the starting code. Run `cargo mutants --dir <this kata> -- -- --skip the_cases_kill_the_ge_mutant`, then read `mutants.out/missed.txt`. Does it list the `>` mutant from `src/lib.rs`? Why not?
//! 2. `cargo mutants --list --diff --dir <this kata>`: which change does the tool make to `>=`, and which existing row already kills it?
//! 3. When is a surviving mutant fine to leave (equivalent mutants, logging)? How would you mark it skipped?
//! 4. Why run cargo-mutants on a schedule or on changed files, not on every CI push?

// Scratch file for the Think further items, as in kata 161. Run with: cargo run --example think_further

use kata_testing_162::{LARGE_FEE_DIVISOR, LARGE_TRANSFER, SMALL_FEE, transfer_fee};

// Think further 2: a hand-written `>` mutant of the real function, unlike the `<` cargo-mutants makes.
fn transfer_fee_gt(amount: u64) -> u64 {
    if amount > LARGE_TRANSFER {
        amount / LARGE_FEE_DIVISOR
    } else {
        SMALL_FEE
    }
}

fn main() {
    for amount in [99_999, 100_000, 100_001] {
        println!(
            "{amount}: transfer_fee {} mutant {}",
            transfer_fee(amount),
            transfer_fee_gt(amount)
        );
    }
}
