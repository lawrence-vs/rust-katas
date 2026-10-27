# 211 intermediate: BinaryHeap pops the greatest first

**Concept:** `BinaryHeap` is a max-heap: `pop` returns the largest element. A priority queue for "lowest first" needs the order reversed, with `std::cmp::Reverse` or an explicit ordering. Pick the direction from the rule, not from the default.
**Source:** std `BinaryHeap` docs, https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html (max-heap, `Reverse` for min-heap); std `cmp::Reverse` docs, https://doc.rust-lang.org/std/cmp/struct.Reverse.html
**Fails with:** `pops_lowest_price_first`
**Vault:** your note on priority queues in a matching engine

## Task
Make `cargo test` pass without changing the test. Keep `Asks` as the type, with the same methods.

## Before you fix it
1. Read the failing assertion. Which price came out first, and which should have?
2. Which end of the heap's order does `pop` take from?

## Answer in the PR
1. Why does a sell side want lowest first while a bid side wants highest first? Name the matching rule that makes each correct.
2. `Reverse` wraps the value. What does it cost at `pop` time, and is there a case where you would write the ordering yourself instead?

## Related concepts
- **`BTreeMap` keyed by price:** orders levels, and `first_key_value` finds the lowest. Kata 188 uses it.
- **`Reverse<T>`:** flips `Ord` so a max-heap pops the smallest value.

```rust
use std::cmp::Reverse;
let mut heap = BinaryHeap::new();
heap.push(Reverse(5_u64));
```
