# 01 beginner: mutable variable vs mutable reference

**Concept:** `let mut p = &x` lets you point `p` somewhere else; only `&mut x` lets you change `x` through `p`.
**Source:** Rust for Rustaceans, ch. 1 "Values, variables and pointers" (Listing 1-1).
**Vault:** your note on references (link it from its `code:` property).

## Task
Make `cargo test` pass. Don't change the test.

## Before you fix it
1. The error says "cannot assign to `*slot`, which is behind a `&` reference". Which of the two is read-only: the variable `slot` or what it points at?
2. Predict the one-word change.

## Answer in the PR
1. What is the difference between `let mut p = &x;` and `let p = &mut x;`?
2. The test passes `&mut x`. Why did that compile against a `&i32` parameter before your fix?
