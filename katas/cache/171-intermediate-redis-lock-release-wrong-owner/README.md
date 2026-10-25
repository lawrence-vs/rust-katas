# 171 cache (intermediate): a late release that frees someone else's lock

**Concept:** A Redis lock is SET key token NX PX ttl. The token must be new on every acquire, and release must delete the key only if it still holds that token, in one atomic step. Otherwise a holder whose TTL ran out frees the next holder's lock. Even then, a TTL lock alone can't stop a paused holder's late write; that needs a fencing token.
**Source:** Redis docs, "Distributed Locks with Redis" (https://redis.io/docs/latest/develop/clients/patterns/distributed-locks/) and DELEX (https://redis.io/docs/latest/commands/delex/); Martin Kleppmann, "How to do distributed locking" (2016-02-08); antirez, reply at http://antirez.com/news/101; katas 39 and 51. No Readwise highlight was available to the builder.
**Fails with:** `a late release freed another client's lock`
**Vault:** your note on leases, locks and fencing tokens

`Store` has one method per Redis command, and each method is atomic. `MemoryStore` is the fake: its clock moves only when `advance` is called, so a pause is `advance` past the TTL. The tests are plain `#[test]` functions, with no threads and no real clock.

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Don't change the tests, `Cargo.toml`, any public signature or `MemoryStore`'s semantics. No `unwrap`, `expect` or `panic!` in `src/`, and no new crates.

## Before you fix it
1. Run `cargo test`. Which three tests fail, and which two bugs do they point at?
2. In `a_late_release_…`, write down who holds `lock:report` just before `release(a)`, and what `DEL` does to it.

## Answer in the PR
1. In `a_late_release_…`, whose key did `release` delete, and why can't a TTL alone prevent that?
2. Why must the token be new on every acquire rather than the owner's name? Name a real case where two acquires share an owner name.
3. Why isn't GET, compare in Rust, then DEL enough? Point at the moment between the two commands, using `release_checks_and_deletes_in_one_step`.
4. Your token comes from std only. What does it guarantee (uniqueness) and what doesn't it (unpredictability)? What would you use in production?
5. With a correct release, worker-a can still write after its lock expired. What does a fencing token add that the lock can't?

## Think further
1. The real commands: `SET lock:report <token> NX PX 10000` (replies `OK` or nil), `PTTL lock:report`, `DELEX lock:report IFEQ <token>` (Redis 8.4+), and before 8.4 the Lua script on the Redis docs page, run with `EVAL`/`EVALSHA`. Which `Store` method is each? Don't paste the script.
2. With the `redis` crate: `redis::Script::new(src)`, then `script.key(key).arg(token)` and invoke it with a connection. Check `ScriptInvocation`'s `invoke` on docs.rs for the current version (the `script` feature is needed), and drop anything you can't confirm.
3. Kleppmann vs antirez on Redlock: process pauses, network delay and clock jumps. The Redis page itself now says to implement fencing tokens, and that Redis TTLs don't use a monotonic clock. Which side's assumptions hold in your system?
4. Leases vs locks: Gray & Cheriton, "Leases: An Efficient Fault-Tolerant Mechanism for Distributed File Cache Consistency" (SOSP 1989). In etcd (lease plus revision) and ZooKeeper (ephemeral node plus zxid), where does a monotonic fencing number come from?
5. Extending a lease while work runs: what must the extension check, and why does it need the same atomicity as release?
6. See the sd-katas card on distributed locks and fencing tokens.

## In the wild
- The Redis distributed-locks page
- Kleppmann's post on distributed locking
- antirez's reply
- The Gray & Cheriton leases paper
- Redsync (Go), Redisson (Java), rslock (Rust)

In a review, check:
- a lock value that is a constant or the owner's name
- release by plain `DEL`
- `GET` then `DEL` from the client
- no fencing on the protected write
- a TTL shorter than the worst-case work

## Related concepts

### A fencing token
A number that rises on every acquire and goes with every write; the resource rejects a write whose number is lower than the last it saw. Use it when a paused holder may still write.
```rust
fn write(&mut self, fence: u64, data: &str) -> Result<(), Stale> {
    if fence < self.last_fence { return Err(Stale); }
    self.last_fence = fence;
    self.data = data.to_string(); Ok(())
}
```

### Redlock
The same key and value set on N independent Redis masters; the lock counts only if a majority granted it within the validity time. Use it so one Redis failure doesn't lose the lock; its safety is debated.
```rust
let granted = nodes.iter_mut().filter(|n| n.set_nx_px(key, &value, ttl)).count();
let held = granted > nodes.len() / 2 && started.elapsed() < ttl;
```

### std::sync::Mutex
A lock within one process, freed when its guard drops, with no TTL. Use it when every contender is a thread in the same process.
```rust
let counter = std::sync::Mutex::new(0);
if let Ok(mut n) = counter.lock() { *n += 1; }
```
