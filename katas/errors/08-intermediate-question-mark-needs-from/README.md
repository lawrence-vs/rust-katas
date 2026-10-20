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
