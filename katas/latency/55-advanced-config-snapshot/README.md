# 55 advanced: a config snapshot, not a read per quote

**Concept:** A quote reads an in-memory snapshot, not the database. The config is loaded once and swapped in one step on change. A quote holds its own `Arc` to the version it priced with, and a failed reload keeps the last good config.
**Source:** Rust Atomics and Locks (Mara Bos), ch. 1 "Basics of Rust Concurrency" (Arc, interior mutability, RwLock), https://marabos.nl/atomics/basics.html; std `Arc`, `RwLock` and `PoisonError::into_inner` docs; own design (exchange quoting); katas 15, 18 and 41.
**Fails with:** `a quote read the config store`
**Vault:** your note on read-mostly shared state

## Why the rule exists
Quoting is the hot path and config changes rarely. A database round trip per quote adds network latency and its jitter to every price. It also ties quoting uptime to database uptime, and can price two legs of one request with two different configs.

## Task
Make `cargo test` pass without changing the tests. Keep the public API, except that `QuoteError::Store` and its `From` impl may be removed. std only. The store is read only in `new` and `reload`. No `.unwrap()`, `.expect(` or `panic!` in `src/`; `unwrap_or_else(PoisonError::into_inner)` is allowed.

## Before you fix it
1. Which tests pass, and why?
2. Count the store round trips for 1 000 quotes.

## Answer in the PR
1. How many round trips did 1 000 quotes cost before your fix, and after? What else did each one put in the quote path (latency, jitter, a failure mode)?
2. Why `Arc<QuoteConfig>` behind the lock? Compare it with holding the read lock for the whole quote, and with cloning `QuoteConfig` on every quote. Name a concrete cost for each.
3. Why does `reload` call the store before it takes the write lock? What would readers wait on otherwise? (kata 18)
4. A quote is priced while `reload` swaps the config. Which version does it use, and when is the old config freed?
5. Why doesn't `RefCell` or `Cell` compile here, and what do you do about a poisoned lock?
6. Which error could `quote` return before your fix that it can't now? Was removing it right?

## Think further
1. `arc-swap` (lock-free `load`/`store` of an `Arc`), and what it saves over `RwLock`.
2. Two reloads racing so that an older version overwrites a newer one: should `reload` compare versions?
3. Push (change notification) or poll, and how stale a snapshot may get.
4. Measuring the difference with a benchmark instead of a load counter.
5. Per-instrument config in one snapshot map, or one `Arc` per instrument.

## In the wild
- Exchange and broker quoting engines that keep fee tiers, spreads and limits in memory and reload on a change event.
- Feature-flag and risk-limit caches with a last-known-good fallback.
- Config hot reload in proxies and load balancers.

In a review, check:
- A database or HTTP call inside a per-request hot path.
- A lock held across I/O.
- A reload that blanks the config on failure.
- A cache with no way to reload it.
