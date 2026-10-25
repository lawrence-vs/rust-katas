# 173 cache (advanced): a slow reader writes a stale row back after the delete

**Concept:** In cache-aside, a reader that loaded a row before a write can put it back after the writer's DEL, and with no TTL it stays forever. A cached row carries its version, a set into the cache only lands if it is newer, and every entry has a TTL to bound what slips through.
**Source:** Facebook, "Scaling Memcache at Facebook" (NSDI 2013, section on leases and stale sets); Redis docs for SET (`PX`) (https://redis.io/docs/latest/commands/set/) and EVAL (https://redis.io/docs/latest/commands/eval/); katas 95 and 172. No Readwise highlight was available to the builder.
**Fails with:** `a stale row stuck in the cache after a write`
**Vault:** your note on cache-aside and stale reads

`Db` has one method per statement and `Cache` has one method per Redis command, and each method is atomic. `lookup` plus `fill` is a read split in two, so a test can pause a reader between the database load and the cache write. The tests are plain `#[test]` functions, with no threads and no real clock: time moves only when a test calls `advance`.

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Don't change the tests, `Cargo.toml`, any public signature or the semantics of `MemoryDb` and `MemoryCache`. No `unwrap`, `expect` or `panic!` in `src/`, and no new crates.

The cache must follow these rules:
- Once `write` returns, no read returns an older version for that key.
- Every cache entry has a TTL of `CACHE_TTL`.
- A missing row is never cached.
- A hit never touches the database.

## Before you fix it
1. Run `cargo test`. Which three tests fail, and which rule does each one break?
2. In `a_slow_reader_…`, write down what the cache holds after `lookup`, after `write` and after `fill`.

## Answer in the PR
1. Write the interleaving as numbered steps. Which step makes the stale row stick, and for how long with the old code?
2. Why does a version check on the fill alone not help while the writer still deletes the key?
3. Why must the compare and the set be one script, not `GET` then `SET`?
4. Your fix still lets a stale row in if the entry expires between the write and the slow fill. Give the steps. What bounds the damage, and what would close the gap?
5. What does a shorter or longer `CACHE_TTL` trade off?

## Think further
1. The real commands: `GET`, `SET key val PX 60000`, `DEL`, and `EVAL`/`EVALSHA` with a script that compares a stored version before `SET … PX`. Which `Cache` method is each? Check each command on redis.io at build time.
2. Memcache leases from the Facebook paper: a lease token is issued on a miss and checked on the fill.
3. Delayed double delete, and why it only narrows the window.
4. Invalidation driven by change-data-capture from the database log.
5. See the sd-katas card on caching (write-through, write-back, write-around).

## In the wild
- "Scaling Memcache at Facebook" leases
- memcached `add` and `cas` (check them in the memcached protocol docs)
- Redis `WATCH`/`MULTI`/`EXEC`
- Microsoft Azure Architecture Center's "Cache-Aside pattern" page

In a review, check:
- a `DEL` after the DB write with an unconditional `SET` on the read path
- cache entries with no TTL
- a cached value with no version
- `GET` then `SET` sent as two commands to "compare"

## Related concepts

### Write-through
Every write goes to the cache and the database together. Use it when reads must see writes at once.
```rust
db.insert(k.clone(), v.clone());
cache.insert(k, v);
```

### Write-back
Writes land in the cache and are flushed to the database later. Use it for write-heavy data that can stand a loss window.
```rust
cache.insert(k.clone(), v);
dirty.insert(k);
// later: for k in dirty.drain() { db.insert(k.clone(), cache[&k].clone()); }
```

### Write-around
Writes go only to the database, and the cache fills on a read. Use it when written data is rarely read back.
```rust
db.insert(k.clone(), v);
// the next read misses and loads from db
```
