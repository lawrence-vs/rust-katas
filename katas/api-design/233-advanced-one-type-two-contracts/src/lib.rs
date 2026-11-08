use serde_json::{Value, json};

// CLUE: the proto file and the agreed wire name say `volume`; what does this attribute emit?
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Order {
    pub symbol: String,
    #[serde(rename = "qty")]
    pub volume: u32,
}

/// CLUE: the schema must stay generated from `Order`, never hand-written.
pub fn openapi() -> Value {
    let schema = serde_json::to_value(schemars::schema_for!(Order)).unwrap_or(Value::Null);
    json!({
        "openapi": "3.1.0",
        "info": {"title": "orders", "version": "1.0.0"},
        "paths": {"/orders": {"post": {
            "requestBody": {"required": true, "content": {"application/json": {
                "schema": {"$ref": "#/components/schemas/Order"}
            }}},
            "responses": {"201": {"description": "created"}}
        }}},
        "components": {"schemas": {"Order": schema}}
    })
}
