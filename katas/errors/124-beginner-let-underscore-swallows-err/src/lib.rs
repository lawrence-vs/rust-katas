use std::collections::HashSet;
use std::fmt;
use std::io::{self, Read};

/// CLUE: `read_to_string` returns a `Result`. What does `let _ =` do with it?
pub fn read_all(mut source: impl Read) -> io::Result<String> {
    let mut text = String::new();
    let _ = source.read_to_string(&mut text);
    Ok(text)
}

#[derive(Debug, PartialEq, Eq)]
pub struct DuplicateId(pub u32);

impl fmt::Display for DuplicateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "duplicate id {}", self.0)
    }
}

impl std::error::Error for DuplicateId {}

#[derive(Debug, Default)]
pub struct Table {
    ids: HashSet<u32>,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, id: u32) -> Result<(), DuplicateId> {
        if self.ids.insert(id) {
            Ok(())
        } else {
            Err(DuplicateId(id))
        }
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// CLUE: the caller reads `Ok(n)` as "n rows are in". Is that true when a row failed?
    /// CLUE: the signature already allows an error; which line stops it being used?
    pub fn insert_all(&mut self, ids: &[u32]) -> Result<usize, DuplicateId> {
        let mut inserted = 0;
        for &id in ids {
            let _ = self.insert(id);
            inserted += 1;
        }
        Ok(inserted)
    }
}
