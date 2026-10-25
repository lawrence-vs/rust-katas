use kata_testing_163::{MAX_NAME, ParseError, Record, parse_record};

/// SplitMix64: a tiny seeded generator, so every run sees the same inputs.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

const SEED: u64 = 163;
const CASES: usize = 10_000;
const SEEDS: &[&[u8]] = &[b"apple,12,fresh", b"pear,0,", b"fig,4294967295,max qty"];

/// A valid record with one to four kinds of damage.
/// CLUE: Each arm builds one kind of garbage. Which line of `parse_record` does each one reach?
fn hostile(rng: &mut SplitMix64) -> Vec<u8> {
    let mut bytes = SEEDS[rng.below(SEEDS.len())].to_vec();
    for _ in 0..1 + rng.below(4) {
        let at = rng.below(bytes.len() + 1);
        match rng.below(6) {
            0 => bytes.truncate(at),
            1 => bytes.insert(at, 0x00),
            2 => bytes.insert(at, 0xFF),
            3 => {
                let len = 1 + rng.below(64);
                bytes.splice(at..at, std::iter::repeat_n(b'9', len));
            }
            4 => bytes.insert(at, b','),
            _ => {
                if at < bytes.len() {
                    bytes[at] = rng.next_u64() as u8;
                }
            }
        }
    }
    bytes
}

/// CLUE: The failing input is printed. Put it in a table case to see which line panics.
#[test]
fn never_panics_on_hostile_bytes() {
    let mut rng = SplitMix64(SEED);
    for i in 0..CASES {
        let input = hostile(&mut rng);
        let outcome = std::panic::catch_unwind(|| parse_record(&input));
        assert!(
            outcome.is_ok(),
            "parse_record panicked on hostile input #{i}: {input:?}"
        );
    }
}

fn ok(name: &str, qty: u32, note: &str) -> Result<Record, ParseError> {
    Ok(Record {
        name: name.to_string(),
        qty,
        note: note.to_string(),
    })
}

#[test]
fn typed_result_for_each_kind_of_input() {
    let long_name = "a".repeat(MAX_NAME + 1);
    let max_name = "a".repeat(MAX_NAME);
    let cases: Vec<(Vec<u8>, Result<Record, ParseError>)> = vec![
        (b"apple,12,fresh".to_vec(), ok("apple", 12, "fresh")),
        (b"pear,0,".to_vec(), ok("pear", 0, "")),
        (b"fig,4294967295,x".to_vec(), ok("fig", u32::MAX, "x")),
        (
            format!("{max_name},1,x").into_bytes(),
            ok(&max_name, 1, "x"),
        ),
        (b"".to_vec(), Err(ParseError::FieldCount { found: 1 })),
        (
            b"apple,12".to_vec(),
            Err(ParseError::FieldCount { found: 2 }),
        ),
        (
            b"a,1,b,c".to_vec(),
            Err(ParseError::FieldCount { found: 4 }),
        ),
        (
            b"ap\xFFple,12,x".to_vec(),
            Err(ParseError::InvalidUtf8 { valid_up_to: 2 }),
        ),
        (
            b"\x00\xFF".to_vec(),
            Err(ParseError::InvalidUtf8 { valid_up_to: 1 }),
        ),
        (
            b"apple,1\x002,x".to_vec(),
            Err(ParseError::Nul { offset: 7 }),
        ),
        (b",12,x".to_vec(), Err(ParseError::EmptyName)),
        (
            format!("{long_name},1,x").into_bytes(),
            Err(ParseError::NameTooLong { len: MAX_NAME + 1 }),
        ),
        (b"apple,4294967296,x".to_vec(), Err(ParseError::BadQty)),
        (b"apple,,x".to_vec(), Err(ParseError::BadQty)),
        (b"apple,1x,x".to_vec(), Err(ParseError::BadQty)),
    ];
    for (input, want) in cases {
        assert_eq!(
            parse_record(&input),
            want,
            "{:?}",
            String::from_utf8_lossy(&input)
        );
    }
}
