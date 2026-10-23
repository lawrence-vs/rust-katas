//! 30 expert: a dangling CString across FFI
//!
//! Think further, from the kata README:
//! 1. Call `c_len("héllo")`. Does `strlen` count characters or bytes? Which does a C caller usually want?
//! 2. The C function keeps the pointer after it returns (it registers a name or a callback). Is keeping the `CString` alive "across the call" still enough? Who must own it, and until when?
//! 3. The other direction: C returns a `*const c_char` that it owns. What must you know before you turn it into a `&str`, and how many ways can that fail?
//! 4. The string comes from a client (an order's client ID) and contains a NUL. Without the check in `CString::new`, what would C have seen?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::ffi::{CStr, CString};

// Think further 1: strlen counts bytes up to the NUL, so "héllo" is 6, not 5.
fn c_len(s: &str) -> Result<usize, std::ffi::NulError> {
    // Think further 2: bind the CString to a name so it lives until the end of the function.
    let owned = CString::new(s)?;
    let as_ptr = owned.as_ptr();
    // SAFETY: owned is alive here and as_ptr points at its NUL-terminated bytes.
    let borrowed = unsafe { CStr::from_ptr(as_ptr) };
    Ok(borrowed.to_bytes().len())
}

fn main() {
    println!("c_len(\"héllo\") = {:?}", c_len("héllo"));

    // Think further 4: a NUL inside a client string is rejected here, not seen as a shorter string by C.
    println!("with NUL: {:?}", c_len("ab\0cd").is_err());

    // In the wild: c"..." literals are borrowed C strings and need no runtime check.
    let literal: &CStr = c"venue";
    println!("literal bytes: {:?}", literal.to_bytes());
}
