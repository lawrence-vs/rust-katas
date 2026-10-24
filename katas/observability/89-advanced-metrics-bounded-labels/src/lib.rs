//! Counting requests with the `metrics` facade.
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Ok,
    Rejected,
    Failed,
}

impl Outcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Outcome::Ok => "ok",
            Outcome::Rejected => "rejected",
            Outcome::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Read,
    Write,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Read => "read",
            Kind::Write => "write",
        }
    }
}

pub struct Request {
    pub account_id: u64,
    pub kind: Kind,
}

/// Registers names, units and descriptions. Call once at startup.
pub fn describe() {
    metrics::describe_counter!("app_requests_total", "Requests handled.");
    // CLUE: Which unit is this histogram in? Where would a dashboard reading it find out?
    metrics::describe_histogram!("app_request_duration", "Time to handle a request.");
}

/// Records one finished request.
pub fn record_request(req: &Request, _outcome: Outcome, elapsed: Duration) {
    // CLUE: How many distinct values can `account` take? Each one is a new time series to store and query.
    metrics::counter!("app_requests_total", "account" => req.account_id.to_string()).increment(1);
    // CLUE: `as_millis` then `as f64`: what does a Prometheus `_seconds` histogram expect, and what is lost below 1 ms?
    metrics::histogram!("app_request_duration", "account" => req.account_id.to_string())
        .record(elapsed.as_millis() as f64);
}
