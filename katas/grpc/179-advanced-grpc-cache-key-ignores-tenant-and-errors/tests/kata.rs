use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::SeqCst};
use std::time::Duration;

use kata_grpc_179::pb::catalogue_client::CatalogueClient;
use kata_grpc_179::pb::catalogue_server::{Catalogue, CatalogueServer};
use kata_grpc_179::pb::{GetItemRequest, Item, SetPriceRequest};
use kata_grpc_179::{CachedCatalogue, MAX_ENTRIES, TENANT_HEADER};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{Code, Request, Response, Status};

#[derive(Default)]
struct Fake {
    get_calls: AtomicUsize,
    set_calls: AtomicUsize,
    fail_next: AtomicBool,
    delay_ms: AtomicU64,
}

#[derive(Clone)]
struct FakeCatalogue(Arc<Fake>);

fn tenant_of<T>(r: &Request<T>) -> Result<String, Status> {
    r.metadata()
        .get(TENANT_HEADER)
        .ok_or_else(|| Status::unauthenticated("no tenant"))?
        .to_str()
        .map(str::to_owned)
        .map_err(|_| Status::unauthenticated("no tenant"))
}

#[tonic::async_trait]
impl Catalogue for FakeCatalogue {
    async fn get_item(&self, req: Request<GetItemRequest>) -> Result<Response<Item>, Status> {
        self.0.get_calls.fetch_add(1, SeqCst);
        let delay = self.0.delay_ms.load(SeqCst);
        if delay > 0 {
            tokio::time::sleep(Duration::from_millis(delay)).await;
        }
        if self.0.fail_next.swap(false, SeqCst) {
            return Err(Status::unavailable("backend down"));
        }
        let tenant = tenant_of(&req)?;
        let price_cents = if tenant == "b" { 200 } else { 100 };
        let body = req.into_inner();
        Ok(Response::new(Item {
            sku: body.sku,
            tenant,
            currency: body.currency,
            price_cents,
        }))
    }

    async fn set_price(&self, req: Request<SetPriceRequest>) -> Result<Response<Item>, Status> {
        self.0.set_calls.fetch_add(1, SeqCst);
        let tenant = tenant_of(&req)?;
        let body = req.into_inner();
        Ok(Response::new(Item {
            sku: body.sku,
            tenant,
            currency: String::new(),
            price_cents: body.price_cents,
        }))
    }
}

/// Starts the fake on a random local port and returns the cache in front of it.
async fn start() -> (CachedCatalogue, Arc<Fake>) {
    let fake = Arc::new(Fake::default());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(CatalogueServer::new(FakeCatalogue(fake.clone())))
            .serve_with_incoming(TcpListenerStream::new(listener)),
    );
    let client = CatalogueClient::connect(format!("http://{addr}"))
        .await
        .unwrap();
    (CachedCatalogue::new(client), fake)
}

fn get(tenant: &str, sku: &str, currency: &str) -> Request<GetItemRequest> {
    let mut r = Request::new(GetItemRequest {
        sku: sku.into(),
        currency: currency.into(),
    });
    r.metadata_mut()
        .insert(TENANT_HEADER, tenant.parse().unwrap());
    r
}

fn set(tenant: &str, sku: &str, price_cents: u64) -> Request<SetPriceRequest> {
    let mut r = Request::new(SetPriceRequest {
        sku: sku.into(),
        price_cents,
    });
    r.metadata_mut()
        .insert(TENANT_HEADER, tenant.parse().unwrap());
    r
}

#[tokio::test]
async fn same_request_twice_is_one_backend_call() {
    let (cache, fake) = start().await;
    let first = cache.get_item(get("a", "sku-1", "USD")).await.unwrap();
    let second = cache.get_item(get("a", "sku-1", "USD")).await.unwrap();
    assert_eq!(first.price_cents, 100);
    assert_eq!(second.price_cents, 100);
    assert_eq!(fake.get_calls.load(SeqCst), 1);
}

#[tokio::test]
async fn tenants_do_not_share_cached_items() {
    let (cache, fake) = start().await;
    let a = cache.get_item(get("a", "sku-1", "USD")).await.unwrap();
    assert_eq!(a.tenant, "a");
    assert_eq!(a.price_cents, 100);
    let b = cache.get_item(get("b", "sku-1", "USD")).await.unwrap();
    assert_eq!(b.tenant, "b", "tenant b got tenant a's cached item");
    assert_eq!(b.price_cents, 200);
    assert_eq!(fake.get_calls.load(SeqCst), 2);
}

#[tokio::test]
async fn another_currency_is_another_entry() {
    let (cache, fake) = start().await;
    cache.get_item(get("a", "sku-1", "USD")).await.unwrap();
    let eur = cache.get_item(get("a", "sku-1", "EUR")).await.unwrap();
    assert_eq!(
        eur.currency, "EUR",
        "a request with another currency got the cached item"
    );
    assert_eq!(fake.get_calls.load(SeqCst), 2);
}

#[tokio::test]
async fn an_error_is_not_cached() {
    let (cache, fake) = start().await;
    fake.fail_next.store(true, SeqCst);
    let first = cache.get_item(get("a", "sku-1", "USD")).await;
    assert_eq!(first.unwrap_err().code(), Code::Unavailable);
    let second = cache.get_item(get("a", "sku-1", "USD")).await;
    assert!(
        second.is_ok(),
        "a cached error was replayed after the backend recovered: {second:?}"
    );
    assert_eq!(fake.get_calls.load(SeqCst), 2);
}

#[tokio::test]
async fn set_price_always_reaches_the_backend() {
    let (cache, fake) = start().await;
    cache.get_item(get("a", "sku-1", "USD")).await.unwrap();
    let r = cache.set_price(set("a", "sku-1", 150)).await.unwrap();
    assert_eq!(
        fake.set_calls.load(SeqCst),
        1,
        "set_price was answered from the cache"
    );
    assert_eq!(r.price_cents, 150);
    let r = cache.set_price(set("a", "sku-1", 175)).await.unwrap();
    assert_eq!(fake.set_calls.load(SeqCst), 2);
    assert_eq!(r.price_cents, 175);
}

#[tokio::test]
async fn a_hit_is_served_within_a_short_deadline() {
    let (cache, fake) = start().await;
    cache.get_item(get("a", "sku-1", "USD")).await.unwrap();
    fake.delay_ms.store(500, SeqCst);
    let mut r = get("a", "sku-1", "USD");
    r.set_timeout(Duration::from_millis(100));
    let hit = cache.get_item(r).await.unwrap();
    assert_eq!(hit.price_cents, 100);
    assert_eq!(fake.get_calls.load(SeqCst), 1);
}

#[tokio::test]
async fn a_slow_miss_honours_the_deadline() {
    let (cache, fake) = start().await;
    fake.delay_ms.store(500, SeqCst);
    let mut r = get("a", "sku-1", "USD");
    r.set_timeout(Duration::from_millis(100));
    let started = tokio::time::Instant::now();
    let res = cache.get_item(r).await;
    let elapsed = started.elapsed();
    assert!(
        res.is_err() && elapsed < Duration::from_millis(400),
        "a slow miss ignored the caller's deadline: {res:?} after {elapsed:?}"
    );
}

#[tokio::test]
async fn cache_stays_bounded() {
    let (cache, _fake) = start().await;
    for i in 0..MAX_ENTRIES + 8 {
        let sku = format!("sku-{i}");
        assert!(cache.get_item(get("a", &sku, "USD")).await.is_ok());
    }
    assert_eq!(cache.len(), MAX_ENTRIES);
}
