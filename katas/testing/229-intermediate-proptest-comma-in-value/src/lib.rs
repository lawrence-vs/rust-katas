/// CLUE: which characters can a value hold that `decode` also treats as a separator?
pub fn encode(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(",")
}

/// CLUE: a round trip must hold for every value, so decode must undo exactly what encode did.
pub fn decode(raw: &str) -> Vec<(String, String)> {
    if raw.is_empty() {
        return Vec::new();
    }
    raw.split(',')
        .map(|piece| {
            let (k, v) = piece.split_once('=').unwrap_or((piece, ""));
            (k.to_string(), v.to_string())
        })
        .collect()
}
