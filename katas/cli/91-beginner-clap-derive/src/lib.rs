use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
    pub limit: u16,
    pub format: Format,
    pub verbose: bool,
}

/// CLUE: How would a parser know which strings are the allowed `Format`s?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Text,
    Json,
}

/// CLUE: A test can't choose what `std::env::args()` returns. Who should hand this function its arguments?
pub fn from_env() -> Args {
    let mut args = std::env::args().skip(1);
    let input = args.next().unwrap().into();
    let mut limit = 10;
    let mut format = Format::Text;
    let mut verbose = false;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            // CLUE: What does `.parse().unwrap()` do with `--limit abc`, and with `--limit 0`?
            "--limit" => limit = args.next().unwrap().parse().unwrap(),
            "--format" => {
                format = match args.next().unwrap().as_str() {
                    "text" => Format::Text,
                    "json" => Format::Json,
                    _ => panic!("bad format"),
                }
            }
            "-v" | "--verbose" => verbose = true,
            _ => {}
        }
    }
    Args {
        input,
        limit,
        format,
        verbose,
    }
}
