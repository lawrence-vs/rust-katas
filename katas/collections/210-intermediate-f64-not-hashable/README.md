# 210 intermediate: f64 is not Eq or Hash

**Concept:** `HashSet` needs `Eq` and `Hash` on its element. `f64` is only `PartialEq`, because `NaN != NaN` breaks reflexivity, so `f64` can't be `Eq` or `Hash`. Money and prices should be integer ticks, which are exact and hashable.
**Source:** std `f64` and `Eq` docs, https://doc.rust-lang.org/std/cmp/trait.Eq.html; Rust Reference, "Traits" (derive); own design (exchange price ticks)
**Fails with:** `E0277`
**Vault:** your note on why prices are integers

## Task
Make `cargo test` pass without changing the test. Keep `PriceLevel` and its `price: f64` field. The set may store a key built from a level, not the level itself. Do not hash a float value directly.

## Before you fix it
1. Read E0277. Which trait does the compiler say `f64` lacks, and which line of the derive asked for it?
2. Why is `NaN` the reason `f64` cannot be `Eq`?

## Answer in the PR
1. What is your price unit (ticks, minor units) and who decides its scale? Why is that safer than a float?
2. A feed sends `101.50` and `101.5` as separate messages. What should the set count, and where would you normalise?

## Related concepts
- **`PartialEq` vs `Eq`:** `Eq` adds reflexivity. Only types whose equality is total can be map keys.
- **Newtype over an integer:** a `Price(u64)` with `Eq` and `Hash` derived. Kata 10 covers newtypes.

```rust
#[derive(PartialEq, Eq, Hash)]
struct Qty(u64);
```
