/// CLUE: the dependency list in Cargo.toml decides which TLS stack the whole tree links.
pub fn ping() -> &'static str {
    "pong"
}
