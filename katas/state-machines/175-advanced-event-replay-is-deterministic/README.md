# 175 state-machines (advanced): replaying the log gives a different state

**Concept:** State is a fold over the event log. `apply` may read only the state and the event, never the clock or a random source. It skips a version it has already seen and refuses a gap, so the same log, or a snapshot plus its tail, always gives the same bytes.
**Source:** Martin Fowler, "Event Sourcing" (martinfowler.com, 2005); std docs for `std::hash::RandomState` and `std::time::SystemTime`; katas 50 and 62.
**Fails with:** `two replays of one log disagree`
**Vault:** your note on event sourcing

## Task
Make `cargo test` pass by changing only `Stock::apply` (and removing whatever becomes unused) in `src/lib.rs`. Don't change the tests, `Cargo.toml`, any public signature, or `encode`, `decode`, `replay` and `replay_from`. No `unwrap`, `expect` or `panic!` in `src/`, and no new crates.

The state must follow these rules:
- Two replays of one log give the same bytes.
- A snapshot at version N plus the events after N gives the same bytes as a full replay.
- An event with a version the state already has is a duplicate: no change, `Ok(Outcome::Duplicate)`.
- An event that skips a version is an error: no change, `Err(ReplayError::Gap { .. })`.

## Before you fix it
1. Run `cargo test`. Which four tests pass, and why do those four not depend on `apply`'s clock or lot?
2. Replay `log()` twice on paper. Which fields can differ between the two results, and where does each one get its value?

## Answer in the PR
1. What must be true of `apply` for a snapshot plus the tail to equal a full replay?
2. Who chose the time and the lot in your fix, and when?
3. Why is a duplicate a no-op but a gap an error? Why not buffer the gap, as kata 62 does?
4. The log holds a ship that would go below zero. Is that `apply`'s job or the command's, and what happens to old logs when a rule changes?
5. Why compare encoded bytes and not just `==`?

## Think further
1. Two different events with the same version: should `Duplicate` compare the payload? What does it mean if it differs?
2. Upcasting a v1 `Received` with no `at_ms`: what value does it get, and why not "now"?
3. A new field in `Stock` breaks old 32-byte snapshots: add a format byte, or throw snapshots away and replay?
4. Real stores: a Postgres `events` table with `PRIMARY KEY (stream_id, version)`, and EventStoreDB/KurrentDB append with an expected revision. Check each against its docs at build time; drop what you can't confirm.
5. A golden-file test of the encoded state in CI.
6. See the sd-katas card on event sourcing (name only, no link).

## In the wild
- Fowler's "Event Sourcing" article
- the Microsoft Azure Architecture Center "Event Sourcing pattern" page
- Akka Persistence event handlers, which should have no side effects (check the docs)
- KurrentDB / EventStoreDB

In a review, check:
- `now()` or a random call inside `apply` or `evolve`
- `apply` with no version check
- a snapshot without its version
- unchecked `+`

## Related concepts

### Upcasting
Converting an old event shape to the current one on read, so old logs still replay. Use it when an event schema changes.
```rust
fn upcast(old: ReceivedV1) -> Received {
    Received { qty: old.qty, unit: Unit::Each }
}
```

### CQRS read model
A separate table or view built by folding the events, read by queries and never written by commands. Use it when reads need a different shape from the log.
```rust
for e in new_events { read_model.on(e); }
let low = read_model.skus_below(5);
```

### Expected version on append
A write states the version it expects the stream to be at, and the store refuses it if the stream has moved on. Use it to stop two writers from appending over each other.
```rust
if store.version(stream) != expected { return Err(Conflict); }
store.append(stream, events);
```
