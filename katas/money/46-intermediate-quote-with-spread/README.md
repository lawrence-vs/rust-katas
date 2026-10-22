# 46 intermediate: quote with a spread

**Concept:** A quote is integer minor units: the spread is applied off mid, everything is multiplied in a type wide enough to hold it, there is one division at the end, and that division rounds in the house's favour (a client buy up, a client sell down). Overflow is an error via `checked_*`, never a saturated or wrapped number.
**Source:** `u128::div_ceil`, https://doc.rust-lang.org/std/primitive.u128.html#method.div_ceil, and `TryFrom` for integers, https://doc.rust-lang.org/std/convert/trait.TryFrom.html
**Fails with:** `a buy rounds up, in the house's favour`
**Vault:** your note on rounding in the house's favour

## Task
Make `cargo test` pass without changing the tests. No `f32` or `f64` and no `as` casts in `src/`. No `unwrap`, `expect`, `panic!`, `saturating_*` or `wrapping_*`.

## Before you fix it
1. What does `.round()` give for 65.00001 cents? And for 97.500015?
2. How many satoshis x cents x 10_050 is a 100 BTC buy at $65,000? Does that fit in a `u64`?

## Answer in the PR
1. Which way did `.round()` move the buy and the sell, and who gained the part cent?
2. Why does the house round a client buy up and a client sell down? What does round-to-nearest cost over a million small quotes?
3. Why divide once at the end instead of computing the ask in cents and then multiplying by size?
4. Why a `u128` product and `u64::try_from`, rather than `u64` with `checked_mul`? Which test tells the two apart?
5. What did `as u64` do to the overflow and over-100% spread results, and why is an `Err` better?

## Think further
1. An asset with 18 decimals (wei), or a quote currency with 0 decimals: do the scale constants and `u128` still fit?
2. A fee on top of the spread: do you round once or twice, and where does the remainder go?
3. The quote shows an ask price on a tick but charges a total. Must the two agree?
4. Minimum notional: should a quote that rounds to 0 cents be an error?

## In the wild
- std [`u128::div_ceil`](https://doc.rust-lang.org/std/primitive.u128.html#method.div_ceil), [`checked_mul`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_mul) and [`u64::try_from`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html): rounding direction and overflow stated in the types.
- [`rust_decimal::RoundingStrategy`](https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html): directional variants such as `ToZero` and `AwayFromZero` next to the midpoint ones.
- An RFQ/OTC desk quotes a two-way price as mid plus and minus a half-spread, and rounds each side against the client.
- Broker statements round fees up to the next minor unit.

In a review, check:
- `.round()` or `as u64` on a money path.
- Rounding mid-calculation instead of once at the end.
- The spread applied in floats.
