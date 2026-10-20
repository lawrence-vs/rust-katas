use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

/// Opens the app's one pool, at startup. The in-memory database lives inside this pool; a new `connect` gets a different, empty one.
pub async fn setup() -> sqlx::Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;
    sqlx::query("CREATE TABLE users (name TEXT NOT NULL)")
        .execute(&pool)
        .await?;
    Ok(pool)
}

/// CLUE: `_pool` is passed in and ignored. What does a fresh `connect` cost, and which database does it see?
pub async fn add_user(_pool: &SqlitePool, name: &str) -> sqlx::Result<()> {
    let conn = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS users (name TEXT NOT NULL)")
        .execute(&conn)
        .await?;
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(name)
        .execute(&conn)
        .await?;
    Ok(())
}

/// CLUE: Why does the count come back 0 after two inserts?
pub async fn count_users(_pool: &SqlitePool) -> sqlx::Result<i64> {
    let conn = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS users (name TEXT NOT NULL)")
        .execute(&conn)
        .await?;
    sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&conn)
        .await
}
