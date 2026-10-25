use kata_statics_167::*;
use std::sync::Barrier;
use std::thread;

#[test]
fn the_keyword_table_is_built_once_across_threads() {
    thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| {
                for _ in 0..1_000 {
                    assert_eq!(keyword("start"), Some(Keyword::Start));
                }
            });
        }
    });
    assert_eq!(
        table_builds(),
        1,
        "the keyword table is built once per process"
    );
}

#[test]
fn keywords_match_ignoring_case_and_others_are_none() {
    assert_eq!(keyword("start"), Some(Keyword::Start));
    assert_eq!(keyword("STOP"), Some(Keyword::Stop));
    assert_eq!(keyword("Pause"), Some(Keyword::Pause));
    assert_eq!(keyword("resume"), Some(Keyword::Resume));
    assert_eq!(keyword("status"), Some(Keyword::Status));
    assert_eq!(keyword("restart"), None);
    assert_eq!(keyword(""), None);
}

#[test]
fn exactly_one_racing_set_wins_and_the_rest_see_its_value() {
    let barrier = Barrier::new(8);
    let results: Vec<(usize, Result<(), ConfigError>)> = thread::scope(|s| {
        let handles: Vec<_> = (1..=8)
            .map(|n| {
                let b = &barrier;
                s.spawn(move || {
                    b.wait();
                    (n, set_workers(n))
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("thread"))
            .collect()
    });
    let winners: Vec<usize> = results
        .iter()
        .filter(|(_, r)| r.is_ok())
        .map(|(n, _)| *n)
        .collect();
    assert_eq!(winners.len(), 1, "exactly one set_workers call wins");
    let won = winners[0];
    assert_eq!(workers(), won);
    for (n, r) in &results {
        if *n != won {
            assert_eq!(
                *r,
                Err(ConfigError::AlreadySet { current: won }),
                "loser {n}"
            );
        }
    }
    assert_eq!(set_workers(0), Err(ConfigError::ZeroWorkers));
    assert_eq!(
        set_workers(99),
        Err(ConfigError::AlreadySet { current: won })
    );
    assert_eq!(workers(), won);
}

#[test]
fn the_library_has_no_unsafe_block_no_static_mut_and_no_crate() {
    let src = include_str!("../src/lib.rs");
    assert!(!src.contains("unsafe {"), "src/lib.rs has no unsafe block");
    assert!(!src.contains("static mut"), "src/lib.rs has no static mut");
    assert!(!src.contains("once_cell"), "std only: no once_cell");
    assert!(!src.contains(".unwrap()"), "no unwrap in the library");
    assert!(!src.contains(".expect("), "no expect in the library");
    assert!(!src.contains("panic!"), "no panic! in the library");
}
