//! Loading a batch of trades from CSV.

use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Trade {
    pub id: u64,
    pub account: String,
    pub amount_cents: i64,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Report {
    pub trades: Vec<Trade>,
    /// CLUE: What ever increments this? Which csv errors still leave the next row readable, and which mean nothing more can be read?
    pub skipped: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("reading trades failed")]
    Io(#[from] std::io::Error),
}

/// CLUE: This sends every csv error down `Io`. Is `12x` in the amount column an I/O failure?
impl From<csv::Error> for LoadError {
    fn from(e: csv::Error) -> Self {
        LoadError::Io(e.into())
    }
}

/// Loads trades from a CSV with an `id,account,amount_cents` header.
///
/// CLUE: `?` returns on the first error. Should one bad row decide that the other rows are lost?
pub fn load_trades<R: Read>(input: R) -> Result<Report, LoadError> {
    let mut rdr = csv::Reader::from_reader(input);
    let mut report = Report::default();
    for result in rdr.deserialize::<Trade>() {
        let trade = result?;
        report.trades.push(trade);
    }
    Ok(report)
}
