# 91 beginner: clap derive, not hand-parsed args

**Concept:** Describe the arguments as a struct and let clap parse, validate and document them; tests hand their own argument list to `try_parse_from` instead of reading the process arguments.
**Source:** clap [derive tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/); own design (a `report` command). Why clap: it is the most-downloaded CLI parser on crates.io; the main alternative is `argh` (or `lexopt`/`pico-args`), which is smaller and quicker to compile but has less validation and fewer helpful errors.
**Fails with:** `E0599`: no function or associated item named `try_parse_from` found for struct `kata_cli_91::Args` in the current scope
**Vault:** your note on parsing user input at the edge

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Update `src/main.rs` to match. None of the following in `src/lib.rs`: `unwrap`, `expect`, `panic!`, `std::env`, `#[allow(...)]`.

## Before you fix it
1. Read the E0599 error: which trait provides `try_parse_from`, and what must `Args` do to have it?
2. What does `from_env` do with `--limit abc`? With `--limit 0`? With `--limt 5`?

## Answer in the PR
1. Why couldn't `from_env` be tested, and what does the test hand the parser instead?
2. Why does `parse()` exit with code 2 while `try_parse_from` returns, and why do the tests need the second?
3. Which `ErrorKind` did `--limit 0` give versus `--format xml`, and why are they different?
4. Where did the `--help` text come from?

## Think further
1. `Option<u16>` versus `default_value_t`: when is "absent" a meaningful value?
2. What would `env = "REPORT_LIMIT"` change, and which source should win?
3. How would you add subcommands with `#[derive(Subcommand)]`?
4. `PathBuf` versus `String` for a file argument: what about non-UTF-8 paths?

## In the wild
- ripgrep and cargo both parse their arguments with clap.
- `clap::Command::debug_assert` in a test catches conflicting argument definitions.

In a review, check:
- `env::args().nth(..)` in library code.
- `.parse().unwrap()` on user input.
- Hand-written usage strings that can drift from the real flags.
