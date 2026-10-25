//! 163 advanced: a parser that never panics on garbage
//!
//! Think further, from the kata README:
//! 1. `cargo install cargo-fuzz`, then from this kata's directory run `cargo fuzz init`. It creates a `fuzz/` directory with a target file under `fuzz/fuzz_targets/`. Replace that file's contents with the target shown in the README. Run `cargo +nightly fuzz run <target>` on the starting code. How long until the first crash, and where is it saved (`fuzz/artifacts/<target>/`)?
//! 2. Run the same command with the crash file as a second argument to reproduce it. Then turn the input into a table case.
//! 3. libFuzzer is coverage-guided; `hostile` is blind. What can libFuzzer reach that `hostile` can't? Why seed `fuzz/corpus/<target>/` with the examples in `examples/think_further.rs`?
//! 4. The target writes `let _ =`, which kata 124 calls a bug. Why is it right in a fuzz target?
//! 5. How would you run it on a schedule anyway (`-- -max_total_time=300`), and why not on every push?

// Scratch file for the Think further items, as in kata 161. Run with: cargo run --example think_further

use kata_testing_163::parse_record;

fn main() {
    // Think further 3: these are the seeds you would put in fuzz/corpus/<target>/.
    for seed in [
        &b"apple,12,fresh"[..],
        b"pear,0,",
        b"fig,4294967295,max qty",
    ] {
        println!(
            "{:?} -> {:?}",
            String::from_utf8_lossy(seed),
            parse_record(seed)
        );
    }
}
