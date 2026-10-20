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
