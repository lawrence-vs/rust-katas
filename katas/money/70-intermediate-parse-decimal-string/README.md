# 70 intermediate: parse a decimal string

**Concept:** A price on the wire is a decimal string. Parse it straight into whole minor units: only digits and one `.`, at most 4 places, `checked_*` when combining, and a typed error for each bad input. Going through `f64` truncates `0.57` to 5699 and accepts `1e3`, `NaN` and `-1`.
**Source:** The Rust Book ch. 9.2 "Recoverable Errors with Result", https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html; std [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html), [`str::split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once), [`u64::from_str_radix`](https://doc.rust-lang.org/std/primitive.u64.html#method.from_str_radix) (the optional `+` sign) and [`u64::checked_mul`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_mul); katas 07, 32 and 42
**Fails with:** `0.57 is exactly 5700 ten-thousandths` (the error-case tests also fail)
**Vault:** your note on parsing money from strings

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`, and keep the public API. None of these in `src/`: `f32`, `f64`, `as`, `unwrap`, `expect`, `panic!`, `saturating_*`, `wrapping_*`, `#[allow(...)]`.

## Before you fix it
1. What does `cargo test` print for `"0.57"`?
2. Predict `"1e3".parse::<f64>()` and `"+5".parse::<u64>()`.
3. What is `u64::MAX / 10_000`?

## Answer in the PR
1. Why did `"0.57"` become 5699, in terms of the value an `f64` holds and what `as u64` does to it?
2. Which of `"1e3"`, `"NaN"`, `"inf"` and `"-1"` did the float version accept, and what `Price` did each become?
3. Why reject `"1.23450"` instead of dropping the zero? What might a 5th place mean upstream?
4. Where can your parser overflow, and which `checked_*` catches each case?
5. Did you call `parse::<u64>()` on the parts? If so, how do you reject `"+5"`; if not, what did you write instead, and what does it cost?

## Think further
1. Negative prices (spreads, some futures): what changes in the type and the grammar?
2. A scale per instrument (8 places for BTC, 0 for JPY): could it be `Price<const SCALE: u32>`?
3. A `Display` that round-trips to `"12.3400"`.
4. serde `#[serde(try_from = "&str")]` at the JSON boundary.
5. A CSV of trades with one bad row: skip it or stop, and how do you report the line number?

## In the wild
- [`rust_decimal::Decimal::from_str_exact`](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html#method.from_str_exact): parses a decimal string and errors rather than rounding.
- Exchange REST APIs send price and quantity as decimal strings.
- The std [`from_str_radix`](https://doc.rust-lang.org/std/primitive.u64.html#method.from_str_radix) docs on the sign.

In a review, check:
- `parse::<f64>()` on a price.
- `as u64` after a float multiply.
- `parse::<u64>()` on a field where a sign is meaningful.
- Silently rounding away extra decimals.
