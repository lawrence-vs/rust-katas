# 92 beginner: a secret that never reaches Debug

**Concept:** A key in a String is one {:?} away from a log; SecretString redacts it in Debug, and expose_secret() marks the one place it's read.
**Source:** secrecy docs, https://docs.rs/secrecy/0.10.3/secrecy/
**Fails with:** `the API key leaked into Debug output`
**Vault:** your note on secrets in logs

Why this crate: `secrecy` is the standard wrapper for secrets. The main alternative is a hand-written `impl Debug` that redacts the field. It works until someone adds a `#[derive(Debug)]` struct that holds the raw `String`, or a `format!` that includes it. `secrecy` makes every read an explicit, greppable `expose_secret()` and zeroizes the memory on drop.

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API. Store the key as `secrecy::SecretString` and call `expose_secret()` in exactly one place. None of `unwrap`, `expect`, `panic!` or `#[allow(...)]` in `src/`.

## Before you fix it
1. Run `cargo test`. Which tests fail, and which one passes on the starting code?
2. Where does the key show up besides the `Config`'s `Debug` output?

## Answer in the PR
1. Where could the key have ended up? Name three places.
2. Where is your one `expose_secret()`, and why there?
3. Why delete the key from `ApiError` instead of wrapping it?
4. One line: secrecy vs. a hand-written `Debug`.

## Think further
1. `serde` `Deserialize` into `SecretString` from env or a config file.
2. What zeroize does and doesn't protect against: copies made by `format!`, swap.
3. `Clone` on secrets: should a secret be easy to copy?
4. `tracing` `#[instrument]` captures arguments by `Debug`.

## In the wild
- `tracing::instrument` logging every argument of a function that takes a token.
- reqwest `HeaderValue::set_sensitive` keeps an auth header out of debug output.
- Panics and backtraces printing a struct's `Debug`.
- Error reporters (Sentry) capturing the `Debug` of errors.

In a review, check:
- `#[derive(Debug)]` on anything holding a token.
- `expose_secret()` outside the one transport call.
- Secrets copied into error variants.
- `{:?}` of config at startup.
