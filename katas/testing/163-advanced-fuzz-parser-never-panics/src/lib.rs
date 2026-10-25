//! A parser for one `name,qty,note` record read from untrusted bytes.

use std::fmt;

/// The longest name, in bytes.
pub const MAX_NAME: usize = 16;

/// One parsed record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub qty: u32,
    pub note: String,
}

/// Why a record was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// The bytes are not UTF-8; `valid_up_to` is the length of the valid prefix.
    InvalidUtf8 { valid_up_to: usize },
    /// A NUL byte at this byte offset.
    Nul { offset: usize },
    /// Not exactly three comma-separated fields.
    FieldCount { found: usize },
    /// The name is empty.
    EmptyName,
    /// The name is longer than `MAX_NAME` bytes.
    NameTooLong { len: usize },
    /// The quantity is not a decimal `u32`.
    BadQty,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidUtf8 { valid_up_to } => {
                write!(f, "not UTF-8 after byte {valid_up_to}")
            }
            ParseError::Nul { offset } => write!(f, "NUL byte at offset {offset}"),
            ParseError::FieldCount { found } => write!(f, "expected 3 fields, found {found}"),
            ParseError::EmptyName => write!(f, "empty name"),
            ParseError::NameTooLong { len } => write!(f, "name is {len} bytes, max {MAX_NAME}"),
            ParseError::BadQty => write!(f, "qty is not a u32"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parses `name,qty,note` from bytes that came off the network.
/// CLUE: Each `unwrap`, `assert!` and `[i]` below is a promise about the input. Who made that promise?
pub fn parse_record(input: &[u8]) -> Result<Record, ParseError> {
    let text = std::str::from_utf8(input).unwrap();
    assert!(!text.contains('\0'), "NUL in record");
    let fields: Vec<&str> = text.split(',').collect();
    // CLUE: This rejects too many fields. What happens with too few?
    if fields.len() > 3 {
        return Err(ParseError::FieldCount {
            found: fields.len(),
        });
    }
    let name = fields[0];
    if name.is_empty() {
        return Err(ParseError::EmptyName);
    }
    if name.len() > MAX_NAME {
        return Err(ParseError::NameTooLong { len: name.len() });
    }
    let qty = fields[1].parse::<u32>().unwrap();
    Ok(Record {
        name: name.to_string(),
        qty,
        note: fields[2].to_string(),
    })
}
