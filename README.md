# rust-katas

Small Rust bugs to fix, one concept each, from what I'm reading. Every kata **fails on purpose** until it's fixed: it doesn't compile, or its test fails.

## How a kata works

```
katas/<topic>/<NN>-<level>-<slug>/
  README.md      concept, source, task, questions to answer
  src/lib.rs     the bug
  tests/kata.rs  passes only when the bug is fixed
```

1. Read the kata's README and run `cargo test` inside it. Read the compiler's error before touching code.
2. Fix it on a branch, answer the README's questions in the PR description in your own words.
3. Open a PR. CI runs `cargo fmt --check`, `cargo clippy -D warnings` and `cargo test` on only the katas the PR changes; Junior then reviews it against [REVIEW.md](REVIEW.md) (idioms, ownership, [API Guidelines](https://rust-lang.github.io/api-guidelines/)).
4. Write what you learned as a permanent note in the vault and link the kata from its `code:` property.

Levels: `beginner` (one rule), `intermediate` (two rules interacting), `advanced` (design: which fix is right and why).

Katas are made with the `code-kata` skill from a concept, a Readwise highlight or a Claude session; `katas/_template/` is the shape.

## Tracks

| Topic | Kata | Source |
|---|---|---|
| ownership | [01 beginner: mutable variable vs mutable reference](katas/ownership/01-beginner-mutable-variable-vs-reference) | Rust for Rustaceans, ch. 1 |
| ownership | [02 intermediate: moving out of a Vec](katas/ownership/02-intermediate-move-out-of-vec) | Rust for Rustaceans, ch. 1 |
| lifetimes | [03 advanced: a struct outliving its reference](katas/lifetimes/03-advanced-struct-outlives-reference) | Rust for Rustaceans, ch. 1 |
