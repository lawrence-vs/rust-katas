# 96 advanced: a poison message, dead-lettered

**Concept:** A message that always fails gets a bounded number of attempts, counted per message. Then it goes to a dead-letter topic and is committed, so the partition moves on.
**Source:** Apache Kafka documentation, consumer offsets and commits and error handling ([docs](https://kafka.apache.org/documentation/)); rdkafka docs for the shape of a consumer ([docs.rs/rdkafka](https://docs.rs/rdkafka)); kata 62 for the trait-over-broker style. No Readwise highlight was available to the builder.
**Fails with:** `test a_poison_message_is_dead_lettered_after_max_attempts ... FAILED`
**Vault:** your note on poison messages

The starting `Consumer` reads one partition through a `Broker` trait. When the handler fails, it sleeps, does not commit, and returns `Retrying`. The next `step` polls the same head message again. The tests use fake brokers and handlers, and paused tokio time, so they run in milliseconds.

## Task
Make `cargo test` pass without changing the tests or any public signature. No `unwrap`, `expect` or `panic!` in `src/`. Do not add a `run()` loop: the tests drive `step` one call at a time.

## Before you fix it
1. Which tests already pass, and why?
2. What does the broker return on the poll after a failure?

## Answer in the PR
1. Why does an uncommitted failure block every message behind it?
2. Why publish to the dead-letter topic before committing, and what does the other order lose?
3. Why reset the attempt count per message?
4. What duplicate can this design still produce, and who has to tolerate it?
5. Why is there no backoff before dead-lettering?

## Think further
1. Retryable and non-retryable errors: which ones should skip the retries and dead-letter at once?
2. Exponential backoff with jitter, and what happens when many consumers back off in step.
3. A retry topic with a delay, so retries do not block the partition at all.
4. Where the attempt count lives across a restart, and what a restart does to it.
5. Replaying the dead-letter topic, and alerting on its depth.
6. Many partitions, and one slow key (kata 73).

## In the wild
- Kafka Connect `errors.tolerance` and `errors.deadletterqueue.topic.name`.
- Spring Kafka `DeadLetterPublishingRecoverer`.
- SQS redrive policy with `maxReceiveCount`.
- RabbitMQ dead-letter exchanges.

In a review, check:
- A retry loop with no limit.
- An attempt counter shared across messages.
- A commit before the dead-letter publish.
- A dead letter that drops the error.
- A sleep before giving up.
