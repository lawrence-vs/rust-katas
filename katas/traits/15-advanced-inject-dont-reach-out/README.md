# 15 advanced: inject, don't reach for a global

**Concept:** A function that reads the clock (or a singleton, or global state) itself can't be tested at 9am in the afternoon. Pass in the dependency behind a trait; production passes the real clock, tests pass a fixed one. Rust's traits replace the inheritance a mock framework would need.
**Source:** All Rust Features Explained, the trait system vs classical inheritance, https://readwise.io/open/841530717; Rust Design Patterns anti-patterns, https://rust-unofficial.github.io/patterns/anti_patterns/index.html
**Fails with:** `E0432`
**Vault:** your note on dependency injection with traits

## Task
Make `cargo test` pass without changing the test. Keep a real clock for production (`SystemClock`). No `unwrap` in `src/`.

## Before you fix it
1. The test imports a `Clock` trait with `fn hour(&self) -> u32`. Who implements it in the test, and who in production?
2. `&impl Clock` or `&dyn Clock`: will the test compile with either?

## Answer in the PR
1. Why can't a test control `greeting()` as it was?
2. Generic (`&impl Clock`) or trait object (`&dyn Clock`): which did you pick, and what does each cost?
3. Where else in a real service would you inject instead of reaching out? (name two)

## Think further
1. The rule is "hour < 12". At which exact second does the greeting change, and in whose time zone? Which part of the code should own the time zone: the clock, `greeting`, or the caller?
2. A fixed clock answers "what time is it?". A test for "a session expires after 30 minutes" also needs time to move. What would a test clock need for that, and how does `tokio::time::pause` handle it for async code?
3. `SystemTime` is not monotonic. If you measure how long a request took with two `SystemTime::now()` calls and NTP moves the clock in between, what do you get? Which clock should durations use, and does that one need injecting too?

## In the wild
- [std `SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html): "this time measurement is not monotonic". That is why `duration_since` returns a `Result`.
- [std `Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html): the monotonic clock for measuring durations.
- [tokio `time::pause`](https://docs.rs/tokio/1.53.2/tokio/time/fn.pause.html): tokio's test clock. Read which feature flag it needs and what auto-advance does.
- [quanta `Clock`](https://docs.rs/quanta/latest/quanta/struct.Clock.html): a production clock crate with a mock constructor built in. Compare it with the trait you wrote.
- [axum `State`](https://docs.rs/axum/latest/axum/extract/struct.State.html): axum's way to hand a handler its dependencies (pool, clock, config) instead of letting it reach for globals.
- [sqlx `Executor`](https://docs.rs/sqlx/latest/sqlx/trait.Executor.html): a query function that takes a generic executor runs on a pool or inside a transaction. The caller decides.
- Exchange-style: good-till-date order expiry, funding intervals, rate-limit windows and market open or close all depend on "now". When that is a hidden `now()` call, the only way to test midnight or a leap second is to wait for one.

In a review, check:
- `SystemTime::now()`, `Instant::now()`, `chrono::Utc::now()` or `std::env::var` called deep inside business logic.
- Tests that `sleep` to make time pass.
- A global client, config or clock in a `static`/`OnceLock` that tests can't replace.
- `SystemTime` used to measure a duration.
