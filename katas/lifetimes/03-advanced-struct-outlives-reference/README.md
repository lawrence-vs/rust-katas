# 03 advanced: a struct outliving its reference

**Concept:** a reference is only valid while the variable it points at is alive. A lifetime on a struct is the compiler proving the struct can't outlive what it borrows.
**Source:** Rust for Rustaceans, ch. 1 (lifetimes are about places, not values).
**Vault:** your note on lifetimes.

## Task
Make `cargo test` pass. Two designs fix it; the question is which one you'd ship.

## Before you fix it
1. Read "`temp` does not live long enough". Where exactly is `temp`'s place destroyed?
2. Draw it: which variable holds the data, and which one holds the address?

## Answer in the PR
1. Fix A: move `temp` out of the inner block. Fix B: make `Connection` own a `String`. Which do you pick for a connection that lives for the whole program, and why?
2. When is a struct holding `&'a str` the better design? Name a real case.

## Think further
1. Change `reconnect` to return the `Connection` itself instead of a `String`. With your fix, what must the return type say, and where does the URL's data have to live?
2. Use `db` inside a `std::thread::spawn` closure. What does the compiler require of the URL? What changes with [`std::thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html)?
3. Sometimes the URL is a literal and sometimes it's built at runtime. What would a `Cow<'a, str>` field buy, and what would it cost?

## In the wild
- [`std::str::Lines`](https://doc.rust-lang.org/std/str/struct.Lines.html): an iterator that borrows the string it walks, so it can't outlive it.
- [`std::sync::MutexGuard`](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html): the guard borrows its `Mutex`. Its lifetime is what stops a guard outliving the lock.
- [`tokio::net::tcp::ReadHalf`](https://docs.rs/tokio/1.54.0/tokio/net/tcp/struct.ReadHalf.html) and [`OwnedReadHalf`](https://docs.rs/tokio/1.54.0/tokio/net/tcp/struct.OwnedReadHalf.html): tokio ships a borrowed and an owned version of the same half. Read both signatures and note what each asks of the caller.
- [`std::borrow::Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html): one type that can hold either a borrowed or an owned value.
- Exchange-style: a decoded market-data message whose fields point into the socket's receive buffer, which gets reused for the next frame.

In a review, check:
- A struct with a lifetime parameter that is stored somewhere long-lived (app state, a `static`, a spawned task): who owns the data it points at, and does that owner live as long?
- `'static` bounds or `Box::leak` added only to silence a lifetime error (REVIEW.md 2.4).
- A function that returns a borrow of a local (E0515, E0597).
