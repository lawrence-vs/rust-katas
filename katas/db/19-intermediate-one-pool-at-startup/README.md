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
