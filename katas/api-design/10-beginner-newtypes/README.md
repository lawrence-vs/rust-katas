# 10 beginner: newtypes, not primitives

**Concept:** Four `u64` arguments let a caller swap `from` and `amount` and still compile. A one-field newtype per meaning makes the compiler check it, and `#[repr(transparent)]` guarantees it costs nothing.
**Source:** Rust API Guidelines, C-NEWTYPE, https://rust-lang.github.io/api-guidelines/type-safety.html
**Fails with:** `E0432`
**Vault:** your note on primitive obsession

## Task
Make `cargo test` pass without changing the test. Use `#[repr(transparent)]` on both newtypes.

## Before you fix it
1. The test imports `AccountId` and `Cents`. What must each derive for the test to compile?
2. Swap `bob` and `Cents(300)` in your head. What would the old signature have done?

## Answer in the PR
1. What bug class did the newtypes remove, and when is it caught now?
2. What does `#[repr(transparent)]` promise that a plain one-field struct doesn't?
3. Should `Cents` implement `Add`? What would that let a caller do with an `AccountId`?

## Think further
1. Call `transfer` from an account that isn't in the map, and with an amount bigger than the sender's balance. What happens in each case in a debug build, and in `--release`? Which of these should the caller get back as a value? (Kata 42 covers the arithmetic.)
2. Transfer from `alice` to `alice`. Is the result what a bank would expect? Would you reject it, allow it, or make it impossible to write?
3. The test builds `AccountId(1)` directly, so the field is public. What could a caller do with a public `Cents` field that a private field and a constructor would stop? What would the test call instead?

## In the wild
- [The Reference, the `transparent` representation](https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation): the exact rules. What may sit next to the one field, and what does the layout guarantee?
- [`std::num::NonZero`](https://doc.rust-lang.org/std/num/struct.NonZero.html): a std newtype over an integer that also carries a rule (never zero). Read why `Option<NonZero<u64>>` is the same size as `u64`.
- [API Guidelines, C-NEWTYPE-HIDE](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-newtype-hide): a newtype also hides the type underneath, so that type can change later without breaking callers.
- serde's [`#[serde(transparent)]`](https://serde.rs/container-attrs.html#transparent) and sqlx's [`Type` with `#[sqlx(transparent)]`](https://docs.rs/sqlx/0.9.0/sqlx/trait.Type.html): a newtype that goes to JSON or a database column as the bare value. Which layer still sees a plain number?
- Exchange-style: an order message carries a price, a quantity, an order id and an account id, often all integers. `place(price, qty)` called as `place(qty, price)` still compiles, and sends a real order.

In a review, check:
- Public functions with two or more adjacent parameters of the same primitive type.
- `From<u64>` or `Deref<Target = u64>` on a newtype: does it let callers skip what the type means?
- Newtypes that the serialiser or database layer unwraps early, so the rest of the code passes the bare number around again.
