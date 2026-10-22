//! An order request from an untrusted client. `price` is in cents.

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub u64);

impl Price {
    pub fn cents(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(pub u64);

impl Quantity {
    pub fn get(self) -> u64 {
        self.0
    }
}

/// CLUE: What does serde do with a JSON key this struct has no field for?
/// CLUE: If `Order` deserialises *through* another type, which struct do
/// container attributes like `deny_unknown_fields` belong on?
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Order {
    pub client_order_id: String,
    pub side: Side,
    /// CLUE: What is `1999.5 as u64`? And `-1.0 as u64`?
    pub price: f64,
    pub quantity: i64,
    #[serde(default)]
    pub post_only: bool,
}

impl Order {
    pub fn client_order_id(&self) -> &str {
        &self.client_order_id
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn price(&self) -> Price {
        Price(self.price as u64)
    }

    /// CLUE: What is `-5i64 as u64`?
    pub fn quantity(&self) -> Quantity {
        Quantity(self.quantity as u64)
    }

    pub fn post_only(&self) -> bool {
        self.post_only
    }
}

pub fn parse_order(json: &str) -> Result<Order, serde_json::Error> {
    serde_json::from_str(json)
}
