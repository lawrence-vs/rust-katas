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
