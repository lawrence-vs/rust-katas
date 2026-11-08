# 230 intermediate: a snapshot is the contract

**Concept:** An insta snapshot is a reviewed contract stored in a `.snap` file; a failing snapshot means the code changed the contract, so fix the code, and update the `.snap` only when the change is intended and reviewed.
**Source:** [insta quickstart](https://insta.rs/docs/quickstart/), [insta snapshot types](https://insta.rs/docs/snapshot-types/); kraken-cli `crates/kraken-workspace/tests/snapshots.rs` (MIT) uses the same pattern
**Fails with:** `snapshot assertion for 'receipt_matches_the_contract' failed`
**Vault:** your note on golden-file (snapshot) tests

## Task
Make `cargo test` pass by fixing `src/lib.rs`. Do not edit or delete `tests/kata.rs` or anything in `tests/snapshots/`, and do not run `cargo insta accept` or set `INSTA_UPDATE`. Keep the public API.

## Related concepts
### `assert_eq!`
Compares a value to an expected value written in the test. Use it when the expectation is one small value a reader can see inline.
```rust
assert_eq!(2 + 2, 4);
```

### Inline snapshots
`assert_snapshot!` with the expected text inside the source file instead of a `.snap` file. Use it for short output you want reviewed next to the test.
```rust
insta::assert_snapshot!(value, @"expected text");
```

### `cargo insta review`
The interactive tool that accepts or rejects pending `.snap.new` files. Use it to approve a change you intended, never to silence one you did not.
```sh
cargo insta review
```

## Before you fix it
1. Read the diff insta prints. Which line differs, and which side is the contract?
2. Is the code wrong or the snapshot wrong? What would you check to decide?

## Answer in the PR
1. What does a failing snapshot tell you that a failing `assert_eq!` does not?
2. When is updating a snapshot correct?
3. Why store the snapshot in the repo rather than regenerate it each run?

## Think further
1. A reviewer sees a PR that changes a `.snap` file and no source. What do they ask?
2. Where does a snapshot test stop being useful, such as timestamps or random ids, and what do you do then?

## In the wild
- [insta](https://insta.rs/) snapshots in the [kraken-cli](https://github.com/krakenfx/kraken-cli) workspace tests.
- [Jest snapshot testing](https://jestjs.io/docs/snapshot-testing) uses the same reviewed-file contract in JavaScript.
