# 186 intermediate: two references in, one out

**Concept:** Lifetime elision only works when the output can borrow from exactly one input. With two reference inputs and a reference output, you name the lifetime yourself and say which inputs the output may borrow from.
**Source:** The Rust Reference, lifetime elision, https://doc.rust-lang.org/reference/lifetime-elision.html
**Fails with:** `E0106`
**Vault:** your note on lifetime elision

## Task
Make `cargo test` pass without changing the test. Keep the function returning a borrow, not a `String`.

## Before you fix it
1. Read E0106. How many reference inputs does the compiler count, and why can't it pick one for the output?
2. Which input can the returned `&str` point into: the row, the fallback, or both?

## Answer in the PR
1. Which lifetime did you name, and which parameters carry it? Why not give `fallback` a different one?
2. What would `fallback: &'static str` change for callers, and when is that the right signature?

## Related concepts
- **Lifetime elision on one input:** the output borrows from the single reference input. `fn first(row: &str) -> &str` needs no annotation.
- **`'static`:** a reference that lives for the whole program, such as a string literal. It satisfies any lifetime, but makes callers pass literals only.

```rust
fn first(row: &str) -> &str {
    row.split(',').next().unwrap_or("")
}
```
