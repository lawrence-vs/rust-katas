# 11 intermediate: bool flags to an enum

**Concept:** Three bools make eight combinations; an order has four real states. An enum makes the impossible combinations unrepresentable, and transitions become a match on one value.
**Source:** Rust API Guidelines, C-CUSTOM-TYPE (arguments convey meaning through types, not bool), https://rust-lang.github.io/api-guidelines/type-safety.html
**Fails with:** `E0432`
**Vault:** your note on making invalid states unrepresentable

## Task
Make `cargo test` pass without changing the test. No bool fields left on `Order`.

## Before you fix it
1. List the eight bool combinations. Which four are real?
2. Which transitions are allowed from each real state?

## Answer in the PR
1. Which impossible states did the enum remove?
2. Why does `state()` return a copy instead of exposing the field?
3. How would kata 12's typestate change this API, and what would it cost a caller that stores orders in a Vec?
