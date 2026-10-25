# NN level: title

**Concept:** one sentence.
**Source:** book, chapter, highlight link (`https://readwise.io/open/<id>`) or Claude session note.
**Fails with:** `E0000` (the error code, or the failing test's name, CI checks for it).
**Vault:** the permanent note this kata backs, e.g. `Assignment Moves The Value`.

## Task
What to make pass, and the one thing not to do (e.g. no `.clone()` unless this README names it).

## Related concepts
Sibling features a solver may reach for instead. At most 4. For each: one sentence saying what it is, its main use case, and a snippet of at most 5 lines. Write `None: no close siblings.` if there are none. Snippets are illustrative and must not give away this kata's fix. Example for a `RefCell` kata:

### `Cell<T>`
Interior mutability for `Copy` values, with no borrow tracking. Use it for counters shared behind `&self`.
```rust
let c = std::cell::Cell::new(1);
c.set(c.get() + 1);
```

## Before you fix it
1. Read the compiler error. Which rule does it name?
2. Predict the fix in one sentence.

## Answer in the PR
1. Why was it a bug, in terms of variables, values and pointers?
2. Which fix did you pick, and what does the other option cost?
