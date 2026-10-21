# 45 expert: capstone: a mini order book

**Concept:** Everything at once: a matching engine keeps each side's price levels in a sorted map (`BTreeMap`), each level as a FIFO queue (`VecDeque`), prices as whole ticks (never a float), fills an incoming order across as many levels as it crosses, and rests the remainder. A cancel removes the order and any level it leaves empty.
**Source:** own design (an exchange-style exchange prep); std `BTreeMap`, https://doc.rust-lang.org/std/collections/struct.BTreeMap.html, and `VecDeque`, https://doc.rust-lang.org/std/collections/struct.VecDeque.html
**Fails with:** `stage1_prices_are_whole_ticks ... FAILED`
**Vault:** your MOC note linking the money, collections and capstone track notes

## Task
Make the four stages pass in order without changing the test. Keep every public signature. In `src/`: no `f32` or `f64`; `Price` holds whole ticks and is `Ord`; each side's levels live in a `BTreeMap` keyed by `Price`, and each level is a FIFO queue. No `unwrap`, `expect` or `panic!`.

## Stages

Work in order. After each stage, write down your **Observation** (what the test showed), a **Question** it raised, and the **Insight** you'd keep. Put all three in the PR.

1. `stage1_prices_are_whole_ticks`: a price that can't round-trip its own ticks is not a price.
2. `stage2_best_price_first_then_oldest`: price first, then time.
3. `stage3_partial_fill_rests_the_remainder`: one order, many fills, and what is left rests.
4. `stage4_cancel_removes_the_order_and_its_empty_level`: an empty level is not a best price.

## Before you fix it
1. Run `cargo test stage1`. What did `115` come back as, and which line lost the cent?
2. In stage 2, which maker does the starting code pick for order 5, and why that one?
3. Stage 2's first assertions pass today. Will they still pass after you switch to a `BTreeMap`, and what do you have to get right for them to?

## Answer in the PR
1. Stage 1: why did `1.15 * 100.0` not give `115`, and why can't `f64` be a `BTreeMap` key?
2. Stage 2: which end of the bids map is the best price, and which end of the asks map? What does finding it cost now, compared with the `Vec` scan?
3. Stage 3: your matching loop stops for one of two reasons. Name both.
4. Stage 4: `cancel` gets only an id. How do you find its level without scanning every level, and what would `best_ask` return if you left an empty level in the map?
5. Which earlier kata does each stage lean on?

## Think further
1. Self-trade: the same account rests a sell and then sends a crossing buy. Where in `submit` would you check it, and what should happen: cancel the old order, the new one, or both?
2. An immediate-or-cancel order must never rest. A market order has no limit price. What do the two need from `submit` that a limit order doesn't?
3. A caller submits an `OrderId` that is already resting. What does your index do with it today, and what should `submit` return instead?
4. `best_bid` sums the whole queue on every call, and that sum could overflow `u64`. How would you keep a running total per level, with `checked_add` (kata 42), so it costs O(1)?

## In the wild
- [`BTreeMap::first_key_value`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.first_key_value), [`last_entry`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.last_entry) and [`OccupiedEntry::remove`](https://doc.rust-lang.org/std/collections/btree_map/struct.OccupiedEntry.html#method.remove): the best level, and removing it when it empties, without looking it up twice.
- [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html): `push_back` to rest, `front_mut`/`pop_front` to match, which is time priority within a level.
- [`f64::total_cmp`](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp): the reason `f64` isn't `Ord` (NaN), and why wrapping it to get an ordering still leaves you with float prices.
- Exchange-style: an exchange API doc publishes price levels (price, total qty) per side, plus a checksum the client computes over its own copy of the top levels. That only works if both sides agree on every price exactly.

In a review, check:
- Floats anywhere on the price path, including `as f64` just to compare or sort.
- A linear scan of every order for the best price or a cancel.
- Levels that are left in the map after their last order fills or cancels.
- A matching loop that fills one maker and returns, or drops the remainder instead of resting it.
