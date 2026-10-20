# 21 intermediate: a bounded queue for untrusted input

**Concept:** An unbounded channel fed from the network grows as fast as a client can send: a memory-exhaustion DoS. A bounded `channel(n)` with `try_send` turns overload into a fast, explicit rejection (backpressure).
**Source:** tokio `mpsc::channel` docs, https://docs.rs/tokio/1.53.2/tokio/sync/mpsc/fn.channel.html; the Rust security gist in your Readwise, https://gist.github.com/auser/c3161f55a8393faa8af5ddda68c6befa
**Fails with:** `floods_are_rejected`
**Vault:** your note on backpressure

## Task
Make `cargo test` pass without changing the test. `accept` must not wait.

## Before you fix it
1. What bounds the memory this queue can use today?
2. `send().await` or `try_send`: which one tells the client no?

## Answer in the PR
1. Describe the attack the old code allowed, with numbers.
2. Why reject instead of waiting when the queue is full? When would waiting be right?
3. What should the network task do with a `Rejected`?
