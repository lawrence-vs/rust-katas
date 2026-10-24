use std::error::Error;
use std::fmt;
use std::io::{self, Read, Write};

#[derive(Debug)]
pub enum CopyError {
    /// The input stream failed.
    In(io::Error),
    /// The output stream failed.
    Out(io::Error),
}

impl fmt::Display for CopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CopyError::In(_) => write!(f, "copy failed reading the input"),
            CopyError::Out(_) => write!(f, "copy failed writing the output"),
        }
    }
}

/// CLUE: A caller holding a `&dyn Error` can only reach the `io::Error` through one method.
/// What does it return here?
impl Error for CopyError {}

/// Copies `input` to `output` and returns the number of bytes copied.
///
/// CLUE: Which stream does each `map_err` below blame? Read the variant docs again.
pub fn copy<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<u64, CopyError> {
    let mut buf = [0u8; 8];
    let mut total = 0u64;
    loop {
        let n = input.read(&mut buf).map_err(CopyError::Out)?;
        if n == 0 {
            return Ok(total);
        }
        output.write_all(&buf[..n]).map_err(CopyError::In)?;
        total += n as u64;
    }
}

/// Returns the kind of the first `io::Error` found by walking the `source()` chain of `err`.
///
/// CLUE: The walk starts at `err` itself; `source()` is how it steps to the next error.
pub fn io_kind(err: &(dyn Error + 'static)) -> Option<io::ErrorKind> {
    let mut current = Some(err);
    while let Some(e) = current {
        if let Some(io_err) = e.downcast_ref::<io::Error>() {
            return Some(io_err.kind());
        }
        current = e.source();
    }
    None
}
