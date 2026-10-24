# 94 intermediate: framing, not one read

**Concept:** TCP is a byte stream, so one `read` can return half a message or two. Frame with a codec that keeps leftover bytes between reads and caps the line length.
**Source:** tokio-util codec docs, https://docs.rs/tokio-util/0.7.20/tokio_util/codec/index.html; the tokio tutorial, "Framing", https://tokio.rs/tokio/tutorial/framing; *Network Programming with Rust* (Chanda) (no Readwise highlight yet, so the book is the citation)
**Fails with:** `one_message_split_across_two_reads`
**Vault:** your note on message framing

## Task
Make `cargo test` pass without changing the tests or the signatures. No `unwrap`/`expect`/`panic!` in `src/`. Don't hand-write a byte-scanning loop. Only `src/lib.rs` needs to change; the dependencies are already in `Cargo.toml`.

## Before you fix it
Run the tests and read which reads each failing test made. Read the CLUEs.

## Answer in the PR
1. Walk the split test through the starting code.
2. Where do the leftover bytes live after your fix?
3. What does `MAX_LINE` protect against, and what does the codec do with the rest of an over-long line?
4. Why can the TCP test never be the one that proves the bug?

## Think further
1. `LengthDelimitedCodec` against lines: what changes for binary payloads? (Kata 23.)
2. The peer closes after `HEL` with no newline. What does `LinesCodec` return at EOF, and is that what you want?
3. How would you reply on the same connection with `Framed` and `SinkExt`?
4. A per-line deadline against a whole-connection deadline. (Kata 22.)
5. After `MaxLineLengthExceeded`, can the stream resume, or should you close it?

## In the wild
- [mini-redis `Connection::read_frame`](https://github.com/tokio-rs/mini-redis/blob/master/src/connection.rs): a buffer that keeps leftover bytes between reads.
- [`LinesCodec`](https://docs.rs/tokio-util/0.7.20/tokio_util/codec/struct.LinesCodec.html) and [`LengthDelimitedCodec`](https://docs.rs/tokio-util/0.7.20/tokio_util/codec/struct.LengthDelimitedCodec.html).
- tokio-tungstenite and tonic frame their protocols the same way over a byte stream.

In a review, check:
- `read(&mut buf)` results used as whole messages.
- Line buffers with no limit.
- `from_utf8_lossy` on network bytes.
