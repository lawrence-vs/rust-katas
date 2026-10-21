# 38 intermediate: rc-refcell-borrow-panic

**Concept:** A shared mutable graph needs `Rc<RefCell<_>>` to compile, but `RefCell` enforces borrowing at runtime—two overlapping mutable borrows panic. Scope the borrow (drop the guard) before the recursive call to avoid the panic.
**Source:** Programming Rust, 2nd ed. (Blandy, Orendorff, Tindall), ch. 4 Structs and ch. 9 Interior Mutability, https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch04.html, https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch09.html
**Fails with:** `thread 'main' panicked at src/lib.rs:XX: already borrowed: BorrowMutError`
**Vault:** your note on Rc and RefCell

## Task
Make `cargo test` pass without changing the test. Use only `std` (`Rc`, `RefCell`). No `unsafe`, no `Weak`, no crates. The fix is to narrow the `borrow_mut()` / `borrow()` scope so the guard is dropped before recursing.

## Before you fix it
1. Run `cargo test`. Which test panics, and at which line in `src/lib.rs`?
2. The panic says `already borrowed`. Which two borrows overlap?
3. Predict the fix in one sentence: where does the first borrow end?

## Answer in the PR
1. Why does a shared mutable graph need both `Rc` and `RefCell`? What does each one provide?
2. In `increment_all`, why does the mutable borrow from `self.value.borrow_mut()` stay alive across the recursive call to `edge.increment_all()`?
3. How does adding a block `{ *self.value.borrow_mut() += 1; }` change the borrow lifetime? What trait method on the guard runs at the end of the block?
4. The test `cycle_detected_by_runtime_borrow_check` creates a cycle. Why doesn't it deadlock? What would happen if you used `Mutex` instead of `RefCell`?
5. When would you reach for `Weak` instead of `Rc` in a graph like this?

## Think further
1. `RefCell::try_borrow_mut` returns a `Result` instead of panicking. When would you use it instead of a scoped block?
2. A `RefCell` inside an `Rc` can't be made `Sync`. What would you wrap it in for cross-thread sharing, and which trait would the wrapper implement?
3. The graph holds `Vec<Rc<Node>>`. If you added a `parent: RefCell<Option<Weak<Node>>>` field, what cycle would that create, and how does `Weak` break it?
4. In an exchange order book, multiple price levels point to the same instrument. Why is `RefCell` (not `Mutex`) the right choice for single-threaded order matching?

## In the wild
- std [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) + [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html): "Interior Mutability"
- [`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html) breaks reference cycles so `Rc` can drop
- petgraph crate: graph algorithms with `NodeIndex` / `EdgeIndex` instead of direct `Rc` pointers
- Exchange-style: single-threaded matching engine where many order nodes share an instrument's `RefCell<InstrumentState>`; borrow scoping prevents panics during cascade updates
- Tokio's `task-local` storage uses `RefCell` under the hood; scoped borrows matter there too

In a review, check:
- `borrow_mut()` guard held across a function call (especially recursion)
- `.borrow()` / `.borrow_mut()` without a limiting block
- `Rc<RefCell<_>>` where `Arc<Mutex<_>>` is needed for threads
- Missing `Weak` in parent pointers causing memory leaks