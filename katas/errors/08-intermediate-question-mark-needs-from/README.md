# 08 intermediate: `?` needs a From

**Concept:** `?` returns early and converts the error with `From::from`. Two error types meet in one function only when there is a conversion between them.
**Source:** Network Programming With Rust (Chanda), enums carrying data and error handling, https://readwise.io/open/943199004
**Fails with:** `E0277`
**Vault:** your note on how `?` converts errors

## Task
Make `cargo test` pass without changing the test or `ConfigError`'s variants. No `unwrap`.

## Before you fix it
1. Read E0277. Which trait does it say is missing, and for which two types?
2. What does `?` desugar to?

## Answer in the PR
1. Fix A: `impl From<ParseIntError> for ConfigError`. Fix B: `.map_err(ConfigError::BadNumber)?`. Which did you pick, and when is the other better?
2. Why keep the `ParseIntError` inside `BadNumber` instead of a `String`?

## Think further
1. Add a second key, `retries`, parsed as a `u8`. Both parses fail with the same error type. With your fix, can a caller tell which key was bad? What would it take?
2. The config now comes from a file, so reading it can fail with `std::io::Error`. What changes in `read_timeout`'s signature and in `ConfigError`? Then look at `#[derive(PartialEq)]`: does it still compile, and what would the tests compare instead?
3. Try `timeout_ms` set to `"0"`, `"-5"` and `"18446744073709551616"`. Which variant does each land in? Should a zero timeout be an error of its own?

## In the wild
- [`std::convert::From`](https://doc.rust-lang.org/std/convert/trait.From.html): the trait `?` calls on the way out. Read what the docs say about using it for error types.
- [`sqlx::Error`](https://docs.rs/sqlx/0.9.0/sqlx/enum.Error.html): one enum for every way a query can fail. Look at its trait implementations: which source errors convert in by themselves, and which variants can only be built by hand?
- [`tonic::Status`](https://docs.rs/tonic/0.14.6/tonic/struct.Status.html): a gRPC handler's error. Note how a handler turns its own error into a `Status`, and which gRPC `Code` the client ends up seeing.
- Exchange-style: an order-entry handler meets decode errors, risk rejects and database errors in one function. A blanket `From` that turns all of them into one `Internal` variant makes every `?` compile. Then the client gets "internal error" for a price off the tick size, and nobody can alert on database failures separately.

In a review, check:
- `From` impls that send two different failures to the same variant.
- `map_err(|e| e.to_string())` or `format!` that throws away the original error.
- `Box<dyn Error>` in a library's public signature, where callers can't match on it.
- Several `?` on the same error type in one function: can the caller still tell which step failed?
