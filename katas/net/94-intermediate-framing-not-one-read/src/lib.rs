use std::fmt;

use tokio::io::{AsyncRead, AsyncReadExt};

/// Longest line accepted, in bytes, not counting the newline.
pub const MAX_LINE: usize = 1024;

#[derive(Debug)]
pub enum FrameError {
    /// A line grew past MAX_LINE bytes without a newline.
    TooLong,
    /// The connection failed or sent bad UTF-8.
    Io(std::io::Error),
}

impl From<std::io::Error> for FrameError {
    fn from(e: std::io::Error) -> Self {
        FrameError::Io(e)
    }
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameError::TooLong => write!(f, "line longer than {MAX_LINE} bytes"),
            FrameError::Io(e) => write!(f, "connection error: {e}"),
        }
    }
}

impl std::error::Error for FrameError {}

/// Reads newline-delimited messages until EOF and returns them without the line ending.
/// CLUE: TCP delivers a stream of bytes. What does one `read` promise about where a message starts or ends?
/// CLUE: The peer sent two lines before you read. How many entries does this loop push?
/// CLUE: The peer never sends a newline. Who decides when to stop buffering?
pub async fn read_messages<R: AsyncRead + Unpin>(mut conn: R) -> Result<Vec<String>, FrameError> {
    let mut out = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = conn.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        out.push(String::from_utf8_lossy(&buf[..n]).trim_end().to_string());
    }
    Ok(out)
}
