# 28 expert: unsafe to dodge the borrow checker

**Concept:** `unsafe` doesn't switch the borrow rules off; it switches the checking off. A reference into a Vec that survives `push` dangles when the Vec reallocates. The program may still print the right answer, which is why Miri exists.
**Source:** Rust for Rustaceans ch. 1, pointers point to places, https://readwise.io/open/1041420683; Miri, https://github.com/rust-lang/miri
**Fails with:** `Undefined Behavior`
**Vault:** your note on what unsafe actually allows

## Task
Make `cargo +nightly miri test` pass without changing the test. No `unsafe` left in `src/`.

This is an `expert` kata: the bug compiles and may even pass `cargo test`. Run `cargo +nightly miri test`; CI does too.

## Before you fix it
1. Run `cargo test`, then `cargo +nightly miri test`. Why does only Miri fail?
2. The test made capacity 1. What happens to the first element's address on the first `push`?

## Answer in the PR
1. What rule did `unsafe` let you break, and what did it cost?
2. Why did plain `cargo test` pass?
3. Name a case where `unsafe` is the right tool, and what it must promise.
