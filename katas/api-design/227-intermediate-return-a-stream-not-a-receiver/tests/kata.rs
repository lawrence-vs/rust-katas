use tokio_stream::StreamExt;

#[tokio::test]
async fn yields_in_order() {
    let v: Vec<u32> = kata_api_design_227::events().collect().await;
    assert_eq!(v, vec![1, 2, 3]);
}
