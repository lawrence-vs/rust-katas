# 73 advanced: shard by key, keep per-key order

**Concept:** Route each event to a worker by its key over a bounded channel. One key's events stay on one worker in order, different keys run in parallel, and a slow worker pushes back on the producer.
**Source:** tokio [`mpsc::channel`](https://docs.rs/tokio/latest/tokio/sync/mpsc/fn.channel.html) docs (bounded channels, backpressure); std [`mpsc::sync_channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html) and [`thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html) docs; Rust Atomics and Locks (Mara Bos), ch. 1
**Fails with:** `interleaved accounts keep per-account order`
**Vault:** your note on sharding by key

## Task
Make `cargo test` pass without changing the tests. No `unwrap`, `expect` or `panic!` in `src/`. std only. Don't change the signatures, and don't make it single-threaded.

## Before you fix it
1. Run `cargo test`. Which three tests fail, and which one fails first on the order of an account's `applied` list?
2. Read the `/// CLUE:` lines in `src/lib.rs`.

## Answer in the PR
1. Walk the starting code through the interleaved test for account 1. Which worker saw seq 3's withdraw, and what balance did it see?
2. Why can no merge of two partial ledgers for one account be right?
3. Which two properties (one receiver per channel, FIFO per channel) make `account % N` keep order? What breaks if N changes mid-stream?
4. What does `capacity` bound now? What happens to every other account when one account is hot (head-of-line blocking)?
5. Walk a worker panic through your code. Why is there no hang and no panic out of `thread::scope`?

## Think further
1. Hot keys and skew: one account is 90% of the stream. What does sharding buy you now?
2. Resharding and consistent hashing: how do you change N without breaking per-key order?
3. The same design with tokio: one `spawn` per shard and `mpsc::channel(capacity)`. What changes about `send` and about a dead worker?
4. An event touches two keys, such as a transfer. Which worker owns it?
5. Blocking on `send` versus `try_send` and dropping (kata 21): which is right for an untrusted producer?
6. Batching per shard: what does it cost in latency and gain in throughput?

## In the wild
- Partitioned log brokers, where a partition key gives per-key order.
- Actor-per-entity frameworks: one mailbox per entity.
- Matching engines with one thread per instrument.
- Databases sharded by user id.

In a review, check for:
- A pool fed by position (`i % n`) instead of by key.
- A `Mutex<HashMap>` shared by unordered workers.
- Unbounded channels fed by an untrusted producer.
- An early `?` that skips joining the workers.
