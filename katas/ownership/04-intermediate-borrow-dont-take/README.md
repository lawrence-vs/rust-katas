# 04 intermediate: borrow, don't take

**Concept:** A function that only reads data should borrow it. Taking `Vec<String>` by value forces every caller to give the Vec up, or `.clone()` it to keep using it.
**Source:** Network Programming With Rust (Chanda), introduction to the borrow checker, https://readwise.io/open/935627446
**Fails with:** `E0308`
**Vault:** your note on borrowing vs moving into a function

## Task
Make `cargo test` pass without changing the test. No `.clone()`, no `to_vec()`.

## Before you fix it
1. The test passes `&names`. What type does the function want, and what does the error say it got?
2. Which parameter type accepts a `&Vec<String>`, an array and a slice of a Vec?

## Answer in the PR
1. Why did the old signature force callers to clone, in terms of who owns the Vec?
2. `&Vec<String>` or `&[String]`: which did you pick, and what does the other cost a caller? (API Guidelines C-CALLER-CONTROL)
3. When is taking `Vec<String>` by value the right signature?
