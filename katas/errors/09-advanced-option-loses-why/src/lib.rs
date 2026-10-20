#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u8,
}

/// Parses a `name, age` line from a users file.
/// CLUE: When this returns `None`, can the caller tell a missing field from a bad age?
/// CLUE: `thiserror` is already in Cargo.toml. What do `#[error]` and `#[source]` generate?
pub fn load_user(line: &str) -> Option<User> {
    let mut parts = line.split(',');
    let name = parts.next()?.trim().to_string();
    let age = parts.next()?.trim().parse().ok()?;
    Some(User { name, age })
}
