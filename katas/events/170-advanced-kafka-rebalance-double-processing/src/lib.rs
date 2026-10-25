use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PartitionId(pub u32);

/// Position of a record in its partition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Offset(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MemberId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub partition: PartitionId,
    pub offset: Offset,
    /// The producer's id for this payment: the same payment always carries the same id.
    pub payment_id: String,
    pub account: String,
    pub amount_cents: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogError {
    NoSuchPartition(PartitionId),
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogError::NoSuchPartition(p) => write!(f, "no partition {}", p.0),
        }
    }
}

impl Error for LogError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupError {
    NotAMember(MemberId),
    Log(LogError),
}

impl fmt::Display for GroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupError::NotAMember(m) => write!(f, "member {} is not in the group", m.0),
            GroupError::Log(_) => write!(f, "log error"),
        }
    }
}

impl Error for GroupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GroupError::Log(e) => Some(e),
            GroupError::NotAMember(_) => None,
        }
    }
}

impl From<LogError> for GroupError {
    fn from(e: LogError) -> Self {
        GroupError::Log(e)
    }
}

/// A partitioned log and the group's committed offsets: the part of Kafka this kata needs.
pub trait Log {
    fn partition_count(&self) -> NonZeroU32;
    /// Up to `max` records of `partition`, starting at `from`. Empty past the end or for an unknown partition.
    fn fetch(&self, partition: PartitionId, from: Offset, max: usize) -> Vec<Record>;
    /// The next offset the group should read. `Offset(0)` if nothing was ever committed.
    fn committed(&self, partition: PartitionId) -> Offset;
    fn commit(&mut self, partition: PartitionId, next: Offset) -> Result<(), LogError>;
}

/// An in-process log. It behaves like the broker: records never change, and commits are per partition.
pub struct MemoryLog {
    count: NonZeroU32,
    partitions: Vec<Vec<Record>>,
    committed: HashMap<PartitionId, Offset>,
}

impl MemoryLog {
    pub fn new(partitions: NonZeroU32) -> Self {
        MemoryLog {
            count: partitions,
            partitions: (0..partitions.get()).map(|_| Vec::new()).collect(),
            committed: HashMap::new(),
        }
    }

    pub fn append(
        &mut self,
        partition: PartitionId,
        payment_id: &str,
        account: &str,
        amount_cents: i64,
    ) -> Result<Offset, LogError> {
        let records = self
            .partitions
            .get_mut(partition.0 as usize)
            .ok_or(LogError::NoSuchPartition(partition))?;
        let offset = Offset(records.len() as u64);
        records.push(Record {
            partition,
            offset,
            payment_id: payment_id.to_string(),
            account: account.to_string(),
            amount_cents,
        });
        Ok(offset)
    }
}

impl Log for MemoryLog {
    fn partition_count(&self) -> NonZeroU32 {
        self.count
    }

    fn fetch(&self, partition: PartitionId, from: Offset, max: usize) -> Vec<Record> {
        let Some(records) = self.partitions.get(partition.0 as usize) else {
            return Vec::new();
        };
        let start = usize::try_from(from.0).unwrap_or(usize::MAX);
        records.iter().skip(start).take(max).cloned().collect()
    }

    fn committed(&self, partition: PartitionId) -> Offset {
        self.committed.get(&partition).copied().unwrap_or(Offset(0))
    }

    fn commit(&mut self, partition: PartitionId, next: Offset) -> Result<(), LogError> {
        if partition.0 >= self.count.get() {
            return Err(LogError::NoSuchPartition(partition));
        }
        self.committed.insert(partition, next);
        Ok(())
    }
}

/// A consumer group: who owns which partition, and where each owner reads next.
pub struct Group<L> {
    log: L,
    members: BTreeSet<MemberId>,
    /// (partition, owner) pairs from the last rebalance.
    assignment: Vec<(PartitionId, MemberId)>,
    /// Where each owner reads next. Cleared on every rebalance, so an owner restarts from the committed offset.
    positions: HashMap<(MemberId, PartitionId), Offset>,
}

