# 87 intermediate: rust_decimal rounding

**Concept:** Money is a `Decimal` parsed from the string. Every rounding names its strategy: half-up for fees, half-to-even for settlement. Overflow is an error, not a clamped number.
**Source:** `rust_decimal` docs: [`RoundingStrategy`](https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html), [`Decimal::round_dp_with_strategy`](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html#method.round_dp_with_strategy), [`checked_mul`](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html#method.checked_mul) and [`FromStr`](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html#impl-FromStr-for-Decimal); katas 42, 46 and 70
**Fails with:** `a 0.125 tie settles to the even cent, 0.12` (the other tests also fail)
**Vault:** your note on rounding strategies
**Why this crate:** `rust_decimal` is a fixed-size, `Copy` 128-bit type (96-bit mantissa plus a scale); the main alternative, `bigdecimal`, is arbitrary-precision and allocates on the heap.

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`, and keep the public API. None of these in `src/`: `f32`, `f64`, `as`, `from_f64`, `unwrap`, `expect`, `panic!`, a bare `round_dp(` (it hides its strategy), `saturating_*`, `wrapping_*`, `#[allow(...)]`.

## Before you fix it
1. What does `f64::round` do with 12.5?
2. What is 1.005 x 100 in `f64`?
3. Which strategy does `Decimal::round_dp` use?

## Answer in the PR
1. Why did the fee tie come out as 1.00, in terms of the value the `f64` holds?
2. Why half-up for fees but banker's for settlement? What bias does half-up add over a million ties?
3. Why `round_dp_with_strategy` rather than `round_dp`?
4. What did `as i64` do to the overflow case?
5. Where did the starting code accept `"NaN"`?

## Think further
1. Rounding once per fill versus once on the total.
2. A scale per currency (JPY 0 places, BTC 8).
3. Directional strategies (`ToZero`, `AwayFromZero`) for house-favour quotes: see kata 46.
4. The serde string format for `Decimal` at the JSON boundary.
5. `from_str_exact` versus `from_str` when the input has more than 28 digits.

## In the wild
- [`rust_decimal`](https://docs.rs/rust_decimal/latest/rust_decimal/) docs on `RoundingStrategy`.
- Python `decimal.ROUND_HALF_EVEN` and `ROUND_HALF_UP`.
- Java `BigDecimal` with `RoundingMode`.
- IEEE 754's default rounding, roundTiesToEven.

In a review, check:
- `f64` on a money path.
- A bare `round_dp`.
- `as i64` after a float multiply.
- Rounding without a comment saying which strategy and why.
