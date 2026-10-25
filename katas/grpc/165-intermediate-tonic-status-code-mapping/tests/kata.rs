use std::sync::Arc;

use hyper_util::rt::TokioIo;
use kata_grpc_165::pb::ReserveRequest;
use kata_grpc_165::pb::wallet_client::WalletClient;
use kata_grpc_165::pb::wallet_server::WalletServer;
use kata_grpc_165::{DomainError, LEDGER_DOWN_DETAIL, MemoryLedger, WalletService};
use tonic::transport::{Channel, Endpoint, Server, Uri};
use tonic::{Code, Status};

/// A real tonic server and client joined by an in-memory pipe: HTTP/2 and gRPC framing, no socket.
async fn connect(ledger: Arc<MemoryLedger>) -> WalletClient<Channel> {
    let (client_io, server_io) = tokio::io::duplex(64 * 1024);
    tokio::spawn(
        Server::builder()
            .add_service(WalletServer::new(WalletService::new(ledger)))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server_io))),
    );
    let mut client_io = Some(client_io);
    let channel = Endpoint::from_static("http://in-process")
        .connect_with_connector(tower::service_fn(move |_: Uri| {
            let io = client_io.take();
            async move {
                io.map(TokioIo::new)
                    .ok_or_else(|| std::io::Error::other("in-process client already connected"))
            }
        }))
        .await
        .unwrap();
    WalletClient::new(channel)
}

fn ledger() -> Arc<MemoryLedger> {
    Arc::new(MemoryLedger::with_accounts(&[("acc-1", 1_000)]))
}

fn req(account_id: &str, reservation_id: &str, amount_cents: u64) -> ReserveRequest {
    ReserveRequest {
        account_id: account_id.into(),
        reservation_id: reservation_id.into(),
        amount_cents,
    }
}

fn assert_status(status: &Status, code: Code, message: &str, case: &str) {
    assert_eq!(status.code(), code, "wrong gRPC status code for {case}");
    for secret in [
        "hunter2",
        "10.0.0.7",
        "postgres://",
        "LedgerUnavailable",
        "poisoned",
    ] {
        assert!(
            !status.message().contains(secret),
            "status message leaks {secret:?} for {case}: {:?}",
            status.message()
        );
    }
    assert_eq!(status.message(), message, "wrong status message for {case}");
}

#[tokio::test]
async fn reserve_within_free_funds_returns_what_is_left() {
    let mut client = connect(ledger()).await;
    let first = client.reserve(req("acc-1", "r1", 250)).await.unwrap();
    assert_eq!(first.into_inner().free_cents, 750);
    let second = client.reserve(req("acc-1", "r2", 750)).await.unwrap();
    assert_eq!(second.into_inner().free_cents, 0);
}

#[tokio::test]
async fn empty_account_id_is_invalid_argument() {
    let mut client = connect(ledger()).await;
    let status = client.reserve(req("", "r1", 100)).await.unwrap_err();
    assert_status(
        &status,
        Code::InvalidArgument,
        "account_id must not be empty",
        "EmptyAccountId",
    );
}

#[tokio::test]
async fn empty_reservation_id_is_invalid_argument() {
    let mut client = connect(ledger()).await;
    let status = client.reserve(req("acc-1", "", 100)).await.unwrap_err();
    assert_status(
        &status,
        Code::InvalidArgument,
        "reservation_id must not be empty",
        "EmptyReservationId",
    );
}

#[tokio::test]
async fn zero_amount_is_invalid_argument() {
    let mut client = connect(ledger()).await;
    let status = client.reserve(req("acc-1", "r1", 0)).await.unwrap_err();
    assert_status(
        &status,
        Code::InvalidArgument,
        "amount_cents must be greater than 0",
        "ZeroAmount",
    );
}

#[tokio::test]
async fn unknown_account_is_not_found() {
    let mut client = connect(ledger()).await;
    let status = client.reserve(req("acc-404", "r1", 100)).await.unwrap_err();
    assert_status(
        &status,
        Code::NotFound,
        "account not found",
        "AccountNotFound",
    );
}

#[tokio::test]
async fn reused_reservation_id_is_already_exists() {
    let mut client = connect(ledger()).await;
    client.reserve(req("acc-1", "r1", 100)).await.unwrap();
    let status = client.reserve(req("acc-1", "r1", 100)).await.unwrap_err();
    assert_status(
        &status,
        Code::AlreadyExists,
        "reservation already exists",
        "DuplicateReservation",
    );
}

#[tokio::test]
async fn reserving_more_than_free_is_failed_precondition() {
    let mut client = connect(ledger()).await;
    let status = client.reserve(req("acc-1", "r1", 1_001)).await.unwrap_err();
    assert_status(
        &status,
        Code::FailedPrecondition,
        "insufficient funds",
        "InsufficientFunds",
    );
}

#[tokio::test]
async fn ledger_down_is_unavailable_and_hides_the_detail() {
    let ledger = ledger();
    let mut client = connect(ledger.clone()).await;
    ledger.set_down(true);
    let status = client.reserve(req("acc-1", "r1", 100)).await.unwrap_err();
    assert_status(
        &status,
        Code::Unavailable,
        "ledger unavailable",
        "LedgerUnavailable",
    );
}

#[test]
fn every_domain_error_maps_to_one_status() {
    let cases = [
        (
            DomainError::EmptyAccountId,
            Code::InvalidArgument,
            "account_id must not be empty",
        ),
        (
            DomainError::EmptyReservationId,
            Code::InvalidArgument,
            "reservation_id must not be empty",
        ),
        (
            DomainError::ZeroAmount,
            Code::InvalidArgument,
            "amount_cents must be greater than 0",
        ),
        (
            DomainError::AccountNotFound,
            Code::NotFound,
            "account not found",
        ),
        (
            DomainError::DuplicateReservation,
            Code::AlreadyExists,
            "reservation already exists",
        ),
        (
            DomainError::InsufficientFunds,
            Code::FailedPrecondition,
            "insufficient funds",
        ),
        (
            DomainError::LedgerUnavailable(LEDGER_DOWN_DETAIL.into()),
            Code::Unavailable,
            "ledger unavailable",
        ),
        (
            DomainError::Internal("ledger lock poisoned".into()),
            Code::Internal,
            "internal error",
        ),
    ];
    for (err, code, message) in cases {
        let case = format!("{err:?}");
        let status = Status::from(err);
        assert_status(&status, code, message, &case);
    }
}
