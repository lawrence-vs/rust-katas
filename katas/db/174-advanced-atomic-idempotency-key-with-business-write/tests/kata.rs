use kata_db_174::{
    AccountId, IdempotencyKey, PayError, Payment, PaymentId, Payments, Receipt, Snapshot, setup,
};

const A: AccountId = AccountId(1);
const PAY_30: Payment = Payment {
    account: A,
    amount: 30,
};
const RECEIPT: Receipt = Receipt {
    payment: PaymentId(1),
    balance_after: 70,
};
const NOTHING: Snapshot = Snapshot {
    balance: 100,
    payments: 0,
    keys: 0,
};
const PAID: Snapshot = Snapshot {
    balance: 70,
    payments: 1,
    keys: 1,
};

async fn payments() -> Payments {
    let pool = setup().await.expect("setup");
    let p = Payments::new(pool);
    p.open_account(A, 100).await.expect("open account");
    p
}

fn key(k: &str) -> IdempotencyKey {
    IdempotencyKey(k.to_string())
}

#[tokio::test]
async fn a_payment_debits_and_returns_a_receipt() {
    let mut p = payments().await;
    let r = p.pay(&key("k1"), PAY_30).await;
    assert!(
        matches!(&r, Ok(x) if *x == RECEIPT),
        "a payment debits and returns a receipt: {r:?}"
    );
    assert_eq!(
        p.snapshot(A).await.expect("snapshot"),
        PAID,
        "a payment debits and returns a receipt"
    );
}

#[tokio::test]
async fn a_retry_returns_the_stored_receipt() {
    let mut p = payments().await;
    p.pay(&key("k1"), PAY_30).await.expect("first");
    let r = p.pay(&key("k1"), PAY_30).await;
    assert!(
        matches!(&r, Ok(x) if *x == RECEIPT),
        "a retry returns the stored receipt: {r:?}"
    );
    assert_eq!(
        p.snapshot(A).await.expect("snapshot"),
        PAID,
        "a retry returns the stored receipt"
    );
}

#[tokio::test]
async fn two_keys_are_two_payments() {
    let mut p = payments().await;
    p.pay(&key("k1"), PAY_30).await.expect("k1");
    let r = p.pay(&key("k2"), PAY_30).await;
    let want = Receipt {
        payment: PaymentId(2),
        balance_after: 40,
    };
    assert!(
        matches!(&r, Ok(x) if *x == want),
        "two keys are two payments: {r:?}"
    );
    assert_eq!(
        p.snapshot(A).await.expect("snapshot"),
        Snapshot {
            balance: 40,
            payments: 2,
            keys: 2
        },
        "two keys are two payments"
    );
}

#[tokio::test]
async fn a_crash_then_a_retry_pays_exactly_once() {
    let mut fired = 0;
    for n in 1..=10u32 {
        let mut p = payments().await;
        p.crash_after(n);
        match p.pay(&key("k1"), PAY_30).await {
            Ok(r) => {
                assert_eq!(
                    r, RECEIPT,
                    "a crash then a retry pays exactly once (no crash at {n})"
                );
                break;
            }
            Err(PayError::Crashed { after }) => {
                assert_eq!(
                    after, n,
                    "a crash then a retry pays exactly once (crash after statement {n})"
                );
            }
            Err(e) => {
                panic!("a crash then a retry pays exactly once (crash after statement {n}): {e:?}")
            }
        }
        fired += 1;
        let after_crash = p.snapshot(A).await.expect("snapshot");
        assert!(
            after_crash == NOTHING || after_crash == PAID,
            "a crash then a retry pays exactly once (crash after statement {n}): half a payment was left behind: {after_crash:?}"
        );
        for attempt in 1..=2 {
            let r = p.pay(&key("k1"), PAY_30).await;
            assert!(
                matches!(&r, Ok(x) if *x == RECEIPT),
                "a crash then a retry pays exactly once (crash after statement {n}, retry {attempt}): {r:?}"
            );
        }
        assert_eq!(
            p.snapshot(A).await.expect("snapshot"),
            PAID,
            "a crash then a retry pays exactly once (crash after statement {n})"
        );
    }
    assert!(
        fired >= 3,
        "a crash then a retry pays exactly once: only {fired} crash points fired; call after_statement after every write and after the commit"
    );
}

#[tokio::test]
async fn a_declined_payment_leaves_no_key() {
    let mut p = payments().await;
    let big = Payment {
        account: A,
        amount: 500,
    };
    let r = p.pay(&key("k1"), big).await;
    assert!(
        matches!(r, Err(PayError::Declined(AccountId(1)))),
        "a declined payment leaves no key: {r:?}"
    );
    assert_eq!(
        p.snapshot(A).await.expect("snapshot"),
        NOTHING,
        "a declined payment leaves no key"
    );
    p.deposit(A, 1000).await.expect("deposit");
    let r = p.pay(&key("k1"), big).await;
    let want = Receipt {
        payment: PaymentId(1),
        balance_after: 600,
    };
    assert!(
        matches!(&r, Ok(x) if *x == want),
        "a declined payment leaves no key: {r:?}"
    );
    assert_eq!(
        p.snapshot(A).await.expect("snapshot"),
        Snapshot {
            balance: 600,
            payments: 1,
            keys: 1
        },
        "a declined payment leaves no key"
    );
}

#[tokio::test]
async fn the_key_is_unique_in_the_schema() {
    let p = payments().await;
    let insert =
        "INSERT INTO idempotency_keys (key, payment_id, balance_after) VALUES ('raw', 1, 70)";
    let first = sqlx::query(insert).execute(p.pool()).await;
    assert!(first.is_ok(), "the key is unique in the schema: {first:?}");
    let second = sqlx::query(insert).execute(p.pool()).await;
    assert!(
        matches!(&second, Err(sqlx::Error::Database(e)) if e.is_unique_violation()),
        "the key is unique in the schema: a second row with the same key was accepted: {second:?}"
    );
}
