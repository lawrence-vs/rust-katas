use std::num::NonZeroU32;

use kata_events_170::{
    Applied, Group, GroupError, Ledger, LedgerError, Log, LogError, MemberId, MemoryLog, Offset,
    PartitionId, Record,
};

const M1: MemberId = MemberId(1);
const M2: MemberId = MemberId(2);
const M3: MemberId = MemberId(3);
const P0: PartitionId = PartitionId(0);
const P1: PartitionId = PartitionId(1);
const P2: PartitionId = PartitionId(2);

const PAYMENTS: [(u32, &str, &str, i64); 3] = [
    (0, "pay-1", "acct-a", 100),
    (0, "pay-2", "acct-a", 200),
    (0, "pay-3", "acct-a", 300),
];

fn log_with(partitions: u32, records: &[(u32, &str, &str, i64)]) -> MemoryLog {
    let mut log = MemoryLog::new(NonZeroU32::new(partitions).unwrap());
    for &(p, payment_id, account, amount) in records {
        log.append(PartitionId(p), payment_id, account, amount)
            .unwrap();
    }
    log
}

fn record(
    partition: u32,
    offset: u64,
    payment_id: &str,
    account: &str,
    amount_cents: i64,
) -> Record {
    Record {
        partition: PartitionId(partition),
        offset: Offset(offset),
        payment_id: payment_id.into(),
        account: account.into(),
        amount_cents,
    }
}

fn offsets(records: &[Record]) -> Vec<Offset> {
    records.iter().map(|r| r.offset).collect()
}

fn apply_all(ledger: &mut Ledger, records: &[Record]) -> Vec<Applied> {
    records.iter().map(|r| ledger.apply(r).unwrap()).collect()
}

#[test]
fn memory_log_appends_fetches_and_commits() {
    let mut log = MemoryLog::new(NonZeroU32::new(2).unwrap());

    assert_eq!(log.append(P0, "pay-1", "acct-a", 100), Ok(Offset(0)));
    assert_eq!(log.append(P0, "pay-2", "acct-a", 200), Ok(Offset(1)));
    assert_eq!(log.append(P1, "pay-3", "acct-b", 5), Ok(Offset(0)));
    assert_eq!(
        log.append(P2, "pay-4", "acct-b", 5),
        Err(LogError::NoSuchPartition(P2))
    );

    let ids: Vec<String> = log
        .fetch(P0, Offset(1), 10)
        .into_iter()
        .map(|r| r.payment_id)
        .collect();
    assert_eq!(ids, vec!["pay-2"]);
    assert_eq!(log.fetch(P0, Offset(0), 1).len(), 1);
    assert!(log.fetch(P0, Offset(5), 10).is_empty());

    assert_eq!(log.committed(P0), Offset(0));
    assert_eq!(log.commit(P0, Offset(2)), Ok(()));
    assert_eq!(log.committed(P0), Offset(2));
    assert_eq!(
        log.commit(P2, Offset(0)),
        Err(LogError::NoSuchPartition(P2))
    );
}

#[test]
fn one_member_owns_every_partition() {
    let mut group = Group::new(log_with(3, &[]));
    group.join(M1);

    assert_eq!(group.assignment(M1), vec![P0, P1, P2]);
}

#[test]
fn partitions_are_spread_round_robin_by_member_id() {
    let mut group = Group::new(log_with(3, &[]));
    group.join(M2);
    group.join(M1);

    assert_eq!(group.assignment(M1), vec![P0, P2]);
    assert_eq!(group.assignment(M2), vec![P1]);
}

#[test]
fn a_join_and_a_leave_both_rebalance() {
    let mut group = Group::new(log_with(2, &[]));

    group.join(M1);
    assert_eq!(group.assignment(M1), vec![P0, P1]);

    group.join(M2);
    assert_eq!(group.assignment(M1), vec![P0]);
    assert_eq!(group.assignment(M2), vec![P1]);

    group.leave(M2);
    assert_eq!(group.assignment(M1), vec![P0, P1]);
    assert_eq!(group.assignment(M2), Vec::<PartitionId>::new());
}

#[test]
fn consumers_beyond_the_partition_count_sit_idle() {
    let mut group = Group::new(log_with(2, &[]));
    group.join(M1);
    group.join(M2);
    group.join(M3);

    assert_eq!(group.assignment(M1), vec![P0]);
    assert_eq!(group.assignment(M2), vec![P1]);
    assert_eq!(group.assignment(M3), Vec::<PartitionId>::new());
}

