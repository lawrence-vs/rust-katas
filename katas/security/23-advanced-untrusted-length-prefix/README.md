# 23 advanced: an untrusted length prefix

**Concept:** A length read off the wire is an attacker's number. Check it against a limit before allocating, and never index past what you received: return an error instead of panicking.
**Source:** Network Programming With Rust (Chanda), parsing in networking applications, https://readwise.io/open/900600932
**Fails with:** `rejects_oversized_length`
**Vault:** your note on parsing untrusted input

## Task
Make `cargo test` pass without changing the test. No `unwrap`, `expect` or panicking index in `src/`.

## Before you fix it
1. A client sends `FF FF FF FF`. What does `Vec::with_capacity` try to do?
2. Which lines can panic on a short buffer?

## Answer in the PR
1. Name the two attacks the old code allowed.
2. Why check `MAX_FRAME` before allocating, not after?
3. Where should `MAX_FRAME` come from in a real service?

## Think further
1. A length of exactly `MAX_FRAME`: accepted or rejected? Do the tests pin it down, and should they?
2. `buf` holds one frame and the start of the next. What does the caller need back to find the next frame, and does the signature give it to them?
3. Off a socket, `Truncated` can mean "wait for more bytes" or "the peer hung up mid-frame". Which layer should tell the two apart?
4. On a 32-bit target `usize` is 32 bits. What is `4 + len` when `len` is `u32::MAX`? Does your version ever compute it?

## In the wild
- [tokio-util `LengthDelimitedCodec`](https://docs.rs/tokio-util/latest/tokio_util/codec/length_delimited/index.html): the same 4-byte-length frame format as a codec. Read what [`Builder::max_frame_length`](https://docs.rs/tokio-util/latest/tokio_util/codec/length_delimited/struct.Builder.html#method.max_frame_length) defaults to.
- [tonic `Grpc::max_decoding_message_size`](https://docs.rs/tonic/0.14.6/tonic/server/struct.Grpc.html#method.max_decoding_message_size): gRPC's version of this limit.
- [tungstenite `WebSocketConfig`](https://docs.rs/tungstenite/0.30.0/tungstenite/protocol/struct.WebSocketConfig.html): `max_message_size` and `max_frame_size`, the same idea for WebSocket frames.
- [bytes `Buf`](https://docs.rs/bytes/latest/bytes/buf/trait.Buf.html): read the Panics section of `get_u32`. A convenient getter can be a panicking index in disguise.
- [`Vec::with_capacity`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity): its Panics section, and what happens when the allocation itself fails.
- [Heartbleed, CVE-2014-0160](https://en.wikipedia.org/wiki/Heartbleed): a length field trusted off the wire, in C, where the result was a memory leak and not a panic.

In a review, check:
- `with_capacity(n)`, `vec![0; n]` or `resize(n, ..)` where `n` came off the wire and no limit check comes first.
- `buf[a..b]` and `.try_into().unwrap()` in parser code that untrusted bytes reach. The [`clippy::indexing_slicing`](https://clippy.rust-lang.org/master/index.html#indexing_slicing) lint finds the first.
- `offset + len` arithmetic on untrusted lengths that can overflow.
- Limits that differ between the proxy, the gateway and the service, so one layer buffers what the next one rejects.
