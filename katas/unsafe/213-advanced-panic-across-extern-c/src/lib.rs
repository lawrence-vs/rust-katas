/// Parses `len` bytes at `ptr` as a signed decimal quantity and writes it to `*out`.
/// Returns 0 on success, -1 for a null pointer, -2 for invalid UTF-8, -3 for a number that does not parse.
/// CLUE: A panic that leaves an `extern "C"` function cannot unwind into C. What does Rust do instead?
/// CLUE: Which of the four return codes is never produced when the input is `abc`?
///
/// # Safety
/// `ptr` must be valid for `len` reads, and `out` must be valid for one write, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_qty(ptr: *const u8, len: usize, out: *mut i64) -> i32 {
    if ptr.is_null() || out.is_null() {
        return -1;
    }
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let Ok(s) = std::str::from_utf8(bytes) else {
        return -2;
    };
    let n: i64 = s.parse().unwrap();
    unsafe { *out = n };
    0
}
