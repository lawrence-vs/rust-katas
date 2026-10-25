//! tally: count the lines, words or bytes that arrive on stdin.

use std::fmt;
use std::io::Read;
use std::process::ExitCode;

/// Mode is what to count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Lines,
    Words,
    Bytes,
}

/// UsageError says why the arguments were rejected.
#[derive(Debug, PartialEq, Eq)]
enum UsageError {
    MissingMode,
    UnknownMode(String),
    TooManyArgs,
}

impl fmt::Display for UsageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UsageError::MissingMode => write!(f, "usage: tally <lines|words|bytes>"),
            UsageError::UnknownMode(mode) => {
                write!(f, "unknown mode '{mode}': expected lines, words or bytes")
            }
            UsageError::TooManyArgs => write!(f, "expected one argument: lines, words or bytes"),
        }
    }
}

impl std::error::Error for UsageError {}

/// parse_mode reads the mode from the arguments after the program name.
///
/// CLUE: A file in `tests/` is its own crate, and it can only `use` a library. How many crates does this package build, and which kinds are they?
fn parse_mode(args: &[String]) -> Result<Mode, UsageError> {
    match args {
        [] => Err(UsageError::MissingMode),
        [mode] => match mode.as_str() {
            "lines" => Ok(Mode::Lines),
            "words" => Ok(Mode::Words),
            "bytes" => Ok(Mode::Bytes),
            other => Err(UsageError::UnknownMode(other.to_string())),
        },
        _ => Err(UsageError::TooManyArgs),
    }
}

/// count returns how many lines, words or bytes `text` holds.
///
/// ```
/// use kata_project_structure_180::{Mode, count};
/// assert_eq!(count(Mode::Words, "one two\nthree\n"), 3);
/// ```
///
/// CLUE: `cargo test` has never run the example above. Which kind of crate does rustdoc test?
fn count(mode: Mode, text: &str) -> usize {
    match mode {
        Mode::Lines => text.lines().count(),
        Mode::Words => text.split_whitespace().count(),
        Mode::Bytes => text.len(),
    }
}

/// CLUE: When the logic lives somewhere a test can reach, what is left for `main`? Four jobs, none of them counting.
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = match parse_mode(&args) {
        Ok(mode) => mode,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::from(2);
        }
    };
    let mut text = String::new();
    if let Err(err) = std::io::stdin().read_to_string(&mut text) {
        eprintln!("error: reading stdin: {err}");
        return ExitCode::from(1);
    }
    println!("{}", count(mode, &text));
    ExitCode::SUCCESS
}
