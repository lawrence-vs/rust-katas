//! Net position per symbol from a CSV of trades.

use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("csv: {0}")]
    Csv(#[from] csv::Error),
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Deserialize)]
struct Trade {
    symbol: String,
    side: Side,
    qty: u32,
}

/// Reads `symbol,side,qty` rows and writes `"{symbol} {net}\n"` per symbol, sorted by symbol.
/// CLUE: A test can't call this without writing a file first. Which std traits do `File`, `&[u8]` and `Vec<u8>` share?
pub fn run(path: &str) -> Result<(), Error> {
    // CLUE: `from_path` opens the file for you. Which `csv::Reader` constructor takes something already open?
    let mut reader = csv::Reader::from_path(path)?;
    let mut net: BTreeMap<String, i64> = BTreeMap::new();
    for row in reader.deserialize() {
        let trade: Trade = row?;
        let qty = i64::from(trade.qty);
        *net.entry(trade.symbol).or_insert(0) += match trade.side {
            Side::Buy => qty,
            Side::Sell => -qty,
        };
    }
    for (symbol, qty) in &net {
        // CLUE: `println!` always writes to stdout and panics if it can't. Which macro returns the error instead?
        println!("{symbol} {qty}");
    }
    Ok(())
}
