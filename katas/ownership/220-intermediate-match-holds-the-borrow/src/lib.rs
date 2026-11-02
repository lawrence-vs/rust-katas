#[derive(Debug, PartialEq)]
pub struct Pending {
    pub id: u32,
    pub acked: bool,
}

pub struct Session {
    pub queue: Vec<Pending>,
    pub sent: u32,
    counter: u32,
}

impl Session {
    pub fn new(ids: &[u32]) -> Self {
        let queue = ids.iter().map(|&id| Pending { id, acked: false }).collect();
        Session {
            queue,
            sent: 0,
            counter: 0,
        }
    }

    pub fn next_id(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }

    pub fn record(&mut self) {
        self.sent += 1;
    }

    /// CLUE: `p` points into `self.queue`; is it still alive when `self.record()` runs?
    pub fn ack_front(&mut self) -> Option<u32> {
        match self.queue.iter_mut().find(|p| !p.acked) {
            Some(p) => {
                self.record();
                p.acked = true;
                Some(p.id)
            }
            None => None,
        }
    }
}
