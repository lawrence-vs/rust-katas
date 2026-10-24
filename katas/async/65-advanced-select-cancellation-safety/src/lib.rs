use std::error::Error;
use std::fmt;
use std::io;
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::time::{Instant, interval_at};

/// Largest body a frame may carry.
pub const MAX_FRAME: usize = 1024;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Session {
    pub frames: Vec<Vec<u8>>,
    pub heartbeats: u32,
}

#[derive(Debug)]
pub enum FrameError {
    Io(io::Error),
    TooLong(usize),
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameError::Io(e) => write!(f, "{e}"),
            FrameError::TooLong(n) => {
                write!(f, "frame of {n} bytes is over the {MAX_FRAME} limit")
            }
        }
    }
}

impl Error for FrameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FrameError::Io(e) => Some(e),
            FrameError::TooLong(_) => None,
        }
    }
}

impl From<io::Error> for FrameError {
    fn from(e: io::Error) -> Self {
        FrameError::Io(e)
    }
}

/// Reads one frame: a 4-byte big-endian length, then that many bytes.
/// `Ok(None)` when the stream ends before a header.
/// CLUE: When the heartbeat branch wins, what happens to this future, and to the bytes it already read?
/// CLUE: tokio's docs say which read methods are cancellation safe. Is `read_exact`? Is `read`?
pub async fn read_frame<R: AsyncRead + Unpin>(r: &mut R) -> Result<Option<Vec<u8>>, FrameError> {
    let mut len = [0u8; 4];
    match r.read_exact(&mut len).await {
        Ok(_) => {}
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e.into()),
    }
    let len = u32::from_be_bytes(len) as usize;
    if len > MAX_FRAME {
        return Err(FrameError::TooLong(len));
    }
    let mut body = vec![0; len];
    r.read_exact(&mut body).await?;
    Ok(Some(body))
}

/// Reads frames until the stream ends, counting a heartbeat every `heartbeat`.
/// CLUE: Where could half a frame live so it outlasts one turn of this loop?
pub async fn run<R: AsyncRead + Unpin>(
    mut reader: R,
    heartbeat: Duration,
) -> Result<Session, FrameError> {
    let mut session = Session::default();
    let mut ticks = interval_at(Instant::now() + heartbeat, heartbeat);
    loop {
        tokio::select! {
            frame = read_frame(&mut reader) => match frame? {
                Some(f) => session.frames.push(f),
                None => return Ok(session),
            },
            _ = ticks.tick() => session.heartbeats += 1,
        }
    }
}
