use kata_async_17::word_lengths;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn counts_every_word() {
    let words = ["a", "bb", "ccc", "dddd"].map(String::from).to_vec();
    let mut got = word_lengths(words).await;
    got.sort();
    assert_eq!(got, [1, 2, 3, 4]);
}
