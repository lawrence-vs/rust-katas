use serde::Deserialize;

/// A fill report from an upstream feed. A fill without a quantity is invalid.
/// CLUE: `default` is for fields that may be absent. What value does the parser invent when `qty` is missing?
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Fill {
    pub id: u64,
    #[serde(default)]
    pub qty: u64,
}
