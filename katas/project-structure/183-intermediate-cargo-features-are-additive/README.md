# 183 project-structure (intermediate): Cargo features are additive

**Concept:** Cargo turns a dependency's features on as the union of what every crate in the build asks for, so a feature may only add items; it must never change or remove one, and the default set should pull in nothing heavy.
**Source:** The Cargo Book, "Features" (https://doc.rust-lang.org/cargo/reference/features.html); kata 104.
**Fails with:** `slug_is_the_same_in_every_build`
**Vault:** your note on Cargo features and feature unification.

This kata is one std-first library with two optional features. `unicode` is on by default and pulls in `unicode-normalization`. `json` pulls in `serde_json`. Some of the code is written in a way that breaks when a feature is on, so read the tests before you change anything.

## Task
Make all five commands pass: `cargo test`, `cargo test --no-default-features`, `cargo test --features unicode`, `cargo test --features json`, `cargo test --all-features`. Change only `src/lib.rs` and the `[features]` table in `Cargo.toml`. Leave `tests/` and `[dependencies]` alone. Keep the public API and both features. In `src/`: no `unwrap`, `expect`, `panic!` or `#[allow]`.

## Before you fix it
1. Run `cargo test`. Which tests fail, and what are `left` and `right`?
2. Run `cargo test --all-features`. Why is this one a compile error rather than a failing test?
3. Run `cargo tree -e normal` and `cargo tree -e normal --no-default-features`. What is the difference?
4. Try `cargo build --features serde_json`. Why does a feature with that name exist?

## Answer in the PR
1. Two crates in one build depend on this one, one with `unicode` and one without. Which `slug` does each crate get, and why?
2. Why is "replace `slugs_csv` with `slugs_json` when `json` is on" breaking for a crate you have never heard of?
3. What does `dep:` change about the features this crate exposes?
4. What does a consumer that writes `default-features = false` get now, and what did it get before?

## Think further
1. When are two features truly mutually exclusive (two TLS backends, for example), and what does the Cargo Book suggest instead of `compile_error!`?
2. Resolver version 2 (the default from edition 2021) stops unifying some features. Which ones? Check the Cargo Book, "Feature resolver version 2".
3. Why is adding a new feature to `default` a breaking change, and removing one even more so? Check the Cargo Book, "SemVer compatibility".
4. How would you run every feature combination in CI? Look up `cargo-hack --feature-powerset`.

Reading list:
- The Rust Book, ch. 7 "Managing Growing Projects with Packages, Crates, and Modules"
- The Rust Book, ch. 14 "More About Cargo and Crates.io"
- The Cargo Book, "Features" (sections "Optional dependencies", "Feature unification", "Mutually exclusive features", "Inspecting resolved features") and "Specifying Dependencies"
- Rust API Guidelines, C-FEATURE

## In the wild
- serde's `derive` and `std` features, and tokio's opt-in features with `default = []`. These are additive by design.
- A logging or TLS feature that swaps an implementation and breaks a crate elsewhere in the dependency graph.

In a review, check:
- `#[cfg(not(feature = ...))]` on a `pub` item
- a feature that changes a function body
- a heavy crate in `default`
- an optional dependency without `dep:`
- CI that only runs default features

## Related concepts

### `cfg`
Compiles an item only when a condition holds at build time. Main use: platform-specific code.
```rust
#[cfg(unix)]
fn home() -> &'static str { "/home" }
```

### `cfg_attr`
Applies an attribute only when a condition holds. Main use: optional derives gated on a feature.
```rust
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
struct Id(u64);
```

### `#[cfg(test)]`
Compiles an item only for `cargo test`. Main use: a test module that stays out of the release build.
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn ok() {}
}
```

### `target_os`
A `cfg` key that matches the operating system. Main use: choosing a path separator or system call per OS.
```rust
if cfg!(target_os = "windows") { "\\" } else { "/" }
```
