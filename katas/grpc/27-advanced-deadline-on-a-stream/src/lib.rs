use tokio_stream::StreamExt;
use tonic::Status;
use tonic::transport::Channel;

pub mod pb {
    tonic::include_proto!("ticker");
}

use pb::WatchRequest;
use pb::ticker_client::TickerClient;

/// Collects the first `n` prices for `symbol` from a server stream.
/// CLUE: The server sent two ticks and went quiet. Its TCP connection is still open. What is `next()` waiting for?
/// CLUE: Which tokio function bounds how long any future may take? What `Status` should a caller see when it fires?
pub async fn first_n(
    client: &mut TickerClient<Channel>,
    symbol: &str,
    n: usize,
) -> Result<Vec<String>, Status> {
    let mut stream = client
        .watch(WatchRequest {
            symbol: symbol.to_string(),
        })
        .await?
        .into_inner();
    let mut prices = Vec::new();
    while prices.len() < n {
        match stream.next().await {
            Some(tick) => prices.push(tick?.price),
            None => break,
        }
    }
    Ok(prices)
}
