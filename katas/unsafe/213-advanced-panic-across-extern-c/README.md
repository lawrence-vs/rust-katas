# 213 advanced: a panic inside an `extern "C"` function

**Concept:** since Rust 1.81, a panic that tries to leave an `extern "C"` function aborts the process. C cannot catch a Rust unwind, so the function must return an error code instead of panicking.
**Source:** The Rust Reference, "Function types / extern functions and unwinding", https://doc.rust-lang.org/reference/items/external-blocks.html; The Rustonomicon, FFI and unwinding, https://doc.rust-lang.org/nomicon/ffi.html
**Fails with:** `thread caused non-unwinding panic. aborting.` (the test binary aborts with SIGABRT). CI checks for `non-unwinding panic`.
**Vault:** none yet; write one on error codes at an FFI boundary.

## Task
Make `cargo test` pass without changing the tests or the signature of `parse_qty`. A bad number must return `-3`. Do not use `catch_unwind` to hide the panic.

## Before you fix it
1. Read the abort message. Which function panicked, and which call in it?
2. List every code the doc comment promises. Which input produces each one?

## Answer in the PR
1. Why can't a C caller handle a Rust panic, even with `catch_unwind` in the outer layer?
2. Why is `-3` the right return for `"abc"`, and not a panic?
3. What is the difference between `let Ok(n) = ... else { }` and `.unwrap()` for a caller who passes user input?

## Think further
1. `catch_unwind` turns a panic into an `Err` inside the function. When is that still needed, and when do you convert the panic into a code at the top of the function?
2. Could `s.parse()` panic on any input? What does `i64::from_str` never do?

## Related concepts
- `Result<T, E>` with `?`: normal error flow inside Rust. None: no close siblings for an `extern "C"` boundary.
- `std::panic::catch_unwind`: catches a panic in Rust code and returns `Err`. Use it at a boundary you own, not as a shortcut here.
```rust
let r = std::panic::catch_unwind(|| risky());
```

## In the wild
- Market-data and order-entry libraries expose C ABIs; each entry point returns a status code and writes results through an out parameter.
- `unwrap()` on client input in an exported function is a process-wide abort, not a rejected request.

In a review, check:
- `unwrap()`, `expect()`, indexing or arithmetic that can overflow inside any `extern "C"` function.
- A status code that is documented but never produced.
