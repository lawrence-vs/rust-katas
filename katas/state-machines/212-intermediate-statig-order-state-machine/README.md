# 212 intermediate: a statig order state machine

**Concept:** A state machine crate only saves you from invalid transitions if each state says which events it refuses. `Handled` keeps the current state; `Transition` moves it. A `Cancel` that falls through to a `Transition` in a state that should be final moves the order, and no compiler error says so. Write the state's answer for every event it can get.
**Source:** statig 0.4.1 README, https://github.com/mdeloof/statig (`Outcome`, `Transition`, `Handled`, `Super`); the `state_machine` and `state` macros in statig 0.4.1 (`tests/`).
**Fails with:** `shipped_order_refuses_cancel`
**Vault:** `~Rust Crate Triage 2026-10-29` (crate versions and why the API changed)

## Task
Make `cargo test` pass without changing the tests. In `src/lib.rs`, a shipped order must refuse `Cancel` (return `Handled`, stay `Shipped`). Keep `statig`; the states and transitions in the other three arms stay as they are. No `unwrap`, `expect` or `panic!` in `src/`.

## Before you fix it
1. Run `cargo test`. Which test fails, and which state does it end in?
2. Predict the fix in one sentence.

## Answer in the PR
1. Why was it a bug? Say which event a `Shipped` order received and where the state moved.
2. Which fix did you pick, and what does the other option cost?

## Related concepts
### A plain enum and `match`
Same rules without a crate: a `match` on `(state, event)` with every arm named. Cheaper, no macros, but the transitions live in one function instead of per state.
```rust
match (state, event) {
    (Order::Paid, Event::Ship) => Order::Shipped,
    (s, _) => s,
}
```
### Typestate
Each state is its own type, so an illegal transition doesn't compile at all. Kata 12 shows when that is worth the extra types.
