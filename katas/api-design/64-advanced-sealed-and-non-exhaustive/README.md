# 64 advanced: a sealed trait and a non_exhaustive enum

**Concept:** `#[non_exhaustive]` makes other crates add a `_` arm, and a private supertrait stops them implementing a trait, so a new variant or method isn't a breaking change.
**Source:** Rust for Rustaceans (Jon Gjengset), ch. 3 Designing Interfaces; Rust API Guidelines C-SEALED, https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed; The Rust Reference, `non_exhaustive` attribute, https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
**Fails with:** `Test compiled successfully, but it's marked` (both doc tests; the four integration tests pass)
**Vault:** your note on semver-safe API design

## Task
Make `cargo test` pass without changing `tests/`, `Cargo.toml` or the two doc tests. Keep every public name and signature. No `#[doc(hidden)]`, no `#[allow(...)]`, and don't make the enum or trait private.

The `/// CLUE:` lines are in `src/lib.rs`, next to the code they are about.

## Before you fix it
1. Why do both `compile_fail` doc tests compile today?
2. Which kind of crate is a doc test, and why does that matter here?
3. Run `cargo clippy --all-targets`. What does it say about the `_` arm in `tests/kata.rs`, and what will it say after your fix?

## Answer in the PR
1. Which changes to `RejectReason` and `Instrument` were breaking before your fix and aren't now? Name one that still is (e.g. removing a variant, or changing a method signature).
2. Why does a match inside the crate still not need `_`?
3. Before 1.40, crates added a hidden `__NonExhaustive` variant instead. That passes these tests too. What does `#[non_exhaustive]` give you that it doesn't?
4. How does sealing protect `round_price`'s `tick_size() > 0` assumption, and what would you do if users really needed their own instruments?

## Think further
1. `#[non_exhaustive]` on a struct, and why `Spot { .. }` literals then fail outside the crate.
2. Sealing a single method with a private argument type instead of the whole trait.
3. How `cargo-semver-checks` would have caught the starting code.
4. When exhaustive matching is what users actually want (e.g. a closed protocol enum you version yourself).

## In the wild
- [`std::io::ErrorKind`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html): `#[non_exhaustive]`, so std can add kinds without a major version.
- [`std::sync::atomic::Ordering`](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html): also `#[non_exhaustive]`.
- [`std::slice::SliceIndex`](https://doc.rust-lang.org/std/slice/trait.SliceIndex.html): sealed, so std can add methods.
- [`serde_json::value::Index`](https://docs.rs/serde_json/latest/serde_json/value/trait.Index.html): sealed.
- Exchange-style: an SDK whose order-reject codes grow every quarter. Each new code must not break every client's build.

In a review, check:
- A public enum in a library without `#[non_exhaustive]`.
- A public trait that the crate's own code relies on, with an unwritten invariant.
- A `_` arm flagged `unreachable_patterns`.
