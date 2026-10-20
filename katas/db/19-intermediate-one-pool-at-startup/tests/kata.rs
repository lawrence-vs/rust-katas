use kata_db_19::{add_user, count_users, setup};

#[tokio::test]
async fn users_persist_across_calls() {
    let pool = setup().await.unwrap();
    add_user(&pool, "ada").await.unwrap();
    add_user(&pool, "grace").await.unwrap();
    assert_eq!(count_users(&pool).await.unwrap(), 2);
}
