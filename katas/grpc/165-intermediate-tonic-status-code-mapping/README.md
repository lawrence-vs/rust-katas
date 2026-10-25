# 165 grpc (intermediate): a status code per domain error

**Concept:** A gRPC client decides what to do from the status code alone: fix the request, give up, or retry later. One exhaustive `From<DomainError> for Status` gives each domain error its own code and a message safe to show the caller. Internal detail stays in the server's logs.
**Source:** tonic [`Status`](https://docs.rs/tonic/0.14.6/tonic/struct.Status.html) and [`Code`](https://docs.rs/tonic/0.14.6/tonic/enum.Code.html); the [gRPC status codes guide](https://grpc.io/docs/guides/status-codes/); tonic's in-process [mock example](https://github.com/hyperium/tonic/tree/master/examples/src/mock).
**Fails with:** `wrong gRPC status code`
**Vault:** your note on error contracts at a service boundary

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Keep the public API (`DomainError`, `MemoryLedger`, `WalletService`, `LEDGER_DOWN_DETAIL`) and keep one `From<DomainError> for Status` as the only place a domain error becomes a `Status`. None of these in `src/`: `unwrap`, `expect`, `panic!`, `Status::unknown`, a `_` arm in the conversion. No new crates.

## Before you fix it
1. Run `cargo test`. Which test passes, and why does it pass?
2. Read the `message()` the ledger-down test prints. Who could see it?
3. For each variant, predict the code before you open the gRPC guide.

## Answer in the PR
1. For each code you return, would a well-behaved client retry it as is, retry after a change, or never retry? Why?
2. `InsufficientFunds`: why `FailedPrecondition` and not `InvalidArgument` or `Unavailable`?
3. Where does `LEDGER_DOWN_DETAIL` go now, and who still needs it?
4. Why is a `_` arm in the conversion a future bug? What happens when someone adds a variant?
5. When is `Internal` the right code?

## Think further
1. The client's first `Reserve` times out after the server committed it. The retry gets `AlreadyExists`. Is that an error to the caller? What would an idempotent `Reserve` return instead?
2. `NotFound` for an account the caller may not see: does it tell an attacker which accounts exist? When would you return `PermissionDenied` or `NotFound` for both?
3. A client retries `Unavailable` in a tight loop while the ledger restarts. What stops it from making the outage worse? (Kata 97, kata 90.)
4. How would you send the field that failed validation in a form a client can read without parsing the message?

## In the wild
- [gRPC status codes](https://grpc.io/docs/guides/status-codes/): which codes the library generates versus the application, and the `FAILED_PRECONDITION` / `ABORTED` / `UNAVAILABLE` advice.
- [Google AIP-193 errors](https://google.aip.dev/193): error model with machine-readable details.
- [gRPC retry guide](https://grpc.io/docs/guides/retry/): `retryableStatusCodes` in service config.
- Payment and wallet APIs: a duplicate idempotency key returns the first result or a conflict, never a generic 500.

In a review, check:
- `Status::internal(e.to_string())` or `{e:?}` in a handler
- a `_ =>` arm mapping to `internal` or `unknown`
- connection strings, SQL, file paths or stack traces in `Status::message()`
- a client retrying every non-OK code

## Related concepts
### Rich error details (`tonic-types`)
Structured error payloads, such as bad-request field violations, sent alongside the code. Use them when a client must act on which field failed.
```rust
use tonic_types::{ErrorDetails, StatusExt};
let details = ErrorDetails::with_bad_request_violation("page_size", "must be <= 100");
let status = Status::with_error_details(Code::InvalidArgument, "bad request", details);
```

### Interceptors
A function that runs before every handler and can reject the call with its own `Status`. Use one for auth or request metadata checks (issue #167).
```rust
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    req.metadata().get("authorization").ok_or_else(|| Status::unauthenticated("no token"))?;
    Ok(req)
}
```

### Deadlines
A client-side time budget that ends the call with `DeadlineExceeded`. Use one on every call that waits on the network (kata 27).
```rust
let mut request = Request::new(ReserveRequest::default());
request.set_timeout(Duration::from_millis(500));
```
