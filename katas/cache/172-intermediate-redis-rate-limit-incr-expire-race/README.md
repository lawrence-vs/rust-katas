# 172 cache (intermediate): a rate limiter whose counter loses its TTL

**Concept:** INCR then EXPIRE, sent as two commands, can leave a counter with no TTL, and that user is blocked forever. A fixed window that restarts when its key expires lets about 2× the limit through at the boundary. Set the TTL in the same step as the first INCR, and count requests against a sliding window.
**Source:** Redis docs, INCR, "Pattern: Rate limiter" (https://redis.io/docs/latest/commands/incr/) and PEXPIRE (https://redis.io/docs/latest/commands/pexpire/); kata 171. No Readwise highlight was available to the builder.
**Fails with:** `a rate-limit counter was left with no TTL`
**Vault:** your note on rate limiting and atomic Redis commands

`Store` has one method per Redis command, and each method is atomic. `MemoryStore` is the fake: its clock moves only when `advance` is called, so a crash or a wait is a call to `advance`. The tests are plain `#[test]` functions, with no threads and no real clock.

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Don't change the tests, `Cargo.toml`, any public signature or `MemoryStore`'s semantics. No `unwrap`, `expect` or `panic!` in `src/`, and no new crates.

The limiter must follow these rules:
- Windows are aligned to `time / window`, so every user shares the same window edges.
- The previous window counts in proportion to how much of it the last `window` still covers.
- A refused request is not counted.
- Each counter key gets its TTL in the same step as its first INCR.

## Before you fix it
1. Run `cargo test`. Which five tests fail, and which two bugs do they point at?
2. In `a_lost_expire_…`, write down the key's value and its PTTL after the 11th check.

## Answer in the PR
1. Where between the two commands can a crash land, and what does the user see afterwards?
2. Why is `incr_px` atomic when `incr` plus `pexpire` isn't, and what are two real Redis ways to get that one step (a script; MULTI/EXEC with `PEXPIRE … NX`; `SET key 0 NX PX` then INCR)?
3. In the boundary test, how many requests got through in 0.1s with the old code, and why?
4. Your GET-GET-INCR decision is still three commands. What can two app servers do between them, and what would close that gap?
5. Why must a refused request not count?

## Think further
1. The real commands: `INCR rl:user-1:12345`, `PEXPIRE key 120000`, `PEXPIRE key ms NX` (Redis 7.0+), `MULTI`/`EXEC`, `EVAL`/`EVALSHA`, and `TIME`. Which `Store` method is each?
2. The sliding-window log with `ZADD`, `ZREMRANGEBYSCORE` and `ZCARD`: exact, at the cost of memory per request.
3. The `redis` crate's `redis::Script` (check `invoke` on docs.rs; it needs the `script` feature). Drop anything you can't confirm.
4. Clock source: Redis `TIME` vs each app server's clock.
5. See the sd-katas card on rate limiters.

## In the wild
- The Redis INCR rate-limiter pattern
- The `governor` crate (GCRA)
- The `redis-cell` module
- `tower::limit::RateLimit`

In a review, check:
- INCR and EXPIRE sent as separate commands
- EXPIRE only when `count == 1`, with no TTL on the counter otherwise
- a key with no window in its name
- refused requests counted against the next window
- the limiter's clock taken from each app server

## Related concepts

### Token bucket
Tokens refill at a steady rate up to a capacity; each request takes one. Use it when short bursts are fine but the average rate must hold.
```rust
fn take(&mut self, now: Duration) -> bool {
    let refill = now.saturating_sub(self.last).as_secs_f64() * self.rate;
    self.tokens = (self.tokens + refill).min(self.capacity);
    self.last = now;
    if self.tokens >= 1.0 { self.tokens -= 1.0; true } else { false }
}
```

### Leaky bucket / GCRA
Each request moves a theoretical arrival time forward by one interval; it is refused when that time runs too far ahead of now. Use it to smooth traffic to a fixed rate with no counter reset.
```rust
let tat = self.tat.max(now) + self.interval;
if tat.saturating_sub(now) > self.burst { return false; }
self.tat = tat; true
```

### Sliding-window log
A timestamp is kept for each accepted request; old ones are dropped and the rest are counted. Use it when the limit must be exact and the request rate is low.
```rust
let cutoff = now.saturating_sub(window);
while self.log.front().is_some_and(|&t| t <= cutoff) { self.log.pop_front(); }
if self.log.len() < max { self.log.push_back(now); true } else { false }
```
