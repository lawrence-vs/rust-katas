use tokio::sync::mpsc::{self, UnboundedReceiver};

/// CLUE: `StreamExt::next` needs `Stream`; does the channel's receiver implement it, and should callers see the channel at all?
pub fn events() -> UnboundedReceiver<u32> {
    let (tx, rx) = mpsc::unbounded_channel();
    for n in 1..=3 {
        tx.send(n).unwrap();
    }
    rx
}
