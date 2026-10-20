use std::ffi::{CString, c_char};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

/// The length C sees for `s`, or `None` if `s` contains a NUL byte.
/// CLUE: When is the `CString` built on the `let` line dropped? What does `p` point at after that?
/// CLUE: Read the compiler warning before you run the test. Then run `cargo +nightly miri test`.
pub fn c_len(s: &str) -> Option<usize> {
    let p = CString::new(s).ok()?.as_ptr();
    Some(unsafe { strlen(p) })
}
