# 226 intermediate: a fake that clones its own record

**Concept:** A test double owned by the code under test must share its record with the test. If the test keeps a `Clone` of the fake and the record is a plain `Vec`, the clone copies the vec and the test sees nothing. Share the record behind a handle whose clone points at the same data.
**Source:** Kraken's `MockTransport` (`mock_transport.rs:28-35`, MIT) derives `Clone` over `Arc<Mutex<Vec<String>>>`, so the test keeps one handle to inspect while the actor owns the other. Docs: [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html), [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html).
**Fails with:** `sent frames not visible`
**Vault:** your note on test doubles (fakes, mocks, spies) and what they record

## Task
Make `cargo test` pass without changing the tests. Keep the `send` and `sent` signatures and keep `Clone`: the test side and the actor side must see one record. Don't use a global static.

## Related concepts
### `Rc<RefCell<T>>`
Single-threaded shared mutable state, with runtime borrow checks. Use it when nothing crosses threads.
```rust
let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u8>::new()));
let other = log.clone();
other.borrow_mut().push(1);
```

### `mpsc::channel`
Moves values from one owner to another instead of sharing a record. Use it when the consumer drains events.
```rust
let (tx, rx) = std::sync::mpsc::channel();
tx.send("frame").unwrap();
assert_eq!(rx.recv().unwrap(), "frame");
```

## Before you fix it
1. Read the failing assertion. Which of the two handles received "subscribe"?
2. Predict the fix in one sentence.

## Answer in the PR
1. What does `Clone` copy for a `Vec`, and what does it copy for an `Arc`?
2. Why must the test and the actor share one record?
3. What does `Arc<Mutex<..>>` cost compared with the plain `Vec`?

## Think further
1. `sent()` returns a `Vec<String>` copy: why is that better than returning a lock guard to the test?
2. What happens to the record if the actor panics while holding the lock, and how does `lock().unwrap()` react?

## In the wild
- [std `Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html): `Arc::clone` bumps a reference count; it does not copy the data.
- [std `Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html): what "clone" promises, and that derived `Clone` clones every field.
