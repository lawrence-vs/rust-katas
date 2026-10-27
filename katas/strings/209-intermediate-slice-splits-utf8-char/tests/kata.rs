use kata_strings_209::first_chars;

#[test]
fn takes_ascii_prefix() {
    assert_eq!(first_chars("BTCUSD", 3), "BTC");
}

#[test]
fn keeps_multibyte_char() {
    assert_eq!(first_chars("éTH", 1), "é");
}
