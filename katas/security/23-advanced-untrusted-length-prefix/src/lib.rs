/// Largest frame body this service accepts.
pub const MAX_FRAME: usize = 1024 * 1024;

#[derive(Debug, PartialEq)]
pub enum FrameError {
    Truncated,
    TooLarge(usize),
}

/// Decodes one frame: a 4-byte big-endian length, then that many bytes.
/// CLUE: Who chose `len`? What happens to `with_capacity` when they send `0xFFFFFFFF`?
/// CLUE: What does indexing past the end of `buf` do, and who gets to trigger it?
pub fn decode_frame(buf: &[u8]) -> Result<Vec<u8>, FrameError> {
    let len = u32::from_be_bytes(buf[..4].try_into().unwrap()) as usize;
    let mut body = Vec::with_capacity(len);
    body.extend_from_slice(&buf[4..4 + len]);
    Ok(body)
}
