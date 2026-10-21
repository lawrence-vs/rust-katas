# 12 advanced: typestate: consume self

**Concept:** If a method takes `self` by value and returns a different type, the old state can't be used again. Encode a connection's state in its type and calling `query` on a closed connection stops compiling.
**Source:** The Typestate Pattern in Rust (Cliff L. Biffle), https://cliffle.com/blog/rust-typestate/; API Guidelines checklist https://rust-lang.github.io/api-guidelines/checklist.html
**Fails with:** `E0432`
**Vault:** your note on typestate

## Task
Make `cargo test` pass without changing the test or the `compile_fail` doc test on `Connection` (it is part of the contract: `cargo test` runs it).

## Before you fix it
1. Why does the `compile_fail` doc test compile today?
2. If `connect` takes `self` and returns `Connection<Open>`, what happens to the old binding?

## Answer in the PR
1. Where did the runtime check go, and what does the compiler now prove?
2. Why `PhantomData<State>` and zero-sized state types, and what do they cost at runtime?
3. When is typestate the wrong tool? (think: state that comes from the network)

## Think further
1. A real `connect` can fail. What should it return, and what happens to the closed connection the caller passed in when it fails? Can they retry with it?
2. An open connection goes out of scope without `close`. What runs? What can the type system force here, and what can't it? (Rust's types promise "used at most once", not "used exactly once".)
3. Add transactions: `begin` and `commit`. Which methods should a connection inside a transaction have, and what should `close` do in the middle of one?

## In the wild
- [`tokio::net::TcpSocket::connect`](https://docs.rs/tokio/1.54.0/tokio/net/struct.TcpSocket.html#method.connect): `connect` and `listen` take the socket by value and return a `TcpStream` or a `TcpListener`. What does the caller still hold after a failed `connect`?
- [`http::request::Builder::body`](https://docs.rs/http/1.5.0/http/request/struct.Builder.html#method.body): the builder is consumed and a `Request` comes out. axum and hyper both build on `http`.
- [The Embedded Rust Book, typestate programming](https://docs.rust-embedded.org/book/static-guarantees/typestate-programming.html): GPIO pins whose mode is a type, so writing to an input pin doesn't compile.
- Exchange-style: a trading client whose private calls (place order, cancel, balances) exist only after login. Calling `place_order` before logging in becomes a compile error instead of a 401 from the venue.

In a review, check:
- `&mut self` methods that flip a state field, paired with methods that `assert!` or return `Err` on that field.
- Methods available in every state that only make sense in one.
- A failed transition: does the caller get the old value back, or is it lost?
