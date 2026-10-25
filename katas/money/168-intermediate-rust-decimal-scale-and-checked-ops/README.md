# 168 money (intermediate): rust_decimal scale and checked ops

**Concept:** A `Decimal` is a value plus a scale, so `1.5 == 1.50` while each prints its own scale. Output fixes the scale on purpose: round with a named strategy, then format to 4 places. `+` and `*` panic on overflow, so money uses `checked_add` and `checked_mul`. Amounts cross JSON as strings, never numbers.
**Source:** [rust_decimal docs](https://docs.rs/rust_decimal/1.43.0/rust_decimal/) (`Decimal::scale`, `normalize`, `rescale`, `round_dp_with_strategy`, `checked_add`, `checked_mul`, `Display`, module `rust_decimal::serde::str`); katas 42, 70 and 87
**Fails with:** `1.5 prints as 1.5000` (the other tests also fail)
**Vault:** your note on scale versus value

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`, and keep the public API. Banned in `src/`: `f32`, `f64`, `from_f64`, `unwrap`, `expect`, `panic!`, `saturating_`, `wrapping_`, a bare `round_dp(`, `#[allow(...)]`, and a hand-written string pad (`push('0')`, `repeat`).

## Before you fix it
1. What do `d("1.5").scale()` and `d("1.50").scale()` return? Is `d("1.5") == d("1.50")`?
2. Predict `format!("{:.4}", d("0.12346"))`, then check it.
3. Predict what `rescale(4)` does to `Decimal::MAX`.
4. Which panic message does the starting `statement_total` hit?

## Answer in the PR
1. Scale versus value: why are `1.5` and `1.50` equal but printed differently?
2. Why did neither `rescale` alone nor `{:.4}` alone pass?
3. What does a JSON number go through before it becomes a `Decimal` with the default impl, and which input in the tests shows the loss?
4. Why `checked_add` in a `try_fold` (or a loop with `?`) rather than `saturating_add`?
5. When would you call `normalize()`, and why not on output here?

## Think further
1. `round_dp_with_strategy` on `-0.00001` gives a negative zero. What does it print, and should a statement show it?
2. A scale per currency (JPY 0, BTC 8) instead of a fixed `AMOUNT_DP`.
3. `Hash` and `Eq` for `Decimal` keys in a `HashMap`: are `1.5` and `1.50` the same key?
4. A string with more than 28 significant digits: `from_str` versus `from_str_exact`.

## In the wild
- Statements and invoices printed to a fixed number of places.
- Exchange REST APIs that send prices as JSON strings.
- Ledger totals checked for overflow.

In a review, check:
- `to_string()` on money output.
- `+`, `*` or `sum()` on `Decimal`.
- A `Decimal` field without a string serde attribute.
- `{:.N}` without a rounding step before it.

## Related concepts

### `Decimal::normalize`
Removes trailing zeros. Use it for canonical keys or when comparing text.
```rust
assert_eq!(d("1.50").normalize().to_string(), "1.5");
```

### `Decimal::rescale`
Sets the scale in place, padding or rounding `MidpointAwayFromZero`, and keeps a smaller scale if the mantissa can't fit.
```rust
let mut x = d("1.5");
x.rescale(3);
assert_eq!(x.to_string(), "1.500");
```

### `saturating_add`
Clamps at `MAX`/`MIN` instead of failing. Use it for counters where a cap is fine, never balances.
```rust
assert_eq!(Decimal::MAX.saturating_add(Decimal::ONE), Decimal::MAX);
```

### `serde-float` feature
Serialises a `Decimal` as a JSON number. Use it only when a peer demands numbers and the precision loss is accepted.
```toml
rust_decimal = { version = "1", features = ["serde-float"] }
```
