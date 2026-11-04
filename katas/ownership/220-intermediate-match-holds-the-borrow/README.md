# 220 intermediate: a match arm that holds the borrow

**Concept:** A `match` on a value that borrows `self` keeps that borrow alive through every arm that uses the binding. Calling a `&mut self` method inside such an arm is rejected; finish with the borrowed value first, then call the method.
**Source:** kraken-cli crates/kraken-ws/src/connection.rs:479-482, https://github.com/krakenfx/kraken-cli
**Fails with:** `E0499`
**Vault:** your note on two-phase borrows and match scrutinees

## Task
Make `cargo test` pass. Keep `ack_front`'s behaviour and the method names, and don't change the tests. Don't clone the `Pending`, and don't use `RefCell` or `unsafe`.

## Related concepts

### `Option::take`
Moves the value out of an `Option` and leaves `None`. Use it to hand a pending item to one consumer exactly once.
```rust
let mut slot = Some(3);
let got = slot.take();
assert_eq!((got, slot), (Some(3), None));
```

### `std::mem::take`
Swaps a `Default` value into a place and returns the old one. Use it to drain a buffer behind `&mut`.
```rust
let mut buf = vec![1, 2];
let old = std::mem::take(&mut buf);
assert!(buf.is_empty() && old.len() == 2);
```

### `Vec::get_mut`
Returns `Option<&mut T>` for an index instead of panicking. Use it when the index may be out of range.
```rust
let mut v = vec![1, 2];
if let Some(x) = v.get_mut(1) { *x += 10; }
```

## Before you fix it
1. Which two borrows does the compiler name in the error?
2. Which line keeps the first one alive?

## Answer in the PR
1. Why does a `match` scrutinee keep a borrow alive for the arm body?
2. Why is `self.record()` fine before the `match` but not inside it?
3. What does an NLL (non-lexical lifetimes) borrow end at?

## Think further
1. If `record` only needed `&self`, would the code compile? Which error would you get instead, and why?
2. What if `ack_front` also had to ack every later entry? Which borrow shape would you reach for?

## In the wild
- [The Rust book: references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html): the one-`&mut`-at-a-time rule behind E0499.
- [RFC 2094: non-lexical lifetimes](https://rust-lang.github.io/rfcs/2094-nll.html): a borrow lasts until its last use, not the end of the scope.
