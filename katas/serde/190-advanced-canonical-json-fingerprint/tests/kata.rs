use kata_serde_190::{FingerprintError, MAX_DEPTH, canonical_json, fingerprint};
use sha2::{Digest, Sha256};

fn canon(body: &str) -> Result<String, FingerprintError> {
    canonical_json(body.as_bytes())
}

fn dup<T>(key: &str) -> Result<T, FingerprintError> {
    Err(FingerprintError::DuplicateKey(key.to_owned()))
}

/// The contract: SHA-256 over (u64 big-endian length, bytes) for each field in order.
fn framed(fields: &[&str]) -> [u8; 32] {
    let mut h = Sha256::new();
    for field in fields {
        h.update((field.len() as u64).to_be_bytes());
        h.update(field.as_bytes());
    }
    h.finalize().into()
}

#[test]
fn key_order_whitespace_and_escapes_give_one_fingerprint() {
    let bodies = [
        r#"{"amount":100,"to":"bob"}"#,
        "{ \"to\" : \"bob\",\n  \"amount\" : 100 }",
        r#"{"to":"\u0062ob","amount":100}"#,
        r#"{"amount":100,"to":"b\u006F\u0062"}"#,
    ];
    for body in bodies {
        assert_eq!(
            canon(body),
            Ok(r#"{"amount":100,"to":"bob"}"#.to_owned()),
            "{body}"
        );
        assert_eq!(
            fingerprint("POST", "/transfers", body.as_bytes()),
            fingerprint("POST", "/transfers", bodies[0].as_bytes()),
            "{body}"
        );
    }
    assert_eq!(canon(r#"["a\/b"]"#), Ok(r#"["a/b"]"#.to_owned()));
}

#[test]
fn a_19_digit_integer_id_is_preserved_exactly() {
    for body in [
        r#"{"id":-9999999999999999999}"#,
        r#"{"id":9999999999999999999}"#,
        r#"{"id":1234567890123456789012345}"#,
    ] {
        assert_eq!(
            canon(body),
            Ok(body.to_owned()),
            "a 19-digit integer id was not preserved exactly"
        );
    }
}

#[test]
fn numbers_are_kept_as_written() {
    assert_eq!(
        canon("[1,1.0,100,1e2,-0]"),
        Ok("[1,1.0,100,1e2,-0]".to_owned())
    );
    assert_ne!(
        fingerprint("POST", "/t", b"1"),
        fingerprint("POST", "/t", b"1.0"),
        "1 and 1.0 must stay different"
    );
}

#[test]
fn a_duplicate_key_is_rejected() {
    assert_eq!(canon(r#"{"a":1,"a":2}"#), dup("a"));
    assert_eq!(canon(r#"{"a":1,"b":2,"a":1}"#), dup("a"));
    assert_eq!(canon(r#"{"a":1,"\u0061":2}"#), dup("a"));
    assert_eq!(canon(r#"{"x":{"k":1,"k":1}}"#), dup("k"));
    assert_eq!(fingerprint("POST", "/t", br#"{"a":1,"a":2}"#), dup("a"));
}

#[test]
fn nesting_deeper_than_32_is_rejected_while_parsing() {
    assert_eq!(MAX_DEPTH, 32);
    let at_limit = format!("{}{}", "[".repeat(32), "]".repeat(32));
    let got = canon(&at_limit);
    assert_eq!(got, Ok(at_limit));
    let too_deep = format!("{}{}", "[".repeat(33), "]".repeat(33));
    assert_eq!(canon(&too_deep), Err(FingerprintError::TooDeep));
    let objects = format!("{}1{}", r#"{"a":"#.repeat(33), "}".repeat(33));
    assert_eq!(canon(&objects), Err(FingerprintError::TooDeep));
    // Never closed: only a check made while parsing sees the depth before the end.
    assert_eq!(canon(&"[".repeat(100_000)), Err(FingerprintError::TooDeep));
}

#[test]
fn method_path_and_body_are_delimited() {
    assert_ne!(
        fingerprint("GET", "/ab", b"{}"),
        fingerprint("GET/", "ab", b"{}"),
        "method and path ran together"
    );
    assert_ne!(
        fingerprint("POST", "/1", b"23"),
        fingerprint("POST", "/12", b"3"),
        "path and body ran together"
    );
}

#[test]
fn the_fingerprint_is_sha256_over_length_prefixed_fields() {
    let want = framed(&["POST", "/transfers", r#"{"amount":100,"to":"bob"}"#]);
    assert_eq!(
        fingerprint("POST", "/transfers", br#"{ "to":"bob", "amount":100 }"#),
        Ok(want)
    );
}

#[test]
fn keys_sort_by_utf8_bytes_not_utf16_units() {
    let (bmp, astral) = ("\u{FF61}", "\u{1F600}");
    // UTF-16 puts the astral key first: its high surrogate 0xD83D < 0xFF61.
    assert!(astral.encode_utf16().lt(bmp.encode_utf16()));
    let body = format!(r#"{{"{astral}":1,"{bmp}":2,"b":3,"a":4}}"#);
    let want = format!(r#"{{"a":4,"b":3,"{bmp}":2,"{astral}":1}}"#);
    assert_eq!(canon(&body), Ok(want));
}

#[test]
fn unicode_is_not_nfc_folded() {
    let composed = "{\"name\":\"\u{e9}\"}";
    let decomposed = "{\"name\":\"e\u{301}\"}";
    assert_eq!(canon(r#"{"name":"\u00e9"}"#), Ok(composed.to_owned()));
    assert_eq!(canon(r#"{"name":"e\u0301"}"#), Ok(decomposed.to_owned()));
    assert_ne!(
        fingerprint("POST", "/t", composed.as_bytes()),
        fingerprint("POST", "/t", decomposed.as_bytes())
    );
}

#[test]
fn control_characters_have_one_escape_form() {
    assert_eq!(
        canon(r#"["\u000A","\n","\u001f","\u00e9","\t"]"#),
        Ok("[\"\\n\",\"\\n\",\"\\u001f\",\"\u{e9}\",\"\\t\"]".to_owned())
    );
}

#[test]
fn invalid_json_is_an_error() {
    for body in [
        "",
        " ",
        "{",
        "[1,]",
        r#"{"a":1,}"#,
        "{1:2}",
        "01",
        "1.",
        "-",
        "tru",
        "nul",
        "[1] [2]",
        r#""\ud800""#,
        "\"a\u{1}\"",
    ] {
        assert_eq!(canon(body), Err(FingerprintError::Invalid), "{body:?}");
    }
    assert_eq!(canonical_json(b"\xff"), Err(FingerprintError::Invalid));
}

#[test]
fn errors_display_their_cause() {
    assert_eq!(FingerprintError::Invalid.to_string(), "invalid JSON");
    assert_eq!(
        FingerprintError::DuplicateKey("a".to_owned()).to_string(),
        "duplicate key \"a\""
    );
    assert_eq!(
        FingerprintError::TooDeep.to_string(),
        "nesting deeper than 32"
    );
    let _: &dyn std::error::Error = &FingerprintError::TooDeep;
}
