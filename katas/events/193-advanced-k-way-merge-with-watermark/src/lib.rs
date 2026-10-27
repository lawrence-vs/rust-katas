use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

/// Which input stream an event came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StreamId(pub u32);

/// One event. Fields are in sort order: the derived `Ord` compares `ts`, then `id`, then `stream`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub ts: u64,
    pub id: u64,
    pub stream: StreamId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Event(Event),
    /// The stream is quiet, and will send nothing with a timestamp below `ts`.
    Idle {
        stream: StreamId,
        ts: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeError {
    UnknownStream(StreamId),
}

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MergeError::UnknownStream(s) => write!(f, "unknown stream {}", s.0),
        }
    }
}

impl Error for MergeError {}

/// Merges several streams, each already in `(ts, id)` order, into one feed in `(ts, id)` order.
pub struct Merger {
    lateness: u64,
    /// Highest timestamp each stream has reported, by event or idle marker. `None` until it reports.
    latest: Vec<Option<u64>>,
    late: Vec<Event>,
}

impl Merger {
    /// `lateness`: how far below the slowest stream's latest timestamp the watermark sits.
    pub fn new(streams: NonZeroU32, lateness: u64) -> Self {
        Merger {
            lateness,
            latest: vec![None; streams.get() as usize],
            late: Vec::new(),
        }
    }

    /// Feeds one input; returns the events that are now safe to emit, in order.
    /// CLUE: Stream 0 sends ts 20, then stream 1 sends ts 5. Which did the reader get first? And if two streams both send ts 10, does the feed depend on who was faster?
    pub fn push(&mut self, input: Input) -> Result<Vec<Event>, MergeError> {
        match input {
            Input::Event(e) => {
                self.see(e.stream, e.ts)?;
                // Each stream is already in order, so its head can go straight out.
                Ok(vec![e])
            }
            Input::Idle { stream, .. } => {
                self.slot(stream)?;
                Ok(Vec::new())
            }
        }
    }

    /// The minimum over all streams of their latest timestamp, minus `lateness`. `None` until every stream has reported.
    /// CLUE: A stream that is quiet for an hour sends only `Input::Idle`. What does this minimum stay at meanwhile, and what happens to everything waiting on it?
    pub fn watermark(&self) -> Option<u64> {
        let mut min: Option<u64> = None;
        for t in &self.latest {
            let t = (*t)?;
            min = Some(min.map_or(t, |m| m.min(t)));
        }
        min.map(|m| m.saturating_sub(self.lateness))
    }

    /// End of input: emits everything still buffered, in order.
    pub fn finish(&mut self) -> Vec<Event> {
        Vec::new()
    }

    /// Events held back, waiting for the watermark.
    pub fn buffered(&self) -> usize {
        0
    }

    /// Events that arrived behind a position already emitted, in arrival order. Never in the feed.
    /// CLUE: The reader already holds `(12, 2)`. Where can `(11, 4)` go without rewriting what the reader holds?
    pub fn late(&self) -> &[Event] {
        &self.late
    }

    pub fn late_count(&self) -> usize {
        self.late.len()
    }

    fn slot(&mut self, stream: StreamId) -> Result<&mut Option<u64>, MergeError> {
        self.latest
            .get_mut(stream.0 as usize)
            .ok_or(MergeError::UnknownStream(stream))
    }

    fn see(&mut self, stream: StreamId, ts: u64) -> Result<(), MergeError> {
        let slot = self.slot(stream)?;
        *slot = Some(slot.map_or(ts, |t| t.max(ts)));
        Ok(())
    }
}

/// Events and bytes a merge holds to wait `lateness_ms` at `events_per_sec`: rate x lateness, rounded up. `None` on overflow.
pub fn buffer_bound(
    events_per_sec: u64,
    lateness_ms: u64,
    bytes_per_event: u64,
) -> Option<(u64, u64)> {
    let events = events_per_sec.checked_mul(lateness_ms)?.div_ceil(1000);
    let bytes = events.checked_mul(bytes_per_event)?;
    Some((events, bytes))
}
