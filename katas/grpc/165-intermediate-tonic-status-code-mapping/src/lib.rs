use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("wallet");
}

use pb::wallet_server::Wallet;
use pb::{ReserveReply, ReserveRequest};

/// What the ledger's driver says when it can't connect. Fine in a server log; never in a reply.
pub const LEDGER_DOWN_DETAIL: &str =
    "connect postgres://wallet:hunter2@10.0.0.7:5432/ledger: connection refused";

/// Everything that can go wrong with a reservation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    EmptyAccountId,
    EmptyReservationId,
    ZeroAmount,
    AccountNotFound,
    DuplicateReservation,
    InsufficientFunds,
    /// The ledger can't be reached. The detail is for server logs.
    LedgerUnavailable(String),
    /// A bug on our side, such as a poisoned lock.
    Internal(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::EmptyAccountId => write!(f, "account_id is empty"),
            DomainError::EmptyReservationId => write!(f, "reservation_id is empty"),
            DomainError::ZeroAmount => write!(f, "amount_cents is 0"),
            DomainError::AccountNotFound => write!(f, "account not found"),
            DomainError::DuplicateReservation => write!(f, "reservation id already used"),
            DomainError::InsufficientFunds => write!(f, "insufficient funds"),
            DomainError::LedgerUnavailable(detail) => write!(f, "ledger unavailable: {detail}"),
            DomainError::Internal(detail) => write!(f, "internal: {detail}"),
        }
    }
}

impl std::error::Error for DomainError {}

#[derive(Default)]
struct State {
    free: HashMap<String, u64>,
    reservations: HashSet<String>,
}

/// An in-memory ledger of free funds per account. `set_down(true)` makes it act like an unreachable database.
pub struct MemoryLedger {
    state: Mutex<State>,
    down: AtomicBool,
}

impl MemoryLedger {
    pub fn with_accounts(accounts: &[(&str, u64)]) -> Self {
        let free = accounts
            .iter()
            .map(|&(id, cents)| (id.to_string(), cents))
            .collect();
        Self {
            state: Mutex::new(State {
                free,
                reservations: HashSet::new(),
            }),
            down: AtomicBool::new(false),
        }
    }

    pub fn set_down(&self, down: bool) {
        self.down.store(down, Ordering::SeqCst);
    }

    /// Moves `amount_cents` out of the account's free funds and returns what is left.
    pub fn reserve(
        &self,
        account_id: &str,
        reservation_id: &str,
        amount_cents: u64,
    ) -> Result<u64, DomainError> {
        if self.down.load(Ordering::SeqCst) {
            return Err(DomainError::LedgerUnavailable(
                LEDGER_DOWN_DETAIL.to_string(),
            ));
        }
        let mut state = self
            .state
            .lock()
            .map_err(|_| DomainError::Internal("ledger lock poisoned".to_string()))?;
        let State { free, reservations } = &mut *state;
        let Some(balance) = free.get_mut(account_id) else {
            return Err(DomainError::AccountNotFound);
        };
        if reservations.contains(reservation_id) {
            return Err(DomainError::DuplicateReservation);
        }
        if amount_cents > *balance {
            return Err(DomainError::InsufficientFunds);
        }
        *balance -= amount_cents;
        reservations.insert(reservation_id.to_string());
        Ok(*balance)
    }
}

pub struct WalletService {
    ledger: Arc<MemoryLedger>,
}

impl WalletService {
    pub fn new(ledger: Arc<MemoryLedger>) -> Self {
        Self { ledger }
    }
}

fn reserve(ledger: &MemoryLedger, req: &ReserveRequest) -> Result<u64, DomainError> {
    if req.account_id.is_empty() {
        return Err(DomainError::EmptyAccountId);
    }
    if req.reservation_id.is_empty() {
        return Err(DomainError::EmptyReservationId);
    }
    if req.amount_cents == 0 {
        return Err(DomainError::ZeroAmount);
    }
    ledger.reserve(&req.account_id, &req.reservation_id, req.amount_cents)
}

#[tonic::async_trait]
impl Wallet for WalletService {
    async fn reserve(
        &self,
        request: Request<ReserveRequest>,
    ) -> Result<Response<ReserveReply>, Status> {
        let free_cents = reserve(&self.ledger, request.get_ref())?;
        Ok(Response::new(ReserveReply { free_cents }))
    }
}

/// Turns a domain error into what the client sees.
/// CLUE: A client gets `Internal` for an empty account id and for a ledger outage. Can its retry loop tell "fix your request" from "try again in a second"?
/// CLUE: `{e:?}` prints the whole variant, connection string included. Who reads `Status::message()`, and where should the detail go instead?
/// CLUE: The gRPC status-code guide has a table of codes. Which one says the caller can retry later, and which says the thing already exists?
impl From<DomainError> for Status {
    fn from(e: DomainError) -> Self {
        Status::internal(format!("reserve failed: {e:?}"))
    }
}
