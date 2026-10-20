# 29 expert: a safe API over raw pointers

**Concept:** Raw pointers in a public API hand every invariant to every caller. Keep the pointer private, check bounds in safe methods, free it in `Drop`, and let `unsafe` shrink to a few audited lines (or disappear).
**Source:** The Rustonomicon, meet safe and unsafe, https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html
**Fails with:** `E0133`
**Vault:** your note on safe abstractions

## Task
Make `cargo test` and `cargo +nightly miri test` pass without changing the test. No public raw pointers, no `unsafe fn`, no leak (Miri reports leaks).

This is an `expert` kata: the bug compiles and may even pass `cargo test`. Run `cargo +nightly miri test`; CI does too.

## Before you fix it
1. Read E0133. Why can't the test call `get` without `unsafe`?
2. Who frees the allocation from `Box::into_raw`?

## Answer in the PR
1. Fix A: `Box<[u8]>` and no `unsafe`. Fix B: keep the pointer private with `Drop` and `// SAFETY:` comments. Which did you pick, and when is B worth it?
2. What did the leak look like in Miri's output?
3. Which invariant does each `// SAFETY:` comment rely on?
