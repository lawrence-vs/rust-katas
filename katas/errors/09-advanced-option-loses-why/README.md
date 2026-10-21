# 09 advanced: `Option` loses the why

**Concept:** `None` says something failed, not what. A library error enum names each failure, renders a message, and keeps the underlying error as its `source()` so callers can log the chain.
**Source:** Network Programming With Rust (Chanda), implementing the Error trait for an enum, https://readwise.io/open/943199693; thiserror docs https://docs.rs/thiserror/2.0.17/thiserror/
**Fails with:** `E0432`
**Vault:** your note on error types for libraries

## Task
Make `cargo test` pass without changing the test or `Cargo.toml` (`thiserror` is already there). The messages must match the test exactly.

## Before you fix it
1. The test imports `UserError`. What variants and fields does it need?
2. What do `#[error(...)]` and `#[source]` generate for you?

## Answer in the PR
1. Why is `Option` the wrong return type for a parser?
2. What does `source()` give a caller that the message alone doesn't?
3. `thiserror` here, `anyhow` in an application: why the difference?

## Think further
1. What does your parser return for `"ada, 36, admin"`, `"ada,36"` and `" , 36"`? Decide which should be errors, and whether your variants can say so.
2. What does `load_user("")` return? Trace `split(',')` on an empty string: was the name really there?
3. For `"ada, old"`, loop on `source()` and print each error in the chain. Now suppose the top-level message also included the parse error's text. What would a log line that prints the whole chain look like?

## In the wild
- [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html): what `source()` is for, and what the docs say an error message should look like.
- [`std::io::ErrorKind`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html): callers branch on `NotFound`, `PermissionDenied` or `TimedOut`. That only works because the error says why.
- [`HashMap::get`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get): the honest `Option`. Absence is the only way a lookup can come up empty, so `None` says everything.
- Exchange-style: an order rejected with no reason. The client can't tell insufficient balance from a price off the tick size from a halted market, so it keeps retrying the halted market, and support can't answer "why was my order rejected?".

In a review, check:
- Functions that return `Option` but can fail in more than one way.
- `.ok()` or `.ok()?` that throws an error away.
- A `Display` message that repeats its source's message, so a logged chain says it twice.
- Error variants that don't carry the input that caused them.
