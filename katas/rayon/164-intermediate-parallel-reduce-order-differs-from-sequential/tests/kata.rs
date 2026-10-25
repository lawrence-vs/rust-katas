use kata_rayon_164::*;
use rayon::ThreadPoolBuilder;

const THREADS: [usize; 5] = [1, 2, 3, 4, 8];

fn pool(threads: usize) -> rayon::ThreadPool {
    ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("pool")
}

/// `n` entries with amounts in -100_000..=100_000 cents, from a seeded xorshift64.
fn ledger(n: u32, seed: u64) -> Vec<Entry> {
    let mut s = seed;
    (0..n)
        .map(|i| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            let amount = i64::try_from(s % 200_001).expect("fits") - 100_000;
            Entry {
                id: EntryId(i),
                amount: Cents(amount),
            }
        })
        .collect()
}

fn entries(amounts: &[i64]) -> Vec<Entry> {
    amounts
        .iter()
        .zip(0u32..)
        .map(|(&a, i)| Entry {
            id: EntryId(i),
            amount: Cents(a),
        })
        .collect()
}

fn oracle_closing(opening: Cents, entries: &[Entry]) -> Option<Cents> {
    let total = entries
        .iter()
        .fold(i128::from(opening.0), |acc, e| acc + i128::from(e.amount.0));
    i64::try_from(total).ok().map(Cents)
}

fn oracle_statement(entries: &[Entry]) -> String {
    entries.iter().map(line).collect::<Vec<_>>().join("\n")
}

#[test]
fn closing_balance_matches_the_sequential_oracle_on_every_pool() {
    let book = ledger(10_000, 0x5EED_0164);
    let opening = Cents(50_000);
    let want = oracle_closing(opening, &book);
    for threads in THREADS {
        let got = pool(threads).install(|| closing_balance(opening, &book));
        assert_eq!(
            got, want,
            "parallel result equals the sequential oracle ({threads} threads)"
        );
    }
}

#[test]
fn statement_matches_the_sequential_oracle_on_every_pool() {
    let book = ledger(10_000, 0x5EED_0164);
    let want = oracle_statement(&book);
    for threads in THREADS {
        let got = pool(threads).install(|| statement(&book));
        assert_eq!(
            got, want,
            "parallel result equals the sequential oracle ({threads} threads)"
        );
    }
}

#[test]
fn closing_balance_counts_the_opening_balance_once() {
    let book = entries(&[200, -50]);
    let got = pool(1).install(|| closing_balance(Cents(1_000), &book));
    assert_eq!(got, Some(Cents(1_150)));
}

#[test]
fn an_empty_ledger_closes_at_the_opening_balance() {
    assert_eq!(closing_balance(Cents(-7), &[]), Some(Cents(-7)));
    assert_eq!(statement(&[]), "");
}

#[test]
fn a_partial_sum_out_of_range_is_fine_if_the_total_fits() {
    let book = entries(&[i64::MAX, 1, -1]);
    for threads in THREADS {
        let got = pool(threads).install(|| closing_balance(Cents(0), &book));
        assert_eq!(got, Some(Cents(i64::MAX)), "{threads} threads");
    }
}

#[test]
fn a_closing_balance_out_of_range_is_none() {
    for threads in THREADS {
        let p = pool(threads);
        assert_eq!(
            p.install(|| closing_balance(Cents(i64::MAX), &entries(&[1]))),
            None
        );
        assert_eq!(
            p.install(|| closing_balance(Cents(i64::MIN), &entries(&[-1]))),
            None
        );
    }
}

#[test]
fn statement_lists_entries_in_order() {
    let book = entries(&[500, -1250, 0]);
    for threads in THREADS {
        let got = pool(threads).install(|| statement(&book));
        assert_eq!(got, "0 500\n1 -1250\n2 0", "{threads} threads");
    }
}
