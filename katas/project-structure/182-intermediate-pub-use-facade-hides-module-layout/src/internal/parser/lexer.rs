use super::errors::LexError;
use super::tokens::{Token, operator_token};

/// Splits `input` into tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
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
                    .ok_or(LexError::NumberTooLarge { offset })?;
                chars.next();
            }
            tokens.push(Token::Number(value));
        } else if let Some(token) = operator_token(c) {
            tokens.push(token);
        } else {
            return Err(LexError::UnexpectedChar { offset, found: c });
        }
    }
    Ok(tokens)
}
