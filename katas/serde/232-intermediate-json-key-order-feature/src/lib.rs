/// CLUE: where does `Value` keep an object's keys, and in what order does that container iterate them?
pub fn echo(raw: &str) -> Result<String, serde_json::Error> {
    let v: serde_json::Value = serde_json::from_str(raw)?;
    serde_json::to_string(&v)
}
