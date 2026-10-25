//! 185 beginner: dependency hygiene, dev vs normal and pins
//!
//! Think further, from the kata README:
//! 1. What does `cargo update -p itoa --precise <v>` change, and in which file?
//! 2. What does `cargo audit` read, and why does it need the lockfile?
//! 3. `rust-version` in `[package]` vs the repo's `rust-toolchain.toml`: what does each enforce, and for whom?
//! 4. Two crates in one workspace both use itoa: what does `[workspace.dependencies]` plus `itoa.workspace = true` buy you?

// Scratch file for the Think further items, same as katas/project-structure/104. Run with: cargo run --example think_further

fn main() {
    // Think further 1 and 2 are answered by running the commands from the README, not by this file.
    println!("{}", kata_project_structure_185::format_count(7));
}
