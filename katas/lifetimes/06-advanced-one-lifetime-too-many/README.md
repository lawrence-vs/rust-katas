# 06 advanced: one lifetime too many

**Concept:** One lifetime on a struct with two references says both borrow from the same place. When the haystack and the delimiter come from different places, that single `'a` ties the output to the shorter-lived input. Give each source its own lifetime.
**Source:** Network Programming With Rust (Chanda), lifetime parameters on structs, https://readwise.io/open/935627468; Rust for Rustaceans ch. 1 as background
**Fails with:** `E0515`
**Vault:** your note on what a lifetime parameter claims

## Task
Make `cargo test` pass without changing the test or `until_char`'s signature. No `Box::leak`, no `'static`. Two designs work; which would you ship in a library?

## Before you fix it
1. Read E0515. The data `delim` owns dies at the `}`. Why does the compiler think the result points into it?
2. Draw it: which variable does each `&str` field point into?

## Answer in the PR
1. Fix A: a second lifetime for the delimiter. Fix B: a generic delimiter type (a `Delimiter` trait for `&str` and `char`), so `until_char` passes `c` directly. Which do you ship, and what does each cost a caller?
2. With one lifetime, the compiler picked the shorter of the two borrows. Why is that the only sound choice?
