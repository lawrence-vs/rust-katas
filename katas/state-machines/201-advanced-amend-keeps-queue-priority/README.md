# 201 state-machines (advanced): an amend keeps its place in the queue

**Concept:** Price-time priority. Each price is a queue, and a `seq` number records when an order arrived. An amend that only shrinks the size keeps the order's place and its `seq`. Anything that takes more from the orders behind, or moves the order to another price, sends it to the back of its level with a new `seq`.
**Source:** std `BTreeMap` and `VecDeque` docs; own design (exchange price-time priority); katas 45 and 12.
**Fails with:** `size_increase_loses_queue_position`
**Vault:** your note on price-time priority in a matching engine

## Task
Make `cargo test` pass by changing only `Book::amend` in `src/lib.rs`. Don't change the tests, `Cargo.toml` or any public signature. No `unwrap`, `expect` or `panic!` in `src/`, and no new crates.

The rules for an amend:
- A new quantity at or below the old one, at the same price, keeps the place and the `seq`.
- A bigger quantity, or a new price, goes to the back of its level with a new `seq`.
- A rejected amend changes nothing.

## Before you fix it
1. Run `cargo test`. Which three tests fail, and at which assertion does each one stop?
2. In a `VecDeque`, which end is the front and which is the back? Which end does `push_front` use?

## Answer in the PR
1. Why may a size decrease keep its place, but a size increase may not? What does the increase take from the orders behind it?
2. Why is `seq` a better clock than the wall clock for queue position? What changes when an order is amended, and what does not?
3. Why is a price change a new order at the new level, and not a move within the old one?
4. After a rejected amend, what must the book still hold: the order, its place, and the `seq` counter?

## Think further
1. A partial fill reduces the quantity. Is a fill an amend, and should it keep priority?
2. Some venues lose priority on any amend. What do you give up by keeping priority on a decrease?
3. Iceberg or reserve orders: when the hidden part replenishes, does it keep the visible place?
4. Is the lost place disclosed to the client, and should the amend response say so?

## In the wild
- Exchange order-modify rules and cancel-replace semantics. Check the venue's rulebook at build time; drop what you can't confirm.

In a review, check for:
- an in-place edit that changes the size of an order that is then sorted by its old position;
- a new `seq` on a size decrease;
- a price change that stays at the front of the new level;
- a rejected amend that has already removed the order.

## Related concepts

### `BTreeMap` best-price lookup
An ordered map, so the best price on a side is one call away. Use it for the top of a book.
```rust
let best = bids.last_key_value();
```

### `VecDeque` as a FIFO
A double-ended queue, so pushing at one end and popping at the other keeps arrival order. Use it for orders resting at one price.
```rust
q.push_back(order);
let first = q.pop_front();
```

### `BinaryHeap` priority queue
A heap that returns its largest item first, so it can pick the best order. It cannot remove an order by id, so it suits fewer book operations than a per-price queue.
```rust
let mut heap = BinaryHeap::new();
heap.push(Reverse((price, seq)));
```
