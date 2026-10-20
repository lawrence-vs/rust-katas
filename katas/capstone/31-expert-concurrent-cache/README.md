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
