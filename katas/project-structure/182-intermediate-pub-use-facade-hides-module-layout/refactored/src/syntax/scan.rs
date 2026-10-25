use super::token::{Token, operator_token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanError {
    UnexpectedChar { offset: usize, found: char },
    NumberTooLarge { offset: usize },
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::UnexpectedChar { offset, found } => {
                write!(f, "unexpected character {found:?} at byte {offset}")
            }
            ScanError::NumberTooLarge { offset } => {
                write!(f, "number too large at byte {offset}")
            }
        }
    }
}

impl std::error::Error for ScanError {}

/// Splits `input` into tokens.
pub fn scan(input: &str) -> Result<Vec<Token>, ScanError> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some((offset, c)) = chars.next() {
        if c.is_whitespace() {
            continue;
        }
        if let Some(digit) = c.to_digit(10) {
            let mut value = u64::from(digit);
            while let Some(&(_, next)) = chars.peek() {
                let Some(d) = next.to_digit(10) else { break };
                value = value
                    .checked_mul(10)
                    .and_then(|v| v.checked_add(u64::from(d)))
                    .ok_or(ScanError::NumberTooLarge { offset })?;
                chars.next();
            }
            tokens.push(Token::Number(value));
        } else if let Some(token) = operator_token(c) {
            tokens.push(token);
        } else {
            return Err(ScanError::UnexpectedChar { offset, found: c });
        }
    }
    Ok(tokens)
}
