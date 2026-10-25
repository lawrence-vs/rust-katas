use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccountId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaymentId(pub i64);

/// Sent by the client with every attempt of one request; a retry carries the same key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdempotencyKey(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Payment {
    pub account: AccountId,
    pub amount: i64,
}

/// What the client gets back; a retry must get the same one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Receipt {
    pub payment: PaymentId,
    pub balance_after: i64,
}

/// One account's balance plus the row counts of `payments` and `idempotency_keys`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snapshot {
    pub balance: i64,
    pub payments: i64,
    pub keys: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum PayError {
    #[error("database error")]
    Db(#[from] sqlx::Error),
    #[error("account {0:?} is unknown or short of funds")]
    Declined(AccountId),
    #[error("simulated crash after statement {after}")]
    Crashed { after: u32 },
}

/// CLUE: Nothing in this schema stops two rows with the same `key`. Which part of a database exists to say no?
pub async fn setup() -> Result<SqlitePool, PayError> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect("sqlite::memory:")
        .await?;
    sqlx::query("CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance INTEGER NOT NULL)")
        .execute(&pool)
        .await?;
    sqlx::query(
        "CREATE TABLE payments (id INTEGER PRIMARY KEY, account INTEGER NOT NULL, amount INTEGER NOT NULL)",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE idempotency_keys (key TEXT NOT NULL, payment_id INTEGER, balance_after INTEGER)",
    )
    .execute(&pool)
    .await?;
    Ok(pool)
}

pub struct Payments {
    pool: SqlitePool,
    crash_after: Option<u32>,
    statements: u32,
}

impl Payments {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            crash_after: None,
            statements: 0,
        }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Arms one crash: the next `pay` fails at its `n`th `after_statement`, then the crash disarms.
    pub fn crash_after(&mut self, n: u32) {
        self.crash_after = Some(n);
    }

    /// Call after every write statement in `pay`, and after the commit.
    fn after_statement(&mut self) -> Result<(), PayError> {
        self.statements = self.statements.saturating_add(1);
        if self.crash_after == Some(self.statements) {
            self.crash_after = None;
            return Err(PayError::Crashed {
                after: self.statements,
            });
        }
        Ok(())
    }

    pub async fn open_account(&self, account: AccountId, balance: i64) -> Result<(), PayError> {
        sqlx::query("INSERT INTO accounts (id, balance) VALUES (?, ?)")
            .bind(account.0)
            .bind(balance)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn deposit(&self, account: AccountId, amount: i64) -> Result<(), PayError> {
        sqlx::query("UPDATE accounts SET balance = balance + ? WHERE id = ?")
            .bind(amount)
            .bind(account.0)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn snapshot(&self, account: AccountId) -> Result<Snapshot, PayError> {
        let balance = sqlx::query_scalar::<_, i64>("SELECT balance FROM accounts WHERE id = ?")
            .bind(account.0)
            .fetch_one(&self.pool)
            .await?;
        let payments = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM payments")
            .fetch_one(&self.pool)
            .await?;
        let keys = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM idempotency_keys")
            .fetch_one(&self.pool)
            .await?;
        Ok(Snapshot {
            balance,
            payments,
            keys,
        })
    }

    /// CLUE: Four statements, four commits. Crash at any `after_statement` line: what does the next request with this key find in each table?
    /// CLUE: A key row with no receipt. Did the earlier attempt never start, or stop halfway, and can this code tell?
    /// CLUE: The client's retry after a lost response must get the receipt it missed. Where is that receipt kept, and when is it written?
    pub async fn pay(
        &mut self,
        key: &IdempotencyKey,
        payment: Payment,
    ) -> Result<Receipt, PayError> {
        self.statements = 0;
        let stored = sqlx::query_as::<_, (Option<i64>, Option<i64>)>(
            "SELECT payment_id, balance_after FROM idempotency_keys WHERE key = ?",
        )
        .bind(key.0.as_str())
        .fetch_optional(&self.pool)
        .await?;
        if let Some((Some(payment_id), Some(balance_after))) = stored {
            return Ok(Receipt {
                payment: PaymentId(payment_id),
                balance_after,
            });
        }
        sqlx::query("INSERT INTO idempotency_keys (key) VALUES (?)")
            .bind(key.0.as_str())
            .execute(&self.pool)
            .await?;
        self.after_statement()?;
        let balance_after = sqlx::query_scalar::<_, i64>(
            "UPDATE accounts SET balance = balance - ? WHERE id = ? AND balance >= ? RETURNING balance",
        )
        .bind(payment.amount)
        .bind(payment.account.0)
        .bind(payment.amount)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(PayError::Declined(payment.account))?;
        self.after_statement()?;
        let payment_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO payments (account, amount) VALUES (?, ?) RETURNING id",
        )
        .bind(payment.account.0)
        .bind(payment.amount)
        .fetch_one(&self.pool)
        .await?;
        self.after_statement()?;
        sqlx::query("UPDATE idempotency_keys SET payment_id = ?, balance_after = ? WHERE key = ?")
            .bind(payment_id)
            .bind(balance_after)
            .bind(key.0.as_str())
            .execute(&self.pool)
            .await?;
        self.after_statement()?;
        Ok(Receipt {
            payment: PaymentId(payment_id),
            balance_after,
        })
    }
}
