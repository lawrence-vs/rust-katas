//! 57 intermediate: untrusted JSON
//!
//! Think further, from the kata README:
//! 1. `#[serde(flatten)]` doesn't work with `deny_unknown_fields`. Why not?
//! 2. Be strict on requests, but what about responses from an upstream venue that may add fields (forward compatibility)?
//! 3. How would you parse `"19.99"` into cents without `f64`?
//! 4. Where does a body size limit go relative to `from_str` (katas 21 and 23)?
//! 5. How do you get a field path for an error in a nested struct (the `serde_path_to_error` crate)?
//! 6. What length and charset limits does `client_order_id` need?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use serde::Deserialize;

// Think further 2: a strict type for requests. A response from a venue would skip deny_unknown_fields.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderRequest {
    client_order_id: String,
    qty: u64,
}

// Think further 3: a price arrives as a string and becomes integer cents, never through f64.
fn parse_cents(s: &str) -> Option<u64> {
    let (whole, frac) = s.split_once('.').unwrap_or((s, "0"));
    if frac.len() > 2 {
        return None;
    }
    let whole: u64 = whole.parse().ok()?;
    let frac: u64 = format!("{frac:0<2}").parse().ok()?;
    whole.checked_mul(100)?.checked_add(frac)
}

fn main() {
    let ok = r#"{"client_order_id":"abc-1","qty":3}"#;
    if let Ok(req) = serde_json::from_str::<OrderRequest>(ok) {
        println!("ok: {} qty {}", req.client_order_id, req.qty);
    }

    // Think further 6 and 4: the limits the field needs go in the type or right after it parses.
    let extra = r#"{"client_order_id":"abc-2","qty":3,"venue":"x"}"#;
    println!(
        "unknown field: {:?}",
        serde_json::from_str::<OrderRequest>(extra).is_err()
    );

    println!("19.99 -> {:?}", parse_cents("19.99"));
    println!("0.10 -> {:?}", parse_cents("0.10"));
    println!("1.234 -> {:?}", parse_cents("1.234"));
}
