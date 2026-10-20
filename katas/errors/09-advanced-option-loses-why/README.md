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
