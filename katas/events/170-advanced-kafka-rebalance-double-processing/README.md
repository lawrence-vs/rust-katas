# 170 events (advanced): a rebalance that applies a payment twice

**Concept:** A rebalance hands a partition to a new owner, which reads from the last committed offset. Anything the old owner processed but did not commit arrives again. Kafka gives at-least-once delivery, so the apply must be idempotent. A member beyond the partition count gets no partition and adds no parallelism.
**Source:** Apache Kafka documentation, design (message delivery semantics) and consumer groups and offset commits ([docs](https://kafka.apache.org/documentation/)); rdkafka docs for the shape of a real consumer ([docs.rs/rdkafka](https://docs.rs/rdkafka)); kata 96 for the trait-over-broker style. No Readwise highlight was available to the builder.
**Fails with:** `a redelivered record was applied twice`
**Vault:** your note on at-least-once delivery and idempotent consumers

`MemoryLog` is an in-process fake of the broker, behind the `Log` trait. `Group` is the consumer group: members, partition assignment, a read position per member, and commits. `Ledger` is the database the records are applied to. A crash is `leave` without `commit`. The tests are plain `#[test]` functions, with no threads and no clock.

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Don't change the tests, `Cargo.toml` or any public signature. No `unwrap`, `expect` or `panic!` in `src/`, and no new crates. `Group::poll` must not commit, and `MemoryLog` must keep its broker semantics.

## Before you fix it
1. Run `cargo test`. Which tests pass, which fail, and which two separate bugs do the failures point at?
2. In `a_rebalance_redelivers_…`, write down the committed offset and member 1's position just before `leave`. What does member 2 read first?

## Answer in the PR
1. Why does member 2 read offset 2 again, and why is that the broker working as designed rather than a broker bug?
2. Which dedupe key did you choose, `payment_id` or `(partition, offset)`? Name a duplicate that the other key would miss.
3. Why must the dedupe key be stored in the same place and step as the balance change, and what breaks if it is stored before an apply that fails?
4. Committing after every record shrinks the window. Why doesn't it close it?
5. Why can't a fourth consumer make a three-partition topic go faster, and what goes wrong if two members read one partition?

## Think further
1. Commit on revoke: rdkafka's `ConsumerContext::pre_rebalance` with `Rebalance::Revoke`, then `commit_consumer_state(CommitMode::Sync)`. It helps a clean leave. Does it help a crash, or a member kicked out for exceeding `max.poll.interval.ms`?
2. Eager versus `partition.assignment.strategy=cooperative-sticky` (KIP-429), and static membership with `group.instance.id` (KIP-345): which rebalances do they avoid?
3. `enable.idempotence=true` on the producer: which duplicate does it remove, and why doesn't it help here?
4. Transactions: `transactional.id`, `send_offsets_to_transaction` and `isolation.level=read_committed` give exactly-once for read-process-write inside Kafka. Why does that stop at an external database? See the sd-katas card on why exactly-once stops at an external database.
5. Storing the offset in the database in the same transaction as the effect, then seeking to it on assign: the alternative to a dedupe set. Also, how long must a dedupe set remember a key?
6. A real client configuration, as an rdkafka `ClientConfig` with `group.id`, `bootstrap.servers`, `enable.auto.commit=false` and `auto.offset.reset=earliest`. Then the command `kafka-consumer-groups.sh --bootstrap-server localhost:9092 --describe --group payments --members`: what does `#PARTITIONS` show for a spare member, and what is LAG?

## In the wild
- The Kafka docs on message delivery semantics
- KIP-429 (cooperative rebalancing)
- KIP-345 (static membership)
- KIP-98 (exactly-once)
- The microservices.io "Idempotent Consumer" pattern

In a review, check:
- an apply with no dedupe key
- a dedupe key stored apart from the effect
- an auto-commit before processing
- more consumers than partitions "for speed"

## Related concepts

### The idempotent producer
A producer setting that lets the broker drop a resend of the same produce request, so one send lands once in the log. Use it to stop duplicates on the way in, not on the way out of a consumer.
```rust
let producer: ThreadedProducer<_> = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("enable.idempotence", "true")
    .create()?;
```

### The transactional producer
A producer that writes records and consumer offsets in one atomic unit, so a read-process-write inside Kafka either fully happens or not at all. Use it when the output is another Kafka topic.
```rust
producer.init_transactions(Duration::from_secs(5))?;
producer.begin_transaction()?;
producer.send(record)?;
producer.send_offsets_to_transaction(&offsets, &group_meta, Duration::from_secs(5))?;
producer.commit_transaction(Duration::from_secs(5))?;
```

### A rebalance callback that commits on revoke
A `ConsumerContext` hook that runs before partitions are taken away, so the member can commit what it has finished. Use it for a clean leave; it cannot help after a crash.
```rust
fn pre_rebalance(&self, c: &BaseConsumer<Self>, r: &Rebalance) {
    if let Rebalance::Revoke(_) = r {
        if let Err(e) = c.commit_consumer_state(CommitMode::Sync) {
            eprintln!("commit on revoke failed: {e}");
        }
    }
}
```

### The cooperative-sticky assignor
A partition assignment strategy that moves only the partitions that must change in a rebalance, instead of stopping every member. Use it to cut the pause each rebalance causes.
```rust
let consumer: BaseConsumer = ClientConfig::new()
    .set("group.id", "payments")
    .set("partition.assignment.strategy", "cooperative-sticky")
    .create()?;
```
