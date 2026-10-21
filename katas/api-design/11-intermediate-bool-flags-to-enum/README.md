# 11 intermediate: bool flags to an enum

**Concept:** Three bools make eight combinations; an order has four real states. An enum makes the impossible combinations unrepresentable, and transitions become a match on one value.
**Source:** Rust API Guidelines, C-CUSTOM-TYPE (arguments convey meaning through types, not bool), https://rust-lang.github.io/api-guidelines/type-safety.html
**Fails with:** `E0432`
**Vault:** your note on making invalid states unrepresentable

## Task
Make `cargo test` pass without changing the test. No bool fields left on `Order`.

## Before you fix it
1. List the eight bool combinations. Which four are real?
2. Which transitions are allowed from each real state?

## Answer in the PR
1. Which impossible states did the enum remove?
2. Why does `state()` return a copy instead of exposing the field?
3. How would kata 12's typestate change this API, and what would it cost a caller that stores orders in a Vec?

## Think further
1. The business adds refunds: a shipped order can be refunded. List every place in `src/` that has to change. Which ones does the compiler point you to, and which do you have to find by reading?
2. A client retries `cancel` after a timeout, but the first call had already worked. Should the second call return `Err(InvalidTransition)` or `Ok(())`? What does each choice mean for a client on a flaky network?
3. A shipped order needs a tracking number, and a cancelled one needs a reason. Where does that data live, so that a cancelled order can't have a tracking number? What does that change about returning the state as a copy?

## In the wild
- [API Guidelines, C-CUSTOM-TYPE](https://rust-lang.github.io/api-guidelines/type-safety.html#c-custom-type): `Widget::new(Small, Round)` versus `Widget::new(true, false)`. The same argument holds for struct fields.
- [`std::task::Poll`](https://doc.rust-lang.org/std/task/enum.Poll.html): every future reports `Ready(value)` or `Pending`, not a `ready: bool` next to an `Option`. What would the bool-and-option version allow?
- [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html): the other side. Its bools are mostly independent, but read the Errors section of `open`: some combinations are only rejected at runtime.
- Clippy's [`struct_excessive_bools`](https://rust-lang.github.io/rust-clippy/master/index.html#struct_excessive_bools) (pedantic, off by default): what threshold does it use, and would it have caught `Order`?
- [FIX `OrdStatus` (tag 39)](https://www.onixs.biz/fix-dictionary/4.4/tagNum_39.html): an exchange sends an order's status as one field with one value from a fixed list, not as a set of flags.
- Exchange-style: an order book keeps `is_cancelled` and `filled_qty` side by side. A cancel that races a fill can leave a "cancelled" order fully filled, and nothing in the type says which came first.

In a review, check:
- Two or more bools on one struct that describe one lifecycle.
- Transition methods that set a flag without checking the others (`self.x = true; Ok(())`).
- A status enum plus a bool that repeats part of it, so the two can drift apart.
