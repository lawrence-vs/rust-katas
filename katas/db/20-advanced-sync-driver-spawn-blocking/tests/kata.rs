use std::time::{Duration, Instant};

use kata_db_20::lookup;
use tokio::task::JoinSet;

#[tokio::test]
async fn lookups_do_not_block_the_runtime() {
    let start = Instant::now();
    let mut set = JoinSet::new();
    for id in 0..4 {
        set.spawn(lookup(id));
    }
    let mut users = Vec::new();
    while let Some(u) = set.join_next().await {
        users.push(u.unwrap());
    }
    users.sort();
    assert_eq!(users, ["user-0", "user-1", "user-2", "user-3"]);
    assert!(
        start.elapsed() < Duration::from_millis(300),
        "lookups ran one after another: {:?}",
        start.elapsed()
    );
}
