# 23 advanced: an untrusted length prefix

**Concept:** A length read off the wire is an attacker's number. Check it against a limit before allocating, and never index past what you received: return an error instead of panicking.
**Source:** Network Programming With Rust (Chanda), parsing in networking applications, https://readwise.io/open/900600932
**Fails with:** `rejects_oversized_length`
**Vault:** your note on parsing untrusted input

## Task
Make `cargo test` pass without changing the test. No `unwrap`, `expect` or panicking index in `src/`.

## Before you fix it
1. A client sends `FF FF FF FF`. What does `Vec::with_capacity` try to do?
2. Which lines can panic on a short buffer?

## Answer in the PR
1. Name the two attacks the old code allowed.
2. Why check `MAX_FRAME` before allocating, not after?
3. Where should `MAX_FRAME` come from in a real service?
