#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("add.h");

        fn add_i64(a: i64, b: i64) -> i64;
    }
}

/// Adds two numbers in C++.
/// CLUE: `cxx::bridge` writes the C++ header and the Rust glue. Who compiles `add.cc` into the library?
/// CLUE: Read the linker error before you touch the Rust code. Which symbol is missing?
pub fn add(a: i64, b: i64) -> i64 {
    ffi::add_i64(a, b)
}
