# 88 intermediate: thiserror in the library, anyhow in the binary

**Concept:** A library should return an error type callers can match on, with the cause kept as `source()`. Only the binary, which just reports, uses a type-erased error and adds context. Here the library returns `anyhow::Error`, so callers can only compare strings, and `main` prints a bare message with no file and no cause chain.
**Source:** thiserror 2.0.21 docs (https://docs.rs/thiserror/2.0.21/thiserror/); anyhow docs, "Details" and the README's "Comparison to failure/thiserror" (https://docs.rs/anyhow)
**Fails with:** `error[E0432]: unresolved import`
**Vault:** your note on library vs application errors

**Why this crate:** `thiserror` writes the `Error`/`Display`/`From` impls you would otherwise write by hand (the main alternative is `snafu`); `anyhow` is the app-level error with context (the alternative is `eyre`/`color-eyre`).

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`, and update `src/main.rs`. None of `unwrap`, `expect`, `panic!` or `#[allow(...)]` in `src/lib.rs`. The `src/lib.rs` file must not mention the other crate at all, comments included.

## Before you fix it
1. Run `cargo test`. What does the compiler say is missing?
2. Run `cargo run -- /tmp/does-not-exist.conf`. What does the user learn about which file failed?

## Answer in the PR
1. Why can't a caller branch on an `anyhow::Error`? What would it have to do instead?
2. What do `{e}` and `{e:?}` print for an `anyhow::Error`?
3. Why must `BadValue`'s message not include the parse error's text?
4. Where does `.context(..)` belong, the library or the binary? Why?

## Think further
1. `downcast_ref` as the escape hatch on an `anyhow::Error`: when is it fine, and what does it cost?
2. `#[non_exhaustive]` on the enum: what does it let you add later?
3. `#[error(transparent)]`: when is it the right choice?
4. `eyre` and `color-eyre`: what do they add?

## In the wild
- `std::io::Error` and `serde_json::Error` are library errors callers match on (`kind()`, `is_eof()`).
- Binaries such as ripgrep and cargo report errors with a context chain at the top.
- A service config loader: the operator needs the file name and the bad key, the code needs the kind.

In a review, check:
- A public function returning `anyhow::Result` in a library.
- A `#[from]` that merges unrelated errors with no context.
- A `Display` that repeats its source's text.
- A binary printing `{e}` where `{e:?}` would show the chain.
