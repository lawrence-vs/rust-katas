# 05 intermediate: take, don't clone

**Concept:** Behind `&mut self` you may change a field but not move it out, because the struct must stay whole. `Option::take` moves the value out and leaves `None` behind: a move on purpose, not a clone.
**Source:** Network Programming With Rust (Chanda), mutable borrows, https://readwise.io/open/935627452
**Fails with:** `E0507`
**Vault:** your note on moving out of a borrow

## Task
Make `cargo test` pass without changing the test or the signature.

## Before you fix it
1. Why can't a value be moved out through `&mut`? What would `self` look like afterwards?
2. Try `.clone()` first and read which assertion fails.

## Answer in the PR
1. What does `take` leave behind, and why does that make the move legal?
2. What did `.clone()` cost here besides an allocation? (the test tells you)
3. Name the `std::mem` function that does the same for any `Default` type.
