use kata_async_68::{Venue, confirm_all};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn every_fill_is_confirmed_in_order() {
    let venue = Venue {
        name: "XCH".into(),
        fee_bps: 25,
    };
    let fills: Vec<(u64, u64)> = (1..=200).map(|i| (i, i * 10_000)).collect();
    let out = tokio::spawn(confirm_all(venue, fills)).await.unwrap();
    assert_eq!(out.len(), 200);
    assert_eq!(out[0], "XCH:1:25");
    assert_eq!(out[199], "XCH:200:5000");
}
