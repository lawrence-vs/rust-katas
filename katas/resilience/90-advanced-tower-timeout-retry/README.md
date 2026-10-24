# 90 advanced: a timeout and a bounded retry, not a retry-forever loop

**Concept:** Each attempt gets a deadline; only transient errors are retried, a bounded number of times, with growing waits. tower composes this as layers around any `Service`.
**Why tower:** it is the `Service`/`Layer` standard that axum, tonic and hyper share, so one policy wraps any client. The main alternative, a hand-rolled `tokio::time::timeout` plus loop (or a crate like `backon`), has to be rewritten for each client.
**Source:** tower 0.5.3 docs: [`ServiceBuilder`](https://docs.rs/tower/0.5.3/tower/struct.ServiceBuilder.html), [`timeout`](https://docs.rs/tower/0.5.3/tower/timeout/index.html), [`retry::Policy`](https://docs.rs/tower/0.5.3/tower/retry/trait.Policy.html), [`retry::backoff`](https://docs.rs/tower/0.5.3/tower/retry/backoff/index.html) (no Readwise highlight was available to the builder)
**Fails with:** `fetch hung`
**Vault:** your note on retries and timeouts

The starting `fetch` wraps an upstream `tower::Service` in a hand-written `loop`. The tests use fake upstreams and paused tokio time, so they run in milliseconds. Each test has a 60 s guard on the paused clock; that guard is what prints `fetch hung`.

## Task
Make `cargo test` pass without changing the tests. Use `ServiceBuilder` with `timeout` and `retry`, and write your own `tower::retry::Policy`. No hand-written loop, no `unwrap`/`expect`/`panic!` in `src/`, and keep the signature of `fetch`.

## Before you fix it
1. Run `cargo test` and read the CLUEs.
2. Predict which errors a second attempt could change.

## Answer in the PR
1. Why is retrying `NotFound` wrong?
2. Why must only idempotent requests be retried?
3. Which layer order did you pick, and what does the other order mean?
4. Why does `Retry` need `Clone` on both the service and the policy?
5. What does paused time buy these tests?

## Think further
1. Jitter and retry storms: what happens when a thousand clients back off in step? (See kata 82.)
2. A retry budget (`tower::retry::budget`) shared across all requests.
3. One overall deadline versus a deadline per try.
4. A circuit breaker after repeated failures (kata 97).
5. A non-idempotent `POST`: what would you need before retrying it?

## In the wild
- [`tower-http` `TimeoutLayer`](https://docs.rs/tower-http/latest/tower_http/timeout/index.html)
- tonic and hyper clients are built on tower services.
- [`tower::retry::budget::TpsBudget`](https://docs.rs/tower/0.5.3/tower/retry/budget/struct.TpsBudget.html)
- AWS Architecture Blog, "Exponential Backoff and Jitter".

In a review, check:
- `loop` plus `continue` on `Err`.
- A retry with no cap.
- Retry on every error kind.
- No timeout inside a retry.
- Retrying non-idempotent calls.
