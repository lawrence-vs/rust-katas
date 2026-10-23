//! 07 beginner: unwrap in a library
//!
//! Think further, from the kata README:
//! 1. After it passes, call it with `"+80"`, `"0"`, `""` and `"8080\n"`. Which come back as a port? Port 0 is a valid `u16` but tells the OS "pick any port": should this function reject it, and how would a caller learn why?
//! 2. Print the error for `"http"` and for `"70000"`, once with `{}` and once with `{:?}`. Is either good enough to show the person who typed the flag?
//! 3. A binary calls `parse_port` on a command-line flag. Where does the error turn into a message and an exit code now? What does `fn main() -> Result<(), E>` print when it returns `Err`, and what must `E` implement?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further -- 70000

use std::net::SocketAddr;
use std::num::ParseIntError;

// Same behaviour as the kata's parse_port.
fn parse_port(s: &str) -> Result<u16, ParseIntError> {
    s.trim().parse()
}

// Returning Err from main prints Debug and exits with code 1.
fn main() -> Result<(), ParseIntError> {
    // Think further 1: the edge cases the test does not cover.
    for input in ["8080", "+80", "0", "", "8080\n", "http", "70000"] {
        println!("{input:?} => {:?}", parse_port(input));
    }

    // Think further 2: Display is for people, Debug is for developers.
    if let Err(e) = parse_port("http") {
        println!("display: {e}");
        println!("debug:   {e:?}");
    }

    // In the wild: std's own address parser returns an error, it does not panic.
    println!("{:?}", "127.0.0.1:80x".parse::<SocketAddr>().is_err());

    // Think further 3: the flag's error becomes a message and exit code 1 here.
    let flag = std::env::args().nth(1).unwrap_or_else(|| "8080".to_string());
    let port = parse_port(&flag)?;
    println!("listening on {port}");
    Ok(())
}
