use tokio::sync::mpsc;

#[derive(Debug, PartialEq)]
pub struct Rejected;

/// The front door for messages from untrusted clients.
pub struct Ingest {
    tx: mpsc::UnboundedSender<Vec<u8>>,
}

/// Builds the queue between the network and the worker.
/// CLUE: `capacity` is never used. What bounds the memory this queue can hold?
/// CLUE: When the worker falls behind, should the network task wait, or tell the client no?
pub fn ingest(_capacity: usize) -> (Ingest, mpsc::UnboundedReceiver<Vec<u8>>) {
    let (tx, rx) = mpsc::unbounded_channel();
    (Ingest { tx }, rx)
}

impl Ingest {
    /// Queues one message, or rejects it.
    pub fn accept(&self, msg: Vec<u8>) -> Result<(), Rejected> {
        self.tx.send(msg).map_err(|_| Rejected)
    }
}
