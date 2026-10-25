#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    UnexpectedChar { offset: usize, found: char },
    NumberTooLarge { offset: usize },
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnexpectedChar { offset, found } => {
                write!(f, "unexpected character {found:?} at byte {offset}")
            }
            LexError::NumberTooLarge { offset } => {
                write!(f, "number too large at byte {offset}")
            }
        }
    }
}

impl std::error::Error for LexError {}