impl<L: Log> Group<L> {
    pub fn new(log: L) -> Self {
        Group {
            log,
            members: BTreeSet::new(),
            assignment: Vec::new(),
            positions: HashMap::new(),
        }
    }

    pub fn join(&mut self, member: MemberId) {
        self.members.insert(member);
        self.rebalance();
    }

    pub fn leave(&mut self, member: MemberId) {
        self.members.remove(&member);
        self.rebalance();
    }

    pub fn assignment(&self, member: MemberId) -> Vec<PartitionId> {
        let mut owned: Vec<PartitionId> = self
            .assignment
            .iter()
            .filter(|(_, owner)| *owner == member)
            .map(|(p, _)| *p)
            .collect();
        owned.sort();
        owned
    }

    pub fn poll(&mut self, member: MemberId, max: usize) -> Result<Vec<Record>, GroupError> {
        if !self.members.contains(&member) {
            return Err(GroupError::NotAMember(member));
        }
        let mut out = Vec::new();
        for partition in self.assignment(member) {
            let from = match self.positions.get(&(member, partition)) {
                Some(&at) => at,
                None => self.log.committed(partition),
            };
            let batch = self.log.fetch(partition, from, max);
            if let Some(last) = batch.last() {
                self.positions
                    .insert((member, partition), Offset(last.offset.0.saturating_add(1)));
            }
            out.extend(batch);
        }
        Ok(out)
    }

    /// CLUE: The caller commits after applying. If the process dies between apply and commit, what does the partition's next owner read first?
    pub fn commit(&mut self, member: MemberId) -> Result<(), GroupError> {
        if !self.members.contains(&member) {
            return Err(GroupError::NotAMember(member));
        }
        for partition in self.assignment(member) {
            if let Some(&next) = self.positions.get(&(member, partition)) {
                self.log.commit(partition, next)?;
            }
        }
        Ok(())
    }

    pub fn log(&self) -> &L {
        &self.log
    }

    pub fn log_mut(&mut self) -> &mut L {
        &mut self.log
    }

    fn rebalance(&mut self) {
        self.assignment.clear();
        self.positions.clear();
        let members: Vec<MemberId> = self.members.iter().copied().collect();
        if members.is_empty() {
            return;
        }
        let n = self.log.partition_count().get();
        for p in 0..n {
            if let Some(&owner) = members.get(p as usize % members.len()) {
                self.assignment.push((PartitionId(p), owner));
            }
        }
        // Keep every consumer busy: spare members share a partition.
        // CLUE: With three members and two partitions, who reads partition 0, and how many times does each record get read?
        for (i, &spare) in members.iter().enumerate().skip(n as usize) {
            self.assignment.push((PartitionId(i as u32 % n), spare));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applied {
    /// The record changed a balance.
    New,
    /// Seen before: nothing changed.
    Duplicate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    Overflow { account: String },
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LedgerError::Overflow { account } => write!(f, "balance of {account} would overflow"),
        }
    }
}

impl Error for LedgerError {}

/// The external store the consumer writes to (a database, in real life).
#[derive(Debug, Default)]
pub struct Ledger {
    balances: HashMap<String, i64>,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    /// CLUE: After a rebalance this can be handed a record it has already applied. What in `rec` is the same both times, and where must you remember it so that an error leaves no trace?
    pub fn apply(&mut self, rec: &Record) -> Result<Applied, LedgerError> {
        let next = self
            .balance(&rec.account)
            .checked_add(rec.amount_cents)
            .ok_or_else(|| LedgerError::Overflow {
                account: rec.account.clone(),
            })?;
        self.balances.insert(rec.account.clone(), next);
        Ok(Applied::New)
    }

    pub fn balance(&self, account: &str) -> i64 {
        self.balances.get(account).copied().unwrap_or(0)
    }
}
