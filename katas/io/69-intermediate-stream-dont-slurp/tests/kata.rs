use kata_io_69::{Summary, TradeError, summarise};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::io::{self, Read, Write};

// Counts live bytes on the current thread only, so parallel tests don't add noise.
thread_local! {
    static TRACKING: Cell<bool> = const { Cell::new(false) };
    static LIVE: Cell<isize> = const { Cell::new(0) };
    static PEAK: Cell<isize> = const { Cell::new(0) };
}

struct PeakOnThisThread;

fn record(delta: isize) {
    let _ = TRACKING.try_with(|t| {
        if t.get() {
            let _ = LIVE.try_with(|live| {
                live.set(live.get() + delta);
                let _ = PEAK.try_with(|peak| {
                    if live.get() > peak.get() {
                        peak.set(live.get());
                    }
                });
            });
        }
    });
}

unsafe impl GlobalAlloc for PeakOnThisThread {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            record(layout.size() as isize);
        }
        p
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        record(-(layout.size() as isize));
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let p = unsafe { System.realloc(ptr, layout, new_size) };
        if !p.is_null() {
            record(new_size as isize - layout.size() as isize);
        }
        p
    }
}

#[global_allocator]
static ALLOC: PeakOnThisThread = PeakOnThisThread;

fn start_tracking() {
    LIVE.with(|l| l.set(0));
    PEAK.with(|p| p.set(0));
    TRACKING.with(|t| t.set(true));
}

fn stop_tracking() -> isize {
    TRACKING.with(|t| t.set(false));
    PEAK.with(|p| p.get())
}

/// A reader that writes one CSV row at a time into a reused line buffer.
struct GeneratedTrades {
    rows: u64,
    next: u64,
    line: Vec<u8>,
    pos: usize,
}

impl GeneratedTrades {
    fn new(rows: u64) -> Self {
        GeneratedTrades {
            rows,
            next: 0,
            line: Vec::with_capacity(128),
            pos: 0,
        }
    }
}

fn side_of(i: u64) -> &'static str {
    if i.is_multiple_of(2) { "buy" } else { "sell" }
}

impl Read for GeneratedTrades {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos == self.line.len() {
            self.line.clear();
            self.pos = 0;
            if self.next == 0 {
                self.line.extend_from_slice(b"id,symbol,side,qty,price\n");
                self.next = 1;
            } else {
                let i = self.next - 1;
                if i >= self.rows {
                    return Ok(0);
                }
                writeln!(
                    self.line,
                    "{i},BTC-USD,{},{},6500000",
                    side_of(i),
                    i % 10 + 1
                )?;
                self.next += 1;
            }
        }
        let n = buf.len().min(self.line.len() - self.pos);
        buf[..n].copy_from_slice(&self.line[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }
}

#[test]
fn summarises_a_small_file() {
    let input: &[u8] = b"id,symbol,side,qty,price\n1, BTC-USD , buy, 3, 6500000\n2,BTC-USD,sell,2,6500100\n3,ETH-USD,buy,5,320000\n";
    let got = summarise(input).unwrap();
    assert_eq!(
        got,
        Summary {
            trades: 3,
            bought: 8,
            sold: 2
        }
    );
}

#[test]
fn a_header_only_file_has_no_trades() {
    let input: &[u8] = b"id,symbol,side,qty,price\n";
    assert_eq!(summarise(input).unwrap(), Summary::default());
}

#[test]
fn a_bad_row_is_an_error_not_a_panic() {
    let input: &[u8] =
        b"id,symbol,side,qty,price\n1,BTC-USD,buy,3,6500000\n2,BTC-USD,sell,abc,6500100\n";
    match summarise(input) {
        Err(TradeError::Csv(e)) => assert_eq!(e.position().map(|p| p.line()), Some(3)),
        other => panic!("expected a csv error, got {other:?}"),
    }
}

#[test]
fn a_million_rows_in_flat_memory() {
    const ROWS: u64 = 1_000_000;
    const LIMIT: isize = 1 << 20;

    let gen_input = GeneratedTrades::new(ROWS);

    let (mut bought, mut sold) = (0u64, 0u64);
    for i in 0..ROWS {
        if i.is_multiple_of(2) {
            bought += i % 10 + 1;
        } else {
            sold += i % 10 + 1;
        }
    }

    start_tracking();
    let result = summarise(gen_input);
    let peak = stop_tracking();

    let got = result.unwrap();
    assert_eq!(
        got,
        Summary {
            trades: ROWS,
            bought,
            sold
        }
    );
    assert!(
        peak < LIMIT,
        "peak allocation grew with the file: {peak} bytes live at once for {ROWS} rows (limit {LIMIT})"
    );
}
