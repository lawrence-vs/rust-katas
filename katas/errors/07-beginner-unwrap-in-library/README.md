# 07 beginner: unwrap in a library

**Concept:** A library doesn't know what its caller wants to do with bad input, so it returns the error instead of panicking. `unwrap` belongs in tests and in `main`, not in library code.
**Source:** Network Programming With Rust (Chanda), methods that panic: expect() and unwrap(), https://readwise.io/open/954432478
**Fails with:** `E0308`
**Vault:** your note on panics vs errors

## Task
Make `cargo test` pass without changing the test. No `unwrap`, `expect` or `panic!` in `src/`.

## Before you fix it
1. What type does `str::parse::<u16>` return before the `unwrap`?
2. The test compares with `Ok(8080)`. What return type does that imply?

## Answer in the PR
1. Who should decide what happens on bad input, and why not the library?
2. Why is `"70000"` an error here? Which type enforced that?
3. Where in a program is `unwrap` acceptable?
