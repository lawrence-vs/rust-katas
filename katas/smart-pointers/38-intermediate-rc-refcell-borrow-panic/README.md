# 38 intermediate: a RefCell borrow held across recursion

**Concept:** Nodes shared by many edges need `Rc` (many owners) and `RefCell` (change through a shared reference). `RefCell` checks borrows at runtime: a `RefMut` lives until its scope ends, and a second `borrow_mut` on the same cell while it lives panics. A walk over a shared graph must drop its borrow before calling out, and remember where it has been.
**Source:** Programming Rust, 2nd ed. (Blandy, Orendorff, Tindall), ch. 4 Ownership and Moves, "Rc and Arc: Shared Ownership", https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch04.html; ch. 9 Structs, "Interior Mutability", https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch09.html
**Fails with:** `RefCell already borrowed`
**Vault:** your note on interior mutability

## Task
Make `cargo test` pass without changing the test. Change only `for_each_reachable` (add a private helper if you need one). No `unsafe`, no crates, no `Weak`, no `try_borrow*` to swallow the panic. Cloning a `Vec<Rc<Node>>` is allowed: it bumps reference counts and copies no node.

## Before you fix it
1. Run `cargo test`. Which tests panic, which assertion fails, and what does each panic message say?
2. In `a_visitor_can_add_edges` there is no cycle. Which two borrows of which `RefCell` overlap?
3. Predict the two changes in one sentence each.

## Answer in the PR
1. `connect` takes `&Rc<Node>`. Why can't `edges` be a plain `Vec<Rc<Node>>`, and why can `value` be a `Cell` when `edges` can't?
2. In the starting code, when is the `RefMut` in `edges` dropped? Name the calls that ran while it was alive.
3. Changing `borrow_mut()` to `borrow()` stops the cycle panic. Why, what happens to the cycle test instead, and why does the visitor test still panic?
4. Which fix did you pick for the held borrow (clone the list out, or scope the borrow), and what does the other option cost?
5. Why do a diamond and a cycle need the same fix? What does your visited record key on, and what would break if two nodes shared an id?

## Think further
1. The cycle test clears `b.edges` at the end. What leaks without that line, and how would a `Weak` back-edge (`Weak::upgrade` returning `Option<Rc<Node>>`) change who owns whom?
2. Why does `Node` not derive `Debug`? What would `{:?}` do on a cycle?
3. When would `try_borrow_mut` be the right call instead of making the conflict impossible?
4. The same graph shared across threads: what replaces `Rc`, `Cell` and `RefCell`, and what does a held lock across the recursion do instead of panicking?

## In the wild
- std [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) ("Panics if the value is currently borrowed") and [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) / [`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html) (cycles and back-pointers).
- GUI and DOM trees (e.g. `Rc<RefCell<Node>>` in event handlers) hit the same panic when a callback re-enters the node that is calling it.
- [petgraph](https://crates.io/crates/petgraph) avoids the question with indices into a `Vec` instead of `Rc` pointers.
- Exchange-style: a single-threaded risk graph of accounts and sub-accounts, walked to sum exposure; a hook that links a new sub-account during the walk must not meet a held borrow.

In a review, check:
- A `borrow()` or `borrow_mut()` guard bound with `let` and kept alive across a callback, a recursive call or an `.await`.
- A recursive walk over `Rc` links with no visited set.
- `Rc` cycles with no `Weak` and no explicit break, i.e. leaks.
