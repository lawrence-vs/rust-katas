use std::io;

use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};

/// Reads the peer's first line, e.g. `"HELLO v1"`.
/// CLUE: The peer connects and never sends a byte. When does this return?
/// CLUE: Which tokio function puts a deadline on any future?
pub async fn read_greeting<R: AsyncRead + Unpin>(conn: R) -> io::Result<String> {
    let mut line = String::new();
    BufReader::new(conn).read_line(&mut line).await?;
    Ok(line.trim_end().to_string())
}
