use std::rc::Rc;

/// Read-only venue config shared by every fill.
pub struct Venue {
    pub name: String,
    pub fee_bps: u64,
}

/// Builds one fill's confirmation line.
struct Confirmer {
    venue: Rc<Venue>,
}

async fn publish(_fill_id: u64) {
    tokio::task::yield_now().await;
}

impl Confirmer {
    /// CLUE: `self` is moved into the future this returns. What is the future holding before its first poll?
    /// CLUE: Open `Rc` in the std docs and scroll to "Auto Trait Implementations".
    async fn confirm(self, fill_id: u64, notional: u64) -> String {
        let fee = notional * self.venue.fee_bps / 10_000;
        publish(fill_id).await;
        format!("{}:{fill_id}:{fee}", self.venue.name)
    }
}

/// Confirms every fill on the runtime's worker threads, in input order.
/// CLUE: `tokio::spawn` may move a task to another thread at any `.await`. What does that ask of everything the task owns?
pub async fn confirm_all(venue: Venue, fills: Vec<(u64, u64)>) -> Vec<String> {
    let venue = Rc::new(venue);
    let handles: Vec<_> = fills
        .into_iter()
        .map(|(id, notional)| {
            tokio::spawn(
                Confirmer {
                    venue: venue.clone(),
                }
                .confirm(id, notional),
            )
        })
        .collect();
    let mut out = Vec::with_capacity(handles.len());
    for h in handles {
        out.push(h.await.unwrap());
    }
    out
}
