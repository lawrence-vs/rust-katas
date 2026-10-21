# 19 intermediate: one pool, made at startup

**Concept:** A pool is built once and passed as `&SqlitePool`; every query borrows a connection from it. Connecting per call pays a handshake each time and, with in-memory SQLite, even sees a different empty database.
**Source:** sqlx `SqlitePool` docs, https://docs.rs/sqlx/0.8.6/sqlx/type.SqlitePool.html; All Rust Features Explained, database connections and resource leaks, https://readwise.io/open/841527687
**Fails with:** `users_persist_across_calls`
**Vault:** your note on connection pools

## Task
Make `cargo test` pass without changing the test or `setup`.

## Before you fix it
1. Why does `count_users` see 0 rows after two inserts?
2. Why does `setup` set `max_connections(1)`? What happens to an in-memory database when its last connection closes?

## Answer in the PR
1. What did a connection per call cost, on a real Postgres server rather than in-memory SQLite?
2. Why `&SqlitePool` and not `SqlitePool` by value? (what is a pool clone?)
3. Where in an axum or tonic service is the pool created, and how does a handler reach it?

## Think further
1. `setup` allows one connection. If it allowed five, would every connection in the pool see the same in-memory database? Don't guess: find out how sqlx opens `sqlite::memory:` by reading `SqliteConnectOptions` in the sqlx-sqlite source.
2. The pool's one connection is checked out by a task that then awaits a slow HTTP call before giving it back. What does the next `add_user` do, and for how long? Which `PoolOptions` setting decides?
3. Two `add_user` calls must both succeed or both fail. The pool gives each call whichever connection is free. What do you need so both inserts run on the same connection?
4. `PoolOptions` has `idle_timeout` and `max_lifetime`. What could they do to an in-memory database that lives only as long as its connections?

## In the wild
- [sqlx `PoolOptions`](https://docs.rs/sqlx-core/0.8.6/sqlx_core/pool/struct.PoolOptions.html): `max_connections`, `min_connections`, `acquire_timeout`, `idle_timeout`, `max_lifetime`, `test_before_acquire`. Read each default; production pools are tuned here.
- [sqlx `PoolConnection`](https://docs.rs/sqlx-core/0.8.6/sqlx_core/pool/struct.PoolConnection.html): what you hold after `acquire()`, and when it goes back to the pool.
- [sqlx `Transaction`](https://docs.rs/sqlx/0.8.6/sqlx/struct.Transaction.html): a set of statements pinned to one connection until commit or rollback.
- [sqlx pool internals, `inner.rs`](https://github.com/transact-rs/sqlx/blob/v0.8.6/sqlx-core/src/pool/inner.rs): how a pool hands out connections, waits for one, and returns them.
- [PgBouncer features](https://www.pgbouncer.org/features.html): the pooler many shops put in front of Postgres. Session vs transaction pooling changes what a "connection" means to your app.
- Exchange-style: an order gateway runs N replicas, each with its own pool. N × `max_connections` must fit under the database server's connection limit, with room left for migrations and admin sessions.

In a review, check:
- `connect(` on a database URL anywhere other than startup code.
- A function that takes a pool but builds its own connection anyway.
- A pool built with defaults and no stated reason for its size or `acquire_timeout`.
- A `PoolConnection` or `Transaction` held across an unrelated `.await` (an HTTP call, a channel receive).
