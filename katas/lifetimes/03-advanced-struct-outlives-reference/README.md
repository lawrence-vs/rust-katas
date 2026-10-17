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
