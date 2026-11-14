# 217 intermediate: a backoff that is capped at every attempt

**Concept:** An exponential backoff must stay at or below its cap for every attempt number, including ones far past where `base << attempt` overflows.
**Source:** kraken-cli `crates/kraken-ws/src/backoff.rs:91-123` (MIT), https://github.com/krakenfx/kraken-cli
**Fails with:** `delay above max`
**Vault:** your note on exponential backoff

`delay(base, max, attempt)` should be `min(base * 2^attempt, max)`. `Backoff` hands out successive delays and can be reset; it is not the bug.

## Task
Make `cargo test` pass without changing the tests. Keep the doubling for small attempts.

## Related concepts

### `u64::saturating_mul`
Integer multiply that clamps at the type's bounds instead of wrapping. Use it for sizes and counters that must not roll over.
```rust
assert_eq!(u8::MAX.saturating_mul(2), u8::MAX);
```

### `checked_shl`
Shift that returns `None` when the shift amount is too large. Use it when the amount comes from outside.
```rust
assert_eq!(1u8.checked_shl(9), None);
```

### `Duration::mul_f64`
Scales a duration by a float. Use it for jitter.
```rust
let d = std::time::Duration::from_secs(10).mul_f64(0.5);
```

## Before you fix it
1. Run `cargo test` and read which attempt the proptest shrinks to.
2. At what attempt does `100 ms * 2^attempt` pass 30 s?

## Answer in the PR
1. Why does a shift overflow silently in release but panic in debug?
2. Why a proptest over `0..=200` instead of a table of chosen attempts?
3. What should the cap be in terms of the operator's wait?

## Think further
1. Jitter: what happens when a thousand clients back off in step?
2. When should `reset` run: on connect, or after a stable period?

## In the wild
- [proptest book](https://proptest-rs.github.io/proptest/)
- [`u32::checked_shl`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_shl)
