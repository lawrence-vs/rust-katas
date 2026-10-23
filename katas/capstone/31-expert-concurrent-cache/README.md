# 31 expert: capstone: a concurrent cache

**Concept:** Everything at once: a cache shared by many tasks must stay bounded (security), never hold a lock over I/O (async), and load each key once even under a stampede (single-flight with `tokio::sync::OnceCell`).
**Source:** rust-workshop-labs (staged labs with observations), https://github.com/SarthakMakhija/rust-workshop-labs; tokio `OnceCell` docs, https://docs.rs/tokio/1.53.2/tokio/sync/struct.OnceCell.html
**Fails with:** `stage1_capacity_is_bounded`
**Vault:** your MOC note linking the track notes

## Task
Make the three stages pass in order without changing the test. Keep `get_or_load`'s signature. Time is paused in the tests.
## Stages

Work in order. After each stage, write down your **Observation** (what the test showed), a **Question** it raised, and the **Insight** you'd keep. Put all three in the PR.

1. `stage1_capacity_is_bounded`: a cache that never forgets is a memory leak with a hit rate.
2. `stage2_distinct_keys_load_concurrently`: a lock is for the map, not for the database.
3. `stage3_same_key_loads_once`: ten callers, one load.

## Before you fix it
1. Run `cargo test stage1`. What should leave the cache when a third key arrives in a cache of 2?
2. Stage 3 passes today. Why, and why will your stage 2 fix break it?

## Answer in the PR
1. Stage 1: which eviction did you choose, and what does it cost per insert?
2. Stage 2: what does the lock protect now, and for how long?
3. Stage 3: how does `OnceCell` make ten callers share one load? What happens if the load panics?
4. Which earlier kata does each stage come from?

## Think further
1. `load` can fail: say it returns `Result<V, DbError>`. Should a failed load be cached? With your stage 3 design, what do the nine waiting callers see when the first caller's load returns `Err`, and which `OnceCell` method changes that?
2. The first caller's task is aborted (a `tokio::time::timeout` around `get_or_load`, or `JoinHandle::abort`) while its load is sleeping. What happens to the other nine, and how many times does `load` run in the end?
3. Capacity is 2 and a third key arrives while a load for one of the first two is still running. Does an in-flight load count toward capacity? If it is evicted, what do its waiters get, and what stops the next caller from starting a second load for it?
4. The values go stale (a fee tier, an instrument's tick size). How would you add a time-to-live, and how would you test it with paused time (`tokio::time::advance`)? At the moment a key expires, how many callers reload it?

## In the wild
- tokio [`OnceCell::get_or_init`](https://docs.rs/tokio/1.53.2/tokio/sync/struct.OnceCell.html#method.get_or_init): "If some other task is currently working on initializing the `OnceCell`, this call will wait for that other task to finish". [`get_or_try_init`](https://docs.rs/tokio/1.53.2/tokio/sync/struct.OnceCell.html#method.get_or_try_init) is the version for a load that can fail.
- tokio [`Mutex`, "Which kind of mutex should you use?"](https://docs.rs/tokio/1.53.2/tokio/sync/struct.Mutex.html#which-kind-of-mutex-should-you-use) and the tutorial's [Shared state](https://tokio.rs/tokio/tutorial/shared-state) chapter: a lock that only guards data should be held for a few instructions, not across I/O (katas 16 and 18).
- [moka `future::Cache::get_with`](https://docs.rs/moka/0.12.16/moka/future/struct.Cache.html#method.get_with): a production async cache that combines a size bound with coalescing concurrent loads of one key. It is the library version of this kata.
- Go's [`singleflight`](https://pkg.go.dev/golang.org/x/sync/singleflight) names the same idea: "a duplicate function call suppression mechanism".
- Exchange-style: gateways cache instrument metadata such as tradable asset pairs (tick size, lot decimals) and per-account fee tiers in front of a database. After a deploy or at market open, every connection asks for the same few keys at once.

In a review, check:
- A `lock().await` guard still alive across a database, HTTP or `load().await` call.
- A cache keyed by client input with no size bound or TTL (kata 21): anyone can grow it until the process runs out of memory.
- A check-then-load-then-insert sequence with no single-flight, which loads N times when N callers miss together.
- Errors cached forever, or a cancelled or panicking load that leaves waiters hanging.
