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

## Think further
1. `capacity` counts messages, not bytes. The test sends 1 KiB messages; an attacker sends eight 16 MiB ones. What bounds memory now, and what else would you have to limit?
2. Every client shares one queue. One noisy client keeps it full. What do the quiet clients see, and how could you make it fair?
3. What does `ingest(0)` do? Should a caller be able to ask for that?
4. The worker panics and drops `rx`. What does `accept` return now, and is `Rejected` the right thing to tell the client in that case?

## In the wild
- [tokio `mpsc`](https://docs.rs/tokio/1.53.2/tokio/sync/mpsc/index.html): the module docs on bounded vs unbounded channels and what an unbounded one costs.
- [tokio `Semaphore`](https://docs.rs/tokio/1.53.2/tokio/sync/struct.Semaphore.html): bounds how much work is in flight, not how long a queue is.
- [tower `load_shed`](https://docs.rs/tower/latest/tower/load_shed/index.html): a middleware that rejects requests when the service behind it isn't ready.
- [tower `ConcurrencyLimitLayer`](https://docs.rs/tower/latest/tower/limit/concurrency/struct.ConcurrencyLimitLayer.html): the same limit as a layer you can put on an axum or tonic service.
- [axum `DefaultBodyLimit`](https://docs.rs/axum/latest/axum/extract/struct.DefaultBodyLimit.html): a byte limit per request body. Count limits and byte limits are different defences.
- Exchange-style: API rate limits: over the limit, a client gets an error back right away; it is not queued.

In a review, check:
- `unbounded_channel`, or a `Vec`/`VecDeque` that grows, fed straight from network input.
- A capacity counted in items when each item can be any size.
- A capacity with no stated reason, or one hard-coded in several places.
- Rejections with no metric or log, so nobody sees the service shedding load.
