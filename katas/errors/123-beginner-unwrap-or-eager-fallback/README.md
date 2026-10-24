# 123 beginner: unwrap_or runs its fallback even when the value is there

**Concept:** `unwrap_or(f())` and `ok_or(g())` take finished values, so Rust builds the fallback or the error before the call, even when the `Option` is `Some` or the `Result` is `Ok`. `unwrap_or_else` and `ok_or_else` take a closure and run it only when needed.
**Source:** std `Option::unwrap_or_else` and `Option::ok_or_else` docs; Mastering Rust, combinators on `Option` and `Result` (idea only)
**Fails with:** `fallback is not run when value is present`
**Vault:** your note on eager versus lazy arguments

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API of `Config` and `Defaults`. In `src/`: no `unwrap`, `expect`, `panic!` or `#[allow]`.

## Before you fix it
1. Run `cargo test`. Read the counts in the failure messages. Which tests fail, and which pass?
2. Predict the fix in one sentence.
3. Why do the "missing" tests already pass on the starting code?

## Answer in the PR
1. Why was the fallback built when the key was present? Answer in terms of when function arguments are evaluated.
2. Which two combinators did you change, and what does each take in place of a value?
3. The fallback here only bumps a counter. What real cost could it hide (an allocation, a file read, a log line)?

## Think further
1. When is `unwrap_or(0)` fine, and when is it a bug?
2. What does `unwrap_or_default` evaluate, and when?
3. `a && b()` skips `b()` when `a` is false. Why can `unwrap_or` not do the same?
4. Which of `or`, `or_else`, `map_or`, `map_or_else` are eager?

## In the wild
- Clippy has a lint for this: `or_fun_call` (in `nursery`) flags a function call passed to `unwrap_or`, `ok_or` and friends.
- A cache lookup with `get(k).unwrap_or(load_from_disk(k))` reads the disk on every hit.
- An error built with `ok_or(format!(..))` allocates a string on every success.

In a review, check:
- A function call or `format!` inside `unwrap_or(..)`, `ok_or(..)` or `map_or(..)`.
- A fallback with a side effect, such as a log line or a counter.
