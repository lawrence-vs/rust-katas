# 61 advanced: a transactional outbox

**Concept:** One transaction writes the state change, a dedupe row for the command id and an outbox row. A separate publisher drains the outbox and marks a row only after the sink accepts it. A crash loses nothing, a redelivery changes nothing, and consumers dedupe by id because delivery is at-least-once.
**Source:** sqlx `Pool::begin` / `Transaction` docs, https://docs.rs/sqlx/0.8.6/sqlx/struct.Transaction.html; microservices.io "Transactional outbox" and "Idempotent consumer"; kata 50; own design (exchange order events)
**Fails with:** `a crash then a redelivery applies the fill once`
**Vault:** your note on dual writes / the outbox

## Task
Make `cargo test` pass without changing the tests, `Sink`, `CrashPoint`, `crash_once` or `crash_if`. No `unwrap`/`expect`/`panic!` in `src/`. No `query!` macros. No new crates.

## Before you fix it
1. Which two tests already pass, and why?
2. After the crash in test 4, what is in `orders`, and what did the sink see?

## Answer in the PR
1. Walk the starting code through a crash after the UPDATE, then a redelivery: rows, events, and who would notice.
2. Why must the dedupe row commit in the same transaction as the fill? What breaks if it's written first on its own, or after?
3. The publisher crashes after the sink accepted the event. What does the consumer receive, what must it do, and why can't the outbox give exactly-once?
4. Why is a duplicate `Ok(Duplicate)` and not an error, and why does an unknown order leave no dedupe row?
5. The pool has one connection. What happens if a query inside your transaction uses `&self.pool`?

## Think further
- Polling versus CDC (log tailing): what does each cost, and what does each guarantee about order?
- Pruning published rows and the dedupe table: how long must you remember a command id, and what does a redelivery after pruning do?
- Two publisher instances: how do they avoid claiming the same row? SQLite has no `SKIP LOCKED`.
- Per-order versus global ordering: which does the drain promise, and which do consumers need?
- The same `CommandId` arrives with a different qty (kata 50's `KeyReused`). Is it a duplicate?
- A real broker client is async. What changes about `Sink`, and about what you hold across the `.await`?

## In the wild
- [microservices.io, Transactional outbox](https://microservices.io/patterns/data/transactional-outbox.html) and [Idempotent consumer](https://microservices.io/patterns/communication-style/idempotent-consumer.html).
- [Debezium outbox event router](https://debezium.io/documentation/reference/transformations/outbox-event-router.html): CDC reads the outbox table instead of a polling publisher.
- [Kafka idempotent producer and transactions](https://kafka.apache.org/documentation/#producerconfigs_enable.idempotence): the broker-side version of the same problem.
- [SQS FIFO deduplication ids](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagededuplicationid-property.html): a dedupe window kept by the broker.
- Exchange-style: order fills feeding a downstream ledger; a lost fill is a balance error, a doubled fill is a worse one.

In a review, check:
- A DB write followed by a publish.
- Dedupe recorded only on success.
- `execute(&pool)` inside a transaction.
- A row marked sent before the publish.
- A handler's success depending on the broker.
