use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Account {
    pub id: i64,
    pub balance: i64,
    pub version: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("account {0} not found")]
    NotFound(i64),
    #[error("account was changed by someone else since it was read")]
    Conflict,
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

/// One connection, so the in-memory database is shared; statements from different tasks still interleave on it.
pub async fn setup() -> Result<SqlitePool, Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;
    sqlx::query(
        "CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance INTEGER NOT NULL, version INTEGER NOT NULL DEFAULT 0)",
    )
    .execute(&pool)
    .await?;
    Ok(pool)
}

pub async fn open_account(pool: &SqlitePool, id: i64, balance: i64) -> Result<(), Error> {
    sqlx::query("INSERT INTO accounts (id, balance) VALUES (?, ?)")
        .bind(id)
        .bind(balance)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn load(pool: &SqlitePool, id: i64) -> Result<Account, Error> {
    let row = sqlx::query_as::<_, (i64, i64)>("SELECT balance, version FROM accounts WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    match row {
        Some((balance, version)) => Ok(Account {
            id,
            balance,
            version,
        }),
        None => Err(Error::NotFound(id)),
    }
}

/// Writes `new_balance` for an account you earlier `load`ed as `read`.
///
/// CLUE: Which row does this `WHERE` match if someone saved between your `load` and now?
/// CLUE: What does `execute` return, and what would 0 rows mean?
pub async fn save(pool: &SqlitePool, read: &Account, new_balance: i64) -> Result<Account, Error> {
    sqlx::query("UPDATE accounts SET balance = ? WHERE id = ?")
        .bind(new_balance)
        .bind(read.id)
        .execute(pool)
        .await?;
    Ok(Account {
        balance: new_balance,
        ..*read
    })
}

/// CLUE: Two callers both got `balance = 100`. What should the second one do?
pub async fn deposit(pool: &SqlitePool, id: i64, amount: i64) -> Result<Account, Error> {
    let a = load(pool, id).await?;
    save(pool, &a, a.balance + amount).await
}
