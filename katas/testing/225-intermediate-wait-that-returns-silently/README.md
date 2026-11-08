# 225 intermediate: a wait that gives up without saying so

**Concept:** A test helper that polls for a condition must fail loudly when the condition never holds. If it returns silently after the last try, the test fails later, somewhere unrelated, with a confusing assert.
**Source:** Kraken's `wait_until` (MIT; `mock_transport.rs`) polls every 2 ms for 500 tries and then panics. Rust docs: [`panic!`](https://doc.rust-lang.org/std/macro.panic.html) and [`#[should_panic]`](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html).
**Fails with:** `test did not panic as expected` (test `panics_when_the_condition_never_holds`)
**Vault:** your note on test helpers that poll: they fail at the helper, with a message naming the condition.

## Task
Keep the signature of `wait_until`. When it runs out of tries, make it fail loudly with the message `condition never met`. Do not change `tests/kata.rs`, and do not change the 2 ms sleep.

## Related concepts
### `assert!`
Panics with a message when a bool is false. Use it for invariants inside tests and helpers.
```rust
let ready = false;
assert!(ready, "not ready yet");
```

### `Result<(), E>`
Reports failure to the caller instead of panicking. Use it in library code where the caller decides.
```rust
fn check(ok: bool) -> Result<(), String> {
    if ok { Ok(()) } else { Err("bad".into()) }
}
```

### `std::sync::mpsc::Receiver::recv_timeout`
Blocks until a message arrives or a deadline passes, returning an error on timeout. Use it instead of polling a channel.
```rust
use std::time::Duration;
let (_tx, rx) = std::sync::mpsc::channel::<u8>();
let r = rx.recv_timeout(Duration::from_millis(5));
```

## Before you fix it
1. Run `cargo test`. Which test fails, and what does the harness say about it?
2. The second test passes already. Why does the bug not show there?

## Answer in the PR
1. Why is a silent return worse than a failure at the call site?
2. What does `#[should_panic(expected = "...")]` check, and what does it not check?
3. Why sleep 2 ms in a loop instead of using a timeout?

## Think further
1. Tries times 2 ms is the total wait. What is it for 500 tries, and when would you rather pass a `Duration`?
2. Should the panic message include the number of tries? What would a failing CI log need?

## In the wild
- [`std::macro.panic`](https://doc.rust-lang.org/std/macro.panic.html): the macro the fix uses.
- [Rust by Example, unit testing](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html): `#[should_panic]` and `expected`.
