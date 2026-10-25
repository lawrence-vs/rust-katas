//! Uses only the documented paths. Both layouts must pass it unchanged.
use crate::tokenkit::{Error, Token, tokenize};

#[test]
fn tokenize_splits_numbers_and_operators() {
    use Token::*;
    assert_eq!(
        tokenize("12 + 3*(4 - 1) / 2"),
        Ok(vec![
            Number(12),
            Plus,
            Number(3),
            Star,
            LParen,
            Number(4),
            Minus,
            Number(1),
            RParen,
            Slash,
            Number(2)
        ])
    );
}

#[test]
fn blank_input_gives_no_tokens() {
    assert_eq!(tokenize("  \t\n"), Ok(vec![]));
}

#[test]
fn u64_max_still_fits() {
    assert_eq!(
        tokenize("18446744073709551615"),
        Ok(vec![Token::Number(u64::MAX)])
    );
}

#[test]
fn a_number_past_u64_max_is_an_error() {
    assert_eq!(
        tokenize("1 + 18446744073709551616"),
        Err(Error::NumberTooLarge { offset: 4 })
    );
}

#[test]
fn an_unknown_character_is_an_error_with_its_byte_offset() {
    assert_eq!(
        tokenize("1 % 2"),
        Err(Error::UnexpectedChar {
            offset: 2,
            found: '%'
        })
    );
    assert_eq!(
        tokenize("é+1"),
        Err(Error::UnexpectedChar {
            offset: 0,
            found: 'é'
        })
    );
    assert_eq!(
        tokenize("1+é"),
        Err(Error::UnexpectedChar {
            offset: 2,
            found: 'é'
        })
    );
}

#[test]
fn the_error_reads_well() {
    assert_eq!(
        Error::UnexpectedChar {
            offset: 2,
            found: '%'
        }
        .to_string(),
        "unexpected character '%' at byte 2"
    );
    assert_eq!(
        Error::NumberTooLarge { offset: 0 }.to_string(),
        "number too large at byte 0"
    );
}

#[test]
fn the_error_is_a_std_error() {
    fn is_error<E: std::error::Error + Send + Sync + 'static>() {}
    is_error::<Error>();
}

#[test]
fn tokens_are_plain_values() {
    let t = Token::Plus;
    let copy = t;
    assert_eq!(t, copy);
}

mod prelude {
    use crate::tokenkit::prelude::*;

    #[test]
    fn the_prelude_brings_token_and_tokenize() {
        assert_eq!(tokenize("7"), Ok(vec![Token::Number(7)]));
    }
}
