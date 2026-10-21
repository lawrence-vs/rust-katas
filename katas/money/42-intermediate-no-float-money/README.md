# 42 intermediate: no float money

**Concept:** An `f64` can't hold most decimal fractions exactly, so float prices drift as they add up. Store money as a whole number of minor units (cents) and use `checked_*` arithmetic, because an overflowing balance must be an error, not a panic or a silent wrap.
**Source:** The Rust Reference, arithmetic overflow, https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow, and `u64::checked_add`, https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add
**Fails with:** `E0432`
**Vault:** your note on why money is never a float

## Task
Make `cargo test` pass without changing the test. No `f32` or `f64` anywhere in `src/`. No `unwrap`, `expect`, `panic!`, `saturating_*` or `wrapping_*`.

## Before you fix it
1. Predict `0.1 + 0.2 == 0.3` for `f64`, then check it in the Rust Playground. Why?
2. What is the smallest amount in these tests, and how many of it make $19.99?
3. In a debug test build, what does `u64::MAX + 1` do? And in `--release`?

## Answer in the PR
1. Why did the float version drift, in terms of which values an `f64` can represent exactly?
2. What does a bare `+` do on overflow in a debug build and in a release build, and why is neither acceptable for a balance?
3. Why `checked_*`, and not `saturating_*` or `wrapping_*`? Name what each would have done to the overflow test's balance.
4. Why wrap the `u64` in a `Money` type instead of passing bare `u64` cents around? (See kata 10.)

## Think further
1. BTC trades in satoshis (8 decimal places) and JPY has no minor unit. What does `Money` need to know so that `from_cents` stays honest across currencies, and what happens if you add two amounts with different scales?
2. Three fills total 1000 cents over 3 units. What is the average price? Which way do you round, and where does the remainder go?
3. A P&L goes negative. Do you switch to `i64`, or keep `u64` and add a `checked_sub` that returns an `InsufficientFunds` error? Which one stops an account from being overdrawn?
4. A price arrives on the wire as the string `"0.10"`. How do you parse it into cents without ever going through `f64`?

## In the wild
- [`u64::checked_add`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add) and [`checked_mul`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_mul): `Option` instead of a panic or wrap.
- [`std::time::Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html): std's own fixed-point type, whole seconds plus nanoseconds as integers, with `checked_add`/`checked_sub` returning `Option`.
- [`rust_decimal::Decimal`](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html): a 96-bit integer plus a decimal scale, with `checked_add`/`checked_mul`, for when amounts need more than one fixed scale.
- [Stripe currencies](https://docs.stripe.com/currencies): every API amount is an integer in the currency's smallest unit.
- Exchange-style: [Kraken's REST API](https://docs.kraken.com/api/) takes order `price` and `volume` as decimal strings (`'50000'`, `'1.0'`), not JSON numbers, so a client never has to round-trip them through a float.

In a review, check:
- `f32`/`f64` anywhere on a price, balance or fee path, including `as f64` casts "just for the division".
- Bare `+`, `-`, `*` on balances; `saturating_*` that silently caps a balance.
- Money serialised as a JSON number instead of a string or an integer of minor units.
