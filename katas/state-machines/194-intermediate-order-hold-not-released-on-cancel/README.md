# 194 state-machines (intermediate): a hold not released on cancel

**Concept:** Reserve, then settle. A buy holds price × quantity. Each fill moves only its cost from held to spent. A cancel gives back only what the order still holds. A market buy's unused budget comes back when it runs out. Every cent is always available, held or spent.
**Source:** own design (exchange order holds, reserve-then-settle); std docs for `u64::checked_sub` and `checked_div`; katas 42, 51 and 53.
**Fails with:** `cancel released 0, want the unfilled 600`
**Vault:** your note on reserve-then-settle

## Task
Make `cargo test` pass by changing only `Account::fill` and `Account::cancel` in `src/lib.rs`. Don't change the tests, `Cargo.toml` or any public signature. No `unwrap`, `expect`, `panic!`, `wrapping_*` or `saturating_*` in `src/`. No new crates.

The four rules:
- A limit buy holds `price × qty`.
- A fill moves exactly its cost from held to spent.
- A cancel releases exactly what the order still holds.
- A market buy stops at its budget and releases the rest.

## Before you fix it
1. Run `cargo test`. Which 3 pass, and why do none of them fill an order?
2. On paper: 10 @ 100, fill 4, cancel. Write `available`, `reserved`, `spent` and the order's `held` after each step.

## Answer in the PR
1. After a partial fill, where was the unfilled money, and why did cancel not find it?
2. If cancel had released the full original hold, which account number would be wrong, and why did it not underflow in the test with order `b`?
3. Why does each order keep its own `held`, when the account already has `reserved`?
4. Why is the market buy's leftover released at fill time, not at cancel time?
5. Which `Status` transitions are legal? Draw them.

## Think further
1. Price improvement: a limit at 100 fills at 95. What happens to the 5 per unit?
2. Sells: what is held, cash or units?
3. Store `New` before an async risk check takes the hold. What can go wrong between the two?
4. Write a proptest over random place/fill/cancel sequences that checks conservation (kata 161).
5. Fees: who holds the fee, and when is it settled?

## In the wild
- Card authorization holds and partial capture.
- Stripe PaymentIntent `amount_capturable` and partial capture.
- Exchange "hold" or "locked" balances (Coinbase, Binance).

In a review, check for:
- a cancel that releases the original amount;
- a fill that lowers the account total but not the order's;
- a status arm that releases `0`;
- unchecked `-`.

## Related concepts

### Reservation token (kata 51)
A value that proves money was set aside, so it can be spent or returned exactly once. Use it when the reserve and the settle happen in different calls.
```rust
let r = line.reserve(300)?;
line.fill(r, 120)?;
```

### RAII hold guard
A guard whose `Drop` gives back a hold that was not explicitly settled. Use it to make a forgotten release impossible on early return.
```rust
struct Hold<'a> { acct: &'a mut Account, cents: u64 }
impl Drop for Hold<'_> {
    fn drop(&mut self) { /* give back */ }
}
```

### Double-entry ledger
Every change posts equal and opposite amounts to two accounts, so the total never moves. Use it when balances must be auditable.
```rust
ledger.post(&[("available", -300), ("held", 300)])?;
```

### Typestate (kata 12)
Encode the state in the type, so an open order and a filled order are different types. Use it when a method must only exist in one state.
```rust
fn fill(o: Order<Open>, q: u64) -> Either<Order<Open>, Order<Filled>>
```
