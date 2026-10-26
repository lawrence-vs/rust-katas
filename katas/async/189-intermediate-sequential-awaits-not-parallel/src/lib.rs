use std::time::Duration;

use tokio::time::sleep;

/// One venue's price, returned after `latency`.
async fn ask_venue(price: u64, latency: Duration) -> u64 {
    sleep(latency).await;
    price
}

/// Prices from two venues, each answering after 100ms.
/// CLUE: `.await` stops this function until that call finishes. Can the second call start while the first one waits?
/// CLUE: Which tokio tool polls several futures together and waits for all of them?
pub async fn quotes_from_two_venues() -> (u64, u64) {
    let a = ask_venue(101, Duration::from_millis(100)).await;
    let b = ask_venue(102, Duration::from_millis(100)).await;
    (a, b)
}
