//! Summarise a trades CSV: how many trades, how much was bought and sold.

use csv::{ReaderBuilder, Trim};
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Trade {
    pub id: u64,
    pub symbol: String,
    pub side: Side,
    pub qty: u64,
    pub price: u64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Summary {
    pub trades: u64,
    pub bought: u64,
    pub sold: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum TradeError {
    #[error("reading trades: {0}")]
    Io(#[from] std::io::Error),
    #[error("bad trade row: {0}")]
    Csv(#[from] csv::Error),
    #[error("quantity total overflowed")]
    Overflow,
}

/// CLUE: `csv::ReaderBuilder::from_reader` takes any `R: io::Read`. What type is `input` already?
pub fn summarise<R: Read>(mut input: R) -> Result<Summary, TradeError> {
    let mut text = String::new();
    // CLUE: How big is `text` when the file has a million rows? Does a CSV reader need the whole file to read the first row?
    input.read_to_string(&mut text)?;

    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(text.as_bytes());
    // CLUE: Once row 8 has been read, what does the summary still need from row 7?
    let all = rdr.deserialize::<Trade>().collect::<Result<Vec<_>, _>>()?;

    let mut summary = Summary::default();
    for trade in all {
        summary.trades = summary.trades.checked_add(1).ok_or(TradeError::Overflow)?;
        match trade.side {
            Side::Buy => {
                summary.bought = summary
                    .bought
                    .checked_add(trade.qty)
                    .ok_or(TradeError::Overflow)?
            }
            Side::Sell => {
                summary.sold = summary
                    .sold
                    .checked_add(trade.qty)
                    .ok_or(TradeError::Overflow)?
            }
        }
    }
    Ok(summary)
}
