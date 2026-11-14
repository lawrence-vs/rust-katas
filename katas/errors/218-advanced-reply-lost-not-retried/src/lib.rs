#[derive(Debug, PartialEq, Eq)]
pub enum OrderError {
    DialTimeout,
    ReplyLost(String),
}

impl OrderError {
    pub fn is_retryable(&self) -> bool {
        matches!(self, OrderError::DialTimeout)
    }
}

pub trait Wire {
    fn write(&mut self, msg: &str) -> Result<(), String>;
    fn read_reply(&mut self) -> Result<String, String>;
}

/// CLUE: which step was the order already on the wire after?
/// CLUE: does an error from `write` mean the same thing as one from `read_reply`?
pub fn place_order<W: Wire>(wire: &mut W, order: &str) -> Result<String, OrderError> {
    wire.write(order).map_err(|_| OrderError::DialTimeout)?;
    wire.read_reply().map_err(|_| OrderError::DialTimeout)
}
