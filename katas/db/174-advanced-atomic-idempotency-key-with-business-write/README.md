# 174 db (advanced): an idempotency key written apart from the payment it guards

**Concept:** The idempotency key row, the debit and the payment row commit in one transaction. A primary key on `key` lets the database refuse a second row, and a retry returns the stored receipt. Split them over several commits and a crash between them either strands the request or pays it twice.
**Source:** sqlx 0.8.6 `Transaction` docs (https://docs.rs/sqlx/0.8.6/sqlx/struct.Transaction.html); SQLite docs for `PRIMARY KEY` / `UNIQUE` constraints and `RETURNING`; Stripe API, "Idempotent requests"; Brandur Leach, "Implementing Stripe-like Idempotency Keys in Postgres"; the IETF draft "The Idempotency-Key HTTP Header Field"; katas 61 and 95. No Readwise highlight was available to the builder.
**Fails with:** `a crash then a retry pays exactly once`
**Vault:** your note on idempotency keys and atomic writes

`setup` makes an in-memory SQLite database. `crash_after(n)` makes `pay` fail at its nth `after_statement` call, the way a process dies between two lines. Each `execute(&self.pool)` is its own commit, so a crash between two of them leaves the earlier writes behind.

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Don't change the tests, `Cargo.toml`, any public signature, `crash_after`, `after_statement`, `open_account`, `deposit` or `snapshot`, the `accounts` and `payments` tables, or the column names `key`, `payment_id` and `balance_after`. Call `self.after_statement()?` after every write statement in `pay` and after the commit. No `unwrap`, `expect` or `panic!` in `src/`, no `query!` macros, and no new crates.

## Before you fix it
1. Run `cargo test`. Which three tests fail, and which three pass and why?
2. For a crash after statement 1 and after statement 2, write down the rows in each table and what the retry does.

## Answer in the PR
1. Walk the starting code through a crash after the debit, then a retry: rows, receipts, and what the customer sees.
2. Why can't the key row commit on its own, either first or last? What does each order lose or double?
3. A crash after `COMMIT`: what did the client see, what does its retry return, and why must the retry return the stored receipt rather than build a new one?
4. With two connections, two requests with the same key both run the `SELECT` and find nothing. What stops the second one, and what should it tell its client?
5. Why does a declined payment leave no key row? When would you store a failure instead?

## Think further
Check each item against the docs at build time.
1. Postgres: `INSERT … ON CONFLICT (key) DO NOTHING RETURNING …`, and turning a unique violation into a replay or a `409 Conflict`.
2. Same key, different body: storing a request fingerprint and rejecting a mismatch (the IETF draft suggests `422`).
3. Key expiry: how long you keep keys (Stripe documents 24 hours), and what a retry after pruning does.
4. A side effect outside the database, such as calling a card provider: Brandur's recovery points and atomic phases.
5. `BEGIN IMMEDIATE` in SQLite versus deferred transactions, and isolation levels in Postgres.
6. See the sd-katas card on 2PC vs sagas.

## In the wild
- Stripe `Idempotency-Key` header
- the IETF Idempotency-Key header draft
- Brandur's post on idempotency keys in Postgres
- AWS Lambda Powertools idempotency utility

In a review, check:
- a key insert on `&pool` before the business writes
- a key row with a nullable result that a retry treats as "run again"
- no `UNIQUE` or `PRIMARY KEY` on the key column
- a retry that builds a fresh response instead of returning the stored one

## Related concepts

### Savepoints
A nested rollback point inside a transaction. Use one to undo part of a transaction and keep the rest.
```sql
SAVEPOINT step;
UPDATE accounts SET balance = balance - 5 WHERE id = 1;
ROLLBACK TO step;
COMMIT;
```

### Isolation levels and write skew
How much one transaction sees of another's writes. Under `SERIALIZABLE`, one of two transactions that read the same rows and write different ones fails and must retry.
```sql
BEGIN ISOLATION LEVEL SERIALIZABLE;
-- reads and writes, then:
COMMIT; -- may fail with SQLSTATE 40001
```

### 2PC vs sagas
Atomic commit across services, versus local steps with compensations. Use a saga when no single transaction spans the systems.
```rust
for step in steps {
    step.run().await.inspect_err(|_| undo(&completed))?; // undo runs compensate() in reverse
    completed.push(step);
}
```
