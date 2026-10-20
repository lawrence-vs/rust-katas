use kata_security_23::{FrameError, MAX_FRAME, decode_frame};

#[test]
fn decodes_a_frame() {
    assert_eq!(
        decode_frame(&[0, 0, 0, 3, b'a', b'b', b'c']),
        Ok(b"abc".to_vec())
    );
}

#[test]
fn rejects_oversized_length() {
    let claimed = MAX_FRAME as u32 + 1;
    let mut buf = claimed.to_be_bytes().to_vec();
    buf.push(0);
    assert_eq!(decode_frame(&buf), Err(FrameError::TooLarge(MAX_FRAME + 1)));
}

#[test]
fn short_header_is_truncated() {
    assert_eq!(decode_frame(&[0, 0]), Err(FrameError::Truncated));
}

#[test]
fn short_body_is_truncated() {
    assert_eq!(decode_frame(&[0, 0, 0, 5, 1]), Err(FrameError::Truncated));
}
