# 97 advanced: a circuit breaker, not a retry storm

**Concept:** After N consecutive failures, stop calling the dependency and fail fast; after a cool-down read from an injected clock, let one trial call through.
**Source:** Michael Nygard, *Release It!*, ch. 5 Stability Patterns, "Circuit Breaker"; kata 15 (inject the clock). (No Readwise highlight was available to the builder.)
**Fails with:** `test open_fails_fast_without_calling_dependency ... FAILED`
**Vault:** your note on stability patterns

The starting `Breaker` does what many clients do: every request retries the failing dependency 3 times and never stops. During an outage, 1000 requests hit the dependency 3000 times. The tests have a fake clock that they move by hand, so they run in milliseconds and never sleep.

## Task
Make `cargo test` pass without changing the tests or any public signature. No `unwrap`, `expect` or `panic!` in `src/`. No `std::thread::sleep` and no tokio time: read the time only through `Clock`.

## Before you fix it
1. Which tests already pass, and why?
2. How many times does the dependency get called in `outage_calls_are_bounded` today?

## Answer in the PR
1. Why does retrying inside every request turn an outage into a storm?
2. Why let one trial through instead of closing as soon as the cool-down ends?
3. Why count consecutive failures, and what does a failure rate over a window buy you?
4. Why does the breaker read the time through `Clock` and not `Instant::now()`?
5. What should a caller do with `CallError::Open`?

## Think further
1. Many tasks sharing one breaker, with only one half-open trial in flight.
2. Which errors should count: a timeout yes, a "bad request" no. That is a predicate.
3. Retries with backoff and jitter outside the breaker (kata 90 has the retry layer).
4. One breaker per dependency, or per host?
5. Metrics and logs on state changes.
6. `tokio::time::Instant` with `start_paused` as the alternative clock.

## In the wild
- resilience4j `CircuitBreaker` (Java).
- Polly (.NET) circuit breaker policy.
- Envoy outlier detection, which ejects failing hosts from a pool.
- Hystrix, the original Netflix library, now in maintenance mode.

In a review, check:
- Retries with no breaker in front of them.
- An `Instant + Duration` that can overflow and panic.
- A breaker that never goes half-open.
- Half-open letting every call through, not just one trial.
- A cool-down that doesn't restart after a failed trial.
