# 218 advanced: a lost reply must not be retried

**Concept:** an order already written to the wire may be executed, so a failure after the write must not look retryable; tag each error by the phase it happened in.
**Source:** kraken-cli `crates/kraken-ws/src/connection.rs:812-828`, `kraken-core/src/error.rs:127-132`, https://github.com/krakenfx/kraken-cli (MIT)
**Fails with:** `a lost reply must not be retried`
**Vault:** your note on at-least-once side effects

## Task
Make `cargo test` pass without changing the tests. Keep both `OrderError` variants and `is_retryable`. Tag the error by the phase it came from; do not match on message text.

## Related concepts

### `std::error::Error::source`
Links an error to the lower-level error that caused it. Use it so a caller can walk the chain without parsing text.
```rust
let mut cur: Option<&dyn std::error::Error> = Some(&err);
while let Some(e) = cur {
    println!("{e}");
    cur = e.source();
}
```

### `thiserror` `#[error(transparent)]`
Forwards `Display` and `source` to a wrapped error. Use it for a variant that adds no message of its own.
```rust
#[derive(thiserror::Error, Debug)]
enum E {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

### `#[non_exhaustive]` enum
Lets a library add variants later without a breaking change. Callers must keep a wildcard arm.
```rust
#[non_exhaustive]
pub enum E { A, B }
// downstream: match e { E::A => {}, _ => {} }
```

## Before you fix it
1. Run the tests. In which two places can `place_order` return an error, and how does the order's state differ between them?
2. What does the caller's retry loop do with `is_retryable() == true`?

## Answer in the PR
1. Why does the phase, not the message text, decide whether to retry?
2. What must the caller do on `ReplyLost`?
3. Why would idempotency keys change the answer?

## Think further
1. Add a timeout on `read_reply`. Is a reply that arrives late the same case as a lost one?
2. After `ReplyLost`, what query would the caller send, and what if that query also fails?

## In the wild
- [`std::result::Result`](https://doc.rust-lang.org/std/result/enum.Result.html) and [the `?` operator](https://doc.rust-lang.org/std/ops/trait.Try.html): where a function exits early is a phase boundary; the error type is the place to record which one.
- [Stripe idempotent requests](https://docs.stripe.com/api/idempotent_requests): a key lets a client resend a write safely, because the server replays the first result.
