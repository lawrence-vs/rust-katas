# 02 intermediate: moving out of a Vec

**Concept:** `=` moves the value out of a place. A `Vec` owns its elements, so you can't move one out through an index: it would leave a hole.
**Source:** Rust for Rustaceans, ch. 1 (the `=` operator stores a value in a place).
**Vault:** your note on moves.

## Task
Make `cargo test` pass. CI also runs `cargo clippy --all-targets -- -D warnings`, so run that too. There are two good fixes; pick one, then explain the other.

## Before you fix it
1. Read "cannot move out of type `[String]`, a non-copy slice". Why would `&[i32]` compile?
2. Which trait makes the difference?

## Answer in the PR
1. Fix A: `.clone()`. Fix B: return `&str` borrowed from the vector. What does each cost, and who owns what afterwards?
2. With fix B, why don't you need to write a lifetime in the signature? (Look up lifetime elision.)
