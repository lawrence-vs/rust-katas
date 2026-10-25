use std::collections::HashMap;
use std::future::Future;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use tonic::transport::Channel;
use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("catalogue");
}

use pb::catalogue_client::CatalogueClient;
use pb::{GetItemRequest, Item, SetPriceRequest};

/// The metadata key that names the caller's tenant.
pub const TENANT_HEADER: &str = "x-tenant-id";
/// How long a cached reply is served.
pub const CACHE_TTL: Duration = Duration::from_secs(30);
/// The most replies the cache holds.
pub const MAX_ENTRIES: usize = 16;

struct Cached {
    stored_at: Instant,
    reply: Result<Item, Status>,
}

/// A catalogue client that answers repeated calls from memory.
pub struct CachedCatalogue {
    client: CatalogueClient<Channel>,
    entries: Mutex<HashMap<String, Cached>>,
}

impl CachedCatalogue {
    pub fn new(client: CatalogueClient<Channel>) -> Self {
        Self {
            client,
            entries: Mutex::new(HashMap::new()),
        }
    }

    /// Replies held now.
    pub fn len(&self) -> usize {
        self.entries.lock().map_or(0, |e| e.len())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub async fn get_item(&self, request: Request<GetItemRequest>) -> Result<Item, Status> {
        let key = request.get_ref().sku.clone();
        let mut client = self.client.clone();
        self.cached(
            key,
            request,
            move |r| async move { client.get_item(r).await },
        )
        .await
    }

    pub async fn set_price(&self, request: Request<SetPriceRequest>) -> Result<Item, Status> {
        let key = request.get_ref().sku.clone();
        let mut client = self.client.clone();
        self.cached(
            key,
            request,
            move |r| async move { client.set_price(r).await },
        )
        .await
    }

    /// CLUE: Tenant b asks for the same sku as tenant a. What in this key tells their replies apart?
    /// CLUE: The backend failed once and came back. What does `store` keep, and what does the next caller get?
    /// CLUE: `set_price` comes through here too. What should a cache do with a call that changes data?
    /// CLUE: The caller set a 100 ms deadline. Which part of the request carries it, and is it still there on the request you forward?
    async fn cached<M, F, Fut>(
        &self,
        key: String,
        request: Request<M>,
        call: F,
    ) -> Result<Item, Status>
    where
        F: FnOnce(Request<M>) -> Fut,
        Fut: Future<Output = Result<Response<Item>, Status>>,
    {
        if let Some(reply) = self.lookup(&key) {
            return reply;
        }
        // Forward only the tenant: the backend needs it, the rest is per-call noise.
        let tenant = request.metadata().get(TENANT_HEADER).cloned();
        let mut forwarded = Request::new(request.into_inner());
        if let Some(t) = tenant {
            forwarded.metadata_mut().insert(TENANT_HEADER, t);
        }
        let reply = call(forwarded).await.map(Response::into_inner);
        self.store(key, reply.clone());
        reply
    }

    fn lookup(&self, key: &str) -> Option<Result<Item, Status>> {
        let entries = self.entries.lock().ok()?;
        let hit = entries.get(key)?;
        (hit.stored_at.elapsed() < CACHE_TTL).then(|| hit.reply.clone())
    }

    fn store(&self, key: String, reply: Result<Item, Status>) {
        let Ok(mut entries) = self.entries.lock() else {
            return;
        };
        if entries.len() >= MAX_ENTRIES && !entries.contains_key(&key) {
            entries.retain(|_, e| e.stored_at.elapsed() < CACHE_TTL);
            if entries.len() >= MAX_ENTRIES {
                return;
            }
        }
        entries.insert(
            key,
            Cached {
                stored_at: Instant::now(),
                reply,
            },
        );
    }
}
