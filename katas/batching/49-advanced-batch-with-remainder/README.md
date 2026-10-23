# 49 advanced: a batch with a remainder

**Concept:** One provider call per pair per flush, for the pair's total. A short fill goes to the oldest orders first. What wasn't filled goes back to the front of the pair's queue, so no client's lots disappear.
**Source:** std [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html), [`BTreeMap::entry`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.entry), `u64::checked_add` docs; own design (an exchange hedging client orders with a liquidity provider); katas 34, 42, 45.
**Fails with:** `` `one provider call per pair` ``
**Vault:** your note on batching and partial fills

## Task
Make `cargo test` pass without changing the tests. Keep the public API. No `unwrap`, `expect`, `panic!` or unchecked `+`/`-` on quantities in `src/`. std only. Don't call the provider for a pair with nothing pending.

## Before you fix it
1. Which tests already pass, and why?
2. In the partial-fill test, how many lots does the starting code lose, and whose are they?

## Answer in the PR
1. Walk through the starting code on `a_partial_fill_requeues_the_remainder`. Who is still owed lots, and what does the house's position show against what clients were told?
2. How did you split a short fill across orders, and why oldest first rather than pro-rata? What does pro-rata cost with whole lots?
3. Why does the remainder go to the front of the pair's queue, not the back?
4. Why reject the overflow in `submit` and not in `flush`? What would `flush` do if a sum overflowed halfway through a batch?
5. What does batching cost the client (time until the flush), and what does it buy the house?

## Think further
1. Netting buys against sells per pair, so only the net goes to the LP.
2. An LP that reports more than it was asked for: a trust boundary, so cap it or reject it?
3. A provider call that fails, so `fill` returns `Result`. What happens to the batch?
4. A cancel that arrives for a partly filled order.
5. An LP minimum order size (kata 48): a batch below the minimum waits.
6. Flushing on a timer versus on size, and duplicate order ids.

## In the wild
- Brokers and exchanges that aggregate client flow and hedge it with LPs on an interval.
- FIX `ExecutionReport` `CumQty` / `LeavesQty` for partial fills.
- IOC orders at an LP that come back partly filled.
- Block-order allocation across client accounts.

In a review, check:
- A provider call inside a per-order loop.
- The returned filled qty ignored.
- The queue cleared after a flush.
- Zero-qty fills or calls.
- An unchecked sum.
