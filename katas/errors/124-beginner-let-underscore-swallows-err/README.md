# 124 beginner: `let _ =` swallows the error

**Concept:** `let _ = call();` compiles even when `call` returns a `Result`, so the error is dropped and a failure looks like success; `?` hands it to the caller.
**Source:** The Rust Book ch. 9, recoverable errors with `Result`; std `Read::read_to_string` docs
**Fails with:** `a_failed_read_is_an_error_not_an_empty_string`
**Vault:** your note on panics vs errors

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API. In `src/`: no `unwrap`, `expect`, `panic!` or `#[allow]`.

## Before you fix it
1. Run `cargo test`. Which tests fail, and which pass on the starting code?
2. Both functions already return a `Result`. Why does the compiler not warn about the discarded one?
3. Predict the fix in one sentence.

## Answer in the PR
1. Why was it a bug, in terms of the `Result` value and where it went?
2. What does `insert_all` promise when it returns `Ok(n)`, and what did it promise before?
3. Which fix did you pick for the batch (stop at the first error, or collect all), and what does the other cost?

## Think further
1. After the fix, a failed `read_to_string` can leave part of the text in the buffer. Who sees it?
2. `let _ = x;` and `let _unused = x;` differ in when the value is dropped. Which one matters for a lock guard?
3. Sometimes ignoring an error is right (a best-effort cleanup). How do you make that visible to a reader?
4. `#[must_use]` is on `Result`. Which lint or clippy check would catch a discarded `Result` in a statement?

## In the wild
- A config or log loader that returns an empty string when the disk fails, so the service starts with no rules.
- A batch import that reports "done" while some rows hit a unique constraint.
- Clippy's `let_underscore_must_use` (restriction group) flags exactly this shape; it is off by default.

In a review, check:
- `let _ =` on a call whose return type is `Result`.
- A `Result`-returning function whose body can only ever return `Ok`.
- A loop that counts attempts, not successes.
