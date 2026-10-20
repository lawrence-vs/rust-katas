use kata_async_16::bump_many;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn every_bump_counts() {
    assert_eq!(bump_many(1_000).await, 1_000);
}
