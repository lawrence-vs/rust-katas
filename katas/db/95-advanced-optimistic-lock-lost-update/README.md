# 95 advanced: an optimistic lock, no lost update

**Concept:** Read-modify-write over separate statements loses an update when two writers read the same row; a `version` column in the `WHERE` makes the write a compare-and-set, and `rows_affected() == 1` tells you whether you won.
**Source:** sqlx `Query::execute` and `SqliteQueryResult::rows_affected` docs, https://docs.rs/sqlx/0.8.6/sqlx/sqlite/struct.SqliteQueryResult.html; Designing Data-Intensive Applications (Kleppmann), ch. 7, "Preventing Lost Updates"
**Fails with:** `a_stale_save_is_a_conflict`
**Vault:** your note on lost updates

## Task
Make `cargo test` pass without changing the tests, the signatures, `setup`, `open_account` or `load`. No `unwrap`, `expect` or `panic!` in `src/`. Keep `max_connections(1)`. Don't reach for `BEGIN EXCLUSIVE` or a table lock as the fix.

## Before you fix it
1. Walk the first test by hand: two `load`s, then two `save`s. What does the row hold after each step, and what should the second `save` have been told?
2. What does `execute` return, and what is in it besides "it worked"?

## Answer in the PR
1. The interleaving that loses an update, step by step.
2. Retry or return the conflict: which did you pick, and what does the other cost? (Think latency and livelock under contention versus pushing the decision to the caller.)
3. Why bound the retries, and how did you choose the bound?
4. Why a `version` column and not "the balance is still what I read"? (Think ABA.)

## Think further
1. `SET balance = balance + ?` in one statement can't lose an update. When is that enough, and why does the first test still need a version?
2. Pessimistic locking (`SELECT … FOR UPDATE`, `BEGIN IMMEDIATE`) stops the conflict instead of detecting it. What does it cost, and when is it the better choice?
3. Postgres `REPEATABLE READ` and `SERIALIZABLE` fail a transaction with a serialization error. How is that the same problem as your retry?
4. `rows_affected() == 0` can mean the row was deleted or that it went stale. Does your code tell them apart? Should it?
5. `a.balance + amount` can overflow. What should `deposit` do about it?
6. Retrying a write from an HTTP handler that isn't idempotent: what can go wrong? (See kata 50.)

## In the wild
- [sqlx `Transaction`](https://docs.rs/sqlx/0.8.6/sqlx/struct.Transaction.html): the tool for pessimistic locking and for multi-statement units of work.
- [Postgres transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html): what each level does about concurrent updates, and when you must retry.
- Designing Data-Intensive Applications, ch. 7: lost updates, write skew, and the options for each.
- HTTP conditional requests: `ETag` with `If-Match` is the same compare-and-set, exposed to API clients (`412 Precondition Failed` is your `Conflict`).
- ORM optimistic-locking columns, e.g. JPA `@Version`: the same `version` column, managed for you.

In a review, check:
- An `UPDATE` whose result (`rows_affected`) is ignored.
- A `load` then `save` with neither a version check nor a transaction.
- A retry loop with no bound.
