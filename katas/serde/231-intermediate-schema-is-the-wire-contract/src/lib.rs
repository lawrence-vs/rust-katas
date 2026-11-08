use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// An order on the wire. Clients were told the quantity key is `volume`.
/// CLUE: which attribute decides the JSON key, the Rust field name or something else on the field?
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Order {
    pub symbol: String,
    #[serde(rename = "qty")]
    pub volume: u32,
}
