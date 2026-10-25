/// One token of an arithmetic expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Number(u64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

/// The operator token for `c`, if `c` is one. A helper for the lexer, not for callers.
pub fn operator_token(c: char) -> Option<Token> {
    match c {
        '+' => Some(Token::Plus),
        '-' => Some(Token::Minus),
        '*' => Some(Token::Star),
        '/' => Some(Token::Slash),
        '(' => Some(Token::LParen),
        ')' => Some(Token::RParen),
        _ => None,
    }
}
