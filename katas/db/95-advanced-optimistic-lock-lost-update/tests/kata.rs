use kata_db_95::{Error, deposit, load, open_account, save, setup};
use tokio::task::JoinSet;

#[tokio::test]
async fn a_stale_save_is_a_conflict() {
    let pool = setup().await.unwrap();
    open_account(&pool, 1, 100).await.unwrap();
    let a = load(&pool, 1).await.unwrap();
    let b = load(&pool, 1).await.unwrap();

    let saved = save(&pool, &a, 110).await.unwrap();
    assert_eq!(saved.version, 1);
    assert!(matches!(save(&pool, &b, 120).await, Err(Error::Conflict)));

    let now = load(&pool, 1).await.unwrap();
    assert_eq!((now.balance, now.version), (110, 1));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_deposits_lose_nothing() {
    let pool = setup().await.unwrap();
    open_account(&pool, 1, 0).await.unwrap();

    let mut tasks = JoinSet::new();
    for _ in 0..16 {
        let pool = pool.clone();
        tasks.spawn(async move { deposit(&pool, 1, 1).await });
    }
    let mut ok = 0;
    while let Some(joined) = tasks.join_next().await {
        match joined.unwrap() {
            Ok(_) => ok += 1,
            Err(e) => assert!(matches!(e, Error::Conflict), "unexpected error: {e}"),
        }
    }

    let now = load(&pool, 1).await.unwrap();
    assert!(ok >= 1, "no deposit succeeded");
    assert_eq!(
        now.balance, ok,
        "balance: a deposit that returned Ok was lost"
    );
    assert_eq!(
        now.version, ok,
        "version: every successful save must bump it"
    );
}

#[tokio::test]
async fn deposit_to_missing_account_is_not_found() {
    let pool = setup().await.unwrap();
    assert!(matches!(
        deposit(&pool, 99, 1).await,
        Err(Error::NotFound(99))
    ));
}
