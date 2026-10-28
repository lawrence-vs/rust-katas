fn main() {
    cxx_build::bridge("src/lib.rs")
        .include("src")
        .std("c++17")
        .compile("kata_cxx_add");
    println!("cargo:rerun-if-changed=src/add.cc");
    println!("cargo:rerun-if-changed=src/add.h");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
