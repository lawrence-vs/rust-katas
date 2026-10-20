# 10 beginner: newtypes, not primitives

**Concept:** Four `u64` arguments let a caller swap `from` and `amount` and still compile. A one-field newtype per meaning makes the compiler check it, and `#[repr(transparent)]` guarantees it costs nothing.
**Source:** Rust API Guidelines, C-NEWTYPE, https://rust-lang.github.io/api-guidelines/type-safety.html
**Fails with:** `E0432`
**Vault:** your note on primitive obsession

## Task
Make `cargo test` pass without changing the test. Use `#[repr(transparent)]` on both newtypes.

## Before you fix it
1. The test imports `AccountId` and `Cents`. What must each derive for the test to compile?
2. Swap `bob` and `Cents(300)` in your head. What would the old signature have done?

## Answer in the PR
1. What bug class did the newtypes remove, and when is it caught now?
2. What does `#[repr(transparent)]` promise that a plain one-field struct doesn't?
3. Should `Cents` implement `Add`? What would that let a caller do with an `AccountId`?
