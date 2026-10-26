# 191 intermediate: a redelivered event applies once

**Concept:** Brokers deliver at least once, so a consumer sees duplicates after a retry or a rebalance. Making the handler idempotent means recording which event ids were applied and checking that record before changing state. The check and the write must agree.
**Source:** Apache Kafka documentation, delivery semantics (at least once), https://kafka.apache.org/documentation/#semantics; own design (exchange account ledger consumer)
**Fails with:** `redelivered_event_applies_once`
**Vault:** your note on idempotent consumers

## Task
Make `cargo test` pass without changing the test. Keep `apply` taking `&Event`. The fix lives in `Ledger`.

## Before you fix it
1. Run the test. What balance did the second delivery produce?
2. What does the ledger know about event `7` after the first call? Is that enough to recognise the repeat?

## Answer in the PR
1. What did you record per event, and why an id and not the amount?
2. Your set of seen ids grows without bound. What would you bound it by, and what breaks when an old duplicate arrives after that bound?

## Related concepts
- **Idempotency key in a database:** a unique constraint on the event id, written in the same transaction as the balance. Kata 174 covers it.
- **Replay determinism:** re-running the same log gives the same state. Kata 175 covers it.

```rust
let mut seen = std::collections::BTreeSet::new();
seen.insert(7_u64);
```
