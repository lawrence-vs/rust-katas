# 188 intermediate: a buy takes the cheapest ask first

**Concept:** Price-time priority. A buy walks the ask side from the lowest price up, and fills the oldest resting order at each price first. The sort order of the book decides which end you start from.
**Source:** std `BTreeMap` docs (iteration is in ascending key order, `iter_mut` and `rev`), https://doc.rust-lang.org/std/collections/struct.BTreeMap.html; own design (exchange order matching, price-time priority)
**Fails with:** `buy_takes_cheapest_ask_first`
**Vault:** your note on price-time priority in a matching engine

## Task
Make `cargo test` pass without changing the test. The fill order is part of the rule: cheapest first, not the order the asks were inserted.

## Before you fix it
1. Read the failing assertion. Which price filled first, and which should have?
2. Which end of a `BTreeMap<u64, _>` holds the lowest key, and which end does `.rev()` start from?

## Answer in the PR
1. Why does a buyer walk the asks from the low end? What would the same loop look like for a seller walking the bids?
2. The loop stops at `buy.price`. Why is that check on the price, not the quantity?

## Related concepts
- **`BTreeMap` range:** ordered map, cheap to find the best key. `asks.first_key_value()` gives the lowest ask directly.
- **`BinaryHeap`:** also gives the best item first, but cannot remove a single price level by key, so it suits fewer book operations.

```rust
let best = asks.first_key_value();
```
