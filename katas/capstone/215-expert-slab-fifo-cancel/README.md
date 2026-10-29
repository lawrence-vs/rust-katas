# 215 capstone (expert): a price level you can cancel from in O(1)

**Concept:** A price level is a FIFO of resting orders, and a cancel names one order by id. Keep the orders in a `slab::Slab` and link them by index, so the level can unlink any order without walking the queue, and the freed slot is reused.
**Source:** `slab` crate docs, https://docs.rs/slab/0.4.12/slab/; own design (exchange-style price-time priority); katas 45, 21 and 12.
**Fails with:** `stage3_cancel_cost_does_not_grow_with_the_level ... FAILED`
**Vault:** your note on indices instead of pointers for linked structures

## Task
Make the 3 tests pass without changing them, `Cargo.toml` or any public signature. Keep `slab` (already in `Cargo.toml`). Orders live in a `slab::Slab`; no `Box` per order, no `Rc`, no `unsafe`. No `unwrap`, `expect`, `panic!`, and no `slab[i]` indexing (it panics; use `get` / `get_mut`). Stages 1 and 2 pass today and must still pass.

## Why
A scan is fine at 10 orders and a stall at 100k in one level. A `Box` per order is an allocation per rest, and pointers between boxes are hard to hold both ways under the borrow checker. Indices are `Copy`, safe to store, and checkable with `get`.

## Stages
Work in order. After each stage, write down your **Observation** (what the test showed), a **Question** it raised, and the **Insight** you'd keep. Put all three in the PR.

1. `stage1_cancel_head_middle_tail_keeps_fifo`: cancelling the head, a middle order and the tail must leave the rest in arrival order.
2. `stage2_freed_slot_never_cancels_a_stranger`: a slot that is freed and reused must never answer to an old id.
3. `stage3_cancel_cost_does_not_grow_with_the_level`: 100k orders in one level, cancelled newest first, within 2 seconds.

## Before you fix it
1. Run `cargo test`. Which stage fails, and what message does it print?
2. Why does stage 3 cancel in reverse arrival order, and what does that do to a scan from the front?
3. Besides the `position` scan, what does `VecDeque::remove(i)` cost?

## Answer in the PR
1. Stage 3: where did the time go before, and what does one cancel cost now?
2. What do `head`, `tail`, `prev` and `next` hold, and why `Option<usize>`?
3. Stage 2: what stale-id bug appears when a slab key is reused and the id index is not cleaned? Which two places must clean it?
4. Cancelling the head, the tail, the only order: which links change in each?
5. `Slab` vs `Box<Order>` per order: one concrete cost of each.
6. Which earlier kata does each stage lean on?

## Related concepts

### `VecDeque` as a FIFO
A ring buffer with O(1) push and pop at both ends. Use it for a queue where you only add at the back and take from the front.
```rust
let mut q = std::collections::VecDeque::new();
q.push_back(1);
let first = q.pop_front();
```

### `Vec` as an arena
Items are stored in one `Vec` and referred to by `usize` index. Use it when nodes need stable indices and never move in memory. A plain `Vec` never reuses a slot.
```rust
let mut arena: Vec<u64> = Vec::new();
let id = arena.len();
arena.push(42);
```

### `HashMap` as an id index
Maps a key to the position or handle of an item, so a lookup by id does not scan. Use it to find an order from its id.
```rust
let mut index: std::collections::HashMap<u64, usize> = Default::default();
index.insert(7, 0);
let at = index.get(&7).copied();
```

### `Rc<RefCell<T>>` linked nodes
Shared ownership with runtime borrow checks. Use it for graphs that need several owners. Cycles leak unless broken with `Weak`.
```rust
use std::cell::RefCell;
use std::rc::Rc;
let node = Rc::new(RefCell::new(0_u64));
*node.borrow_mut() += 1;
```

## Think further
1. A duplicate resting `OrderId` on `submit`: what does the index do with it, and what should be returned?
2. `best_bid` sums the whole level to get its quantity. Keep a `total` per level with `checked_add` (kata 42), so it costs O(1).
3. Slab keys are reused. Add a generation counter, so an old handle is detectably stale.
4. An amend that shrinks qty keeps its place; one that grows moves to the tail (kata 201). Which links change?

## In the wild
- [`Slab::insert`](https://docs.rs/slab/0.4.12/slab/struct.Slab.html#method.insert), [`Slab::try_remove`](https://docs.rs/slab/0.4.12/slab/struct.Slab.html#method.try_remove) and [`Slab::get_mut`](https://docs.rs/slab/0.4.12/slab/struct.Slab.html#method.get_mut): a key per order, freed keys reused, checked access.
- [`BTreeMap::first_entry`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.first_entry) and [`last_entry`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.last_entry): the best level, without looking it up twice.
- Matching engines index resting orders by id and link them per price level, so a cancel is a lookup plus an unlink.

In a review, check:
- A scan of a level to cancel an order.
- An `index` entry left behind after a fill or a cancel.
- `slab[i]`, `unwrap` or `expect` in the fix.
- A level left in the map when its last order is gone.
- Head or tail not updated when the unlinked order was at an end.