#[test]
fn a_spare_consumer_polls_nothing() {
    let log = log_with(
        2,
        &[(0, "pay-1", "acct-a", 100), (1, "pay-2", "acct-b", 200)],
    );
    let mut group = Group::new(log);
    group.join(M1);
    group.join(M2);
    group.join(M3);

    let counts = [M1, M2, M3].map(|m| group.poll(m, 10).unwrap().len());
    assert_eq!(counts, [1, 1, 0]);
}

#[test]
fn committed_records_are_not_redelivered_after_a_rebalance() {
    let mut group = Group::new(log_with(1, &PAYMENTS));
    let mut ledger = Ledger::new();

    group.join(M1);
    let batch = group.poll(M1, 10).unwrap();
    assert_eq!(
        apply_all(&mut ledger, &batch),
        vec![Applied::New, Applied::New, Applied::New]
    );
    group.commit(M1).unwrap();

    group.leave(M1);
    group.join(M2);
    assert!(group.poll(M2, 10).unwrap().is_empty());
    assert_eq!(ledger.balance("acct-a"), 600);
}

#[test]
fn a_rebalance_redelivers_uncommitted_records_but_applies_them_once() {
    let mut group = Group::new(log_with(1, &PAYMENTS));
    let mut ledger = Ledger::new();

    group.join(M1);

    let first = group.poll(M1, 2).unwrap();
    assert_eq!(offsets(&first), vec![Offset(0), Offset(1)]);
    assert_eq!(
        apply_all(&mut ledger, &first),
        vec![Applied::New, Applied::New]
    );
    group.commit(M1).unwrap();

    let second = group.poll(M1, 2).unwrap();
    assert_eq!(offsets(&second), vec![Offset(2)]);
    assert_eq!(apply_all(&mut ledger, &second), vec![Applied::New]);

    // member 1 dies here, before its next commit
    group.leave(M1);
    group.join(M2);

    let redelivered = group.poll(M2, 10).unwrap();
    assert_eq!(offsets(&redelivered), vec![Offset(2)]);
    let results = apply_all(&mut ledger, &redelivered);

    assert_eq!(
        ledger.balance("acct-a"),
        600,
        "a redelivered record was applied twice"
    );
    assert_eq!(results, vec![Applied::Duplicate]);

    group.commit(M2).unwrap();
    assert_eq!(group.log().committed(P0), Offset(3));
    assert!(group.poll(M2, 10).unwrap().is_empty());
}

#[test]
fn a_new_record_after_a_rebalance_is_still_applied() {
    let mut group = Group::new(log_with(1, &[(0, "pay-1", "acct-a", 100)]));
    let mut ledger = Ledger::new();

    group.join(M1);
    let batch = group.poll(M1, 10).unwrap();
    assert_eq!(apply_all(&mut ledger, &batch), vec![Applied::New]);

    group.leave(M1);
    group.join(M2);
    group.log_mut().append(P0, "pay-2", "acct-a", 50).unwrap();

    let next = group.poll(M2, 10).unwrap();
    assert_eq!(offsets(&next), vec![Offset(0), Offset(1)]);
    assert_eq!(
        apply_all(&mut ledger, &next),
        vec![Applied::Duplicate, Applied::New]
    );
    assert_eq!(ledger.balance("acct-a"), 150);
}

#[test]
fn the_same_record_applied_twice_changes_the_balance_once() {
    let mut ledger = Ledger::new();
    let r = record(0, 0, "pay-1", "acct-a", 100);

    assert_eq!(ledger.apply(&r), Ok(Applied::New));
    assert_eq!(ledger.apply(&r), Ok(Applied::Duplicate));
    assert_eq!(ledger.balance("acct-a"), 100);
}

#[test]
fn an_overflowing_apply_is_an_error_and_records_nothing() {
    let mut ledger = Ledger::new();
    let r1 = record(0, 0, "pay-1", "acct-a", i64::MAX);
    let r2 = record(0, 1, "pay-2", "acct-a", 1);
    let r3 = record(0, 2, "pay-3", "acct-a", -1);

    assert_eq!(ledger.apply(&r1), Ok(Applied::New));
    assert_eq!(
        ledger.apply(&r2),
        Err(LedgerError::Overflow {
            account: "acct-a".into()
        })
    );
    assert_eq!(ledger.balance("acct-a"), i64::MAX);

    assert_eq!(ledger.apply(&r3), Ok(Applied::New));
    assert_eq!(ledger.apply(&r2), Ok(Applied::New));
    assert_eq!(ledger.balance("acct-a"), i64::MAX);
}

#[test]
fn poll_by_a_non_member_is_an_error() {
    let mut group = Group::new(log_with(1, &[]));

    assert_eq!(
        group.poll(MemberId(9), 10),
        Err(GroupError::NotAMember(MemberId(9)))
    );
    assert_eq!(
        group.commit(MemberId(9)),
        Err(GroupError::NotAMember(MemberId(9)))
    );
}
