# 13 beginner: a missing trait bound

**Concept:** A generic `T` can do nothing until a bound says what it can do. `>` is `PartialOrd::gt`, so comparing needs `T: PartialOrd`; floats are `PartialOrd` but not `Ord`.
**Source:** All Rust Features Explained, generics bound by traits, https://readwise.io/open/841530162
**Fails with:** `E0369`
**Vault:** your note on trait bounds

## Task
Make `cargo test` pass without changing the test.

## Before you fix it
1. Read E0369. Which trait provides `>`?
2. The test calls `largest` on `f64`. Does `f64` implement `Ord`? Why not?

## Answer in the PR
1. Why does a generic function need the bound, when every type in the test can be compared?
2. `PartialOrd` vs `Ord`: which did you pick, and what does NaN have to do with it?
