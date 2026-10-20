/// Parses a TCP port such as `"8080"`.
/// CLUE: Who should decide what happens on bad input: this library, or the program that calls it?
/// CLUE: What does `str::parse` already return before you `unwrap` it?
pub fn parse_port(s: &str) -> u16 {
    s.trim().parse().unwrap()
}
