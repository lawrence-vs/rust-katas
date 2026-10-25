# 180 beginner: logic in a lib, a thin main

**Concept:** One package can build one library crate and several binary crates. Logic lives in `src/lib.rs`, where `tests/`, doctests, `examples/` and other bins can `use kata_project_structure_180::...`. `main.rs` only reads arguments, calls the library, maps the result to an exit code and prints errors to stderr.
**Why:** Code in `main.rs` is reachable only by running the process. Integration tests can't import it, and its doc examples never run.
**Source:** The Rust Book, ch. 7 "Packages, Crates, and Modules" and ch. 11.3 "Test Organization" (section "Integration Tests for Binary Crates"). No Readwise highlight.
**Fails with:** `E0432`
**Vault:** your note on packages, crates and modules.

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the binary's output and exit codes. In `src/`: no `unwrap`, `expect`, `panic!` or `#[allow]`, and no new crates. `main.rs` may not contain the counting or parsing logic.

## Before you fix it
1. Run `cargo test` and read the error. Which crate does it say is missing?
2. Run `cargo test --test cli`. Why does that pass?
3. List the crates this package builds, with each one's name.

## Answer in the PR
1. Package vs crate vs module, using this kata's names.
2. Why can't `tests/kata.rs` see a function in `main.rs`, even after you write `pub fn`?
3. How many doctests ran before and after the fix, and why?
4. What did you leave in `main`, and why does it belong there and not in the library?
5. Which items did you make `pub`, and what does that promise to callers?

## Think further
1. Add a second binary in `src/bin/`. How does it call the library, and what is its name?
2. Write an `examples/` program that uses the lib. How is it run, and what does `cargo test` do with it?
3. When would the library return `Result<_, UsageError>` and `main` choose the exit code? Why shouldn't the library call `std::process::exit`?
4. Reading:
   - The Rust Book ch. 7 "Packages, Crates, and Modules", ch. 11.3 and ch. 14 "More About Cargo and Crates.io"
   - The Cargo Book: "Cargo Targets", "Package Layout", "Features" and "Specifying Dependencies"
   - Rust API Guidelines: "Documentation" (C-EXAMPLE)

## In the wild
- [ripgrep](https://github.com/BurntSushi/ripgrep): the `rg` binary is thin and the search logic lives in library crates under `crates/`.
- [cargo](https://github.com/rust-lang/cargo): the `cargo` library plus the `cargo` binary.
- `cargo new --lib`, versus adding a `main.rs` beside an existing `lib.rs`.

In a review, check for: a `main.rs` longer than its arg parsing and error mapping; `std::process::exit` inside library code; tests that only spawn the binary.

## Related concepts

### `src/bin/`
Each file there is its own binary crate in the same package.
```rust
// src/bin/other.rs
fn main() {
    println!("second binary");
}
```

### `examples/`
Example programs that are built by `cargo test` and run with `cargo run --example name`.
```rust
// examples/hello.rs
fn main() {
    println!("hello from an example");
}
```

### Doctests
Code blocks in `///` comments on a library are compiled and run as tests.
```rust
/// ```
/// assert_eq!(1 + 1, 2);
/// ```
fn two() -> i32 { 2 }
```

### Workspaces
Several packages that share one `Cargo.lock` and `target/`.
```toml
# Cargo.toml at the root
[workspace]
members = ["app", "core"]
```
