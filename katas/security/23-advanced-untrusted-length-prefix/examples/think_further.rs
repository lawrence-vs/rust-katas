//! 23 advanced: an untrusted length prefix
//!
//! Think further, from the kata README:
//! 1. A length of exactly `MAX_FRAME`: accepted or rejected? Do the tests pin it down, and should they?
//! 2. `buf` holds one frame and the start of the next. What does the caller need back to find the next frame, and does the signature give it to them?
//! 3. Off a socket, `Truncated` can mean "wait for more bytes" or "the peer hung up mid-frame". Which layer should tell the two apart?
//! 4. On a 32-bit target `usize` is 32 bits. What is `4 + len` when `len` is `u32::MAX`? Does your version ever compute it?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

const MAX_FRAME: usize = 1024;

#[derive(Debug, PartialEq, Eq)]
enum FrameError {
    TooLarge,
    Truncated,
}

// Think further 1 and 4: compare in u64 so 4 + len can't wrap on a 32-bit usize.
// Think further 2: return the frame and the rest of the buffer, so the caller finds the next frame.
fn read_frame(buf: &[u8]) -> Result<(&[u8], &[u8]), FrameError> {
    let header: [u8; 4] = buf.get(..4).and_then(|h| h.try_into().ok()).ok_or(FrameError::Truncated)?;
    let len = u32::from_be_bytes(header) as u64;
    if len > MAX_FRAME as u64 {
        return Err(FrameError::TooLarge);
    }
    let end = 4 + len as usize;
    if buf.len() < end {
        return Err(FrameError::Truncated);
    }
    Ok((&buf[4..end], &buf[end..]))
}

fn main() {
    let mut stream = Vec::new();
    for body in [&b"hi"[..], &b"there"[..]] {
        stream.extend_from_slice(&(body.len() as u32).to_be_bytes());
        stream.extend_from_slice(body);
    }

    // Read frames back to back, passing the rest of the buffer on each time.
    let mut rest: &[u8] = &stream;
    while !rest.is_empty() {
        match read_frame(rest) {
            Ok((frame, next)) => {
                println!("frame: {:?}", String::from_utf8_lossy(frame));
                rest = next;
            }
            Err(e) => {
                println!("stopped: {e:?}");
                break;
            }
        }
    }

    // Think further 3: a length prefix that claims more than the bytes present is Truncated, not a crash.
    println!("huge claim: {:?}", read_frame(&[0xff, 0xff, 0xff, 0xff, 1]));
    println!("exactly the limit, short body: {:?}", read_frame(&[0, 0, 4, 0, 1, 2]));
}
