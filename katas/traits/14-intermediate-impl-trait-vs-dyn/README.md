# 14 intermediate: `impl Trait` vs `dyn Trait`

**Concept:** `impl Trait` in return position is one concrete type the function picks, so two branches returning different types don't compile. When the type is chosen at runtime, return `Box<dyn Trait>`: dynamic dispatch, one allocation.
**Source:** All Rust Features Explained, polymorphism, https://readwise.io/open/841529902; The Rust Book ch. 18.2, https://doc.rust-lang.org/book/ch18-02-trait-objects.html
**Fails with:** `E0308`
**Vault:** your note on static vs dynamic dispatch

## Task
Make `cargo test` pass without changing the test or the `Shape` trait.

## Before you fix it
1. Read E0308. What two types does it name, and why must they match?
2. The test stores shapes in `Vec<Box<dyn Shape>>`. What does that tell you about the return type?

## Answer in the PR
1. What does `impl Shape` promise the caller, and why couldn't this function keep that promise?
2. What does `Box<dyn Shape>` cost per call and per value?
3. An `enum AnyShape { Circle(..), Square(..) }` would avoid the Box. When would you pick it?

## Think further
1. `parse_shape("triangle 3")` and `parse_shape("circle abc")` both return a shape. What should a parser return for input it doesn't understand, and how would that change the test?
2. Someone adds `fn scaled<T: Into<f64>>(&self, by: T) -> Self` to `Shape`. Does `Box<dyn Shape>` still compile? Read the reference's rules for dyn compatibility, then decide what the method should look like.
3. The shapes move into a `tokio::spawn` task (kata 17). What does `Box<dyn Shape>` not promise that the task needs, and where would you write that promise?

## In the wild
- [Rust Reference, dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility): the rules for which traits can be behind `dyn`. This used to be called object safety.
- [std `Error`](https://doc.rust-lang.org/std/error/trait.Error.html): see the `From` impls into `Box<dyn Error + Send + Sync>`. That is why `?` can return any error from a `main` that returns `Box<dyn Error>`.
- [axum `response`](https://docs.rs/axum/latest/axum/response/index.html): handlers return `impl IntoResponse`. Read what axum says about returning different response types from different branches.
- [tower `BoxService`](https://docs.rs/tower/latest/tower/util/struct.BoxService.html): tower erases a service's concrete type with a box so it can be stored and named.
- [either `Either`](https://docs.rs/either/latest/either/enum.Either.html): a two-variant enum that implements `Iterator` and other traits when both sides do. This is the enum alternative to a box.
- Exchange-style: an order arrives as limit, market or stop, and the type is known only at runtime. Matching runs on every order, so a `match` on an enum is often preferred to a vtable call. Plugin-style hooks (risk checks loaded from config) usually stay `Box<dyn …>`.

In a review, check:
- `Box<dyn Trait>` over a closed set of types that the crate owns. Would an enum do?
- `impl Trait` in a public return type that callers need to store in a struct field. They can't name the type.
- Trait objects shared across tasks or threads without `+ Send` (and `+ Sync`).
- A new generic method on a trait that is used as `dyn`. That change breaks callers.
