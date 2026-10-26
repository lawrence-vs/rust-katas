use std::num::NonZeroU32;

use kata_events_193::{Event, Input, MergeError, Merger, StreamId, buffer_bound};

fn merger(streams: u32, lateness: u64) -> Merger {
    Merger::new(NonZeroU32::new(streams).unwrap(), lateness)
}

fn ev(stream: u32, ts: u64, id: u64) -> Event {
    Event {
        ts,
        id,
        stream: StreamId(stream),
    }
}

fn e(stream: u32, ts: u64, id: u64) -> Input {
    Input::Event(ev(stream, ts, id))
}

fn idle(stream: u32, ts: u64) -> Input {
    Input::Idle {
        stream: StreamId(stream),
        ts,
    }
}

/// Pushes every input, then finishes; returns the whole feed.
fn run(m: &mut Merger, inputs: &[Input]) -> Vec<Event> {
    let mut out = Vec::new();
    for &input in inputs {
        out.extend(m.push(input).unwrap());
    }
    out.extend(m.finish());
    out
}

#[test]
fn streams_merge_into_one_timestamp_ordered_feed() {
    let mut m = merger(2, 0);
    let feed = run(
        &mut m,
        &[
            e(0, 10, 1),
            e(0, 20, 2),
            e(1, 5, 3),
            e(1, 15, 4),
            e(1, 25, 5),
            e(0, 30, 6),
        ],
    );
    let want = vec![
        ev(1, 5, 3),
        ev(0, 10, 1),
        ev(1, 15, 4),
        ev(0, 20, 2),
        ev(1, 25, 5),
        ev(0, 30, 6),
    ];
    assert_eq!(feed, want, "merged feed is out of (timestamp, id) order");
}

#[test]
fn equal_timestamps_are_ordered_by_id_whatever_the_arrival() {
    let want = vec![ev(0, 10, 3), ev(1, 10, 7)];
    let mut first = merger(2, 0);
    assert_eq!(run(&mut first, &[e(1, 10, 7), e(0, 10, 3)]), want);
    let mut second = merger(2, 0);
    assert_eq!(run(&mut second, &[e(0, 10, 3), e(1, 10, 7)]), want);
}

#[test]
fn nothing_is_emitted_above_the_watermark() {
    let mut m = merger(2, 5);
    assert_eq!(m.push(e(0, 100, 1)).unwrap(), vec![]);
    assert_eq!(m.watermark(), None);
    assert_eq!(m.push(e(1, 50, 2)).unwrap(), vec![]);
    assert_eq!(m.watermark(), Some(45));
    assert_eq!(m.buffered(), 2);
    assert_eq!(m.push(e(1, 104, 3)).unwrap(), vec![ev(1, 50, 2)]);
    assert_eq!(m.watermark(), Some(95));
    assert_eq!(m.buffered(), 2);
}

#[test]
fn an_idle_marker_advances_a_silent_stream() {
    let mut m = merger(2, 0);
    assert_eq!(m.push(e(0, 10, 1)).unwrap(), vec![]);
    assert_eq!(m.push(e(0, 20, 2)).unwrap(), vec![]);
    assert_eq!(m.watermark(), None);
    assert_eq!(m.push(idle(1, 15)).unwrap(), vec![ev(0, 10, 1)]);
    assert_eq!(m.push(idle(1, 30)).unwrap(), vec![ev(0, 20, 2)]);
    assert_eq!(m.watermark(), Some(20));
    assert_eq!(m.buffered(), 0);
}

#[test]
fn an_event_behind_the_emitted_position_is_counted_late() {
    let mut m = merger(2, 0);
    assert_eq!(m.push(e(0, 10, 1)).unwrap(), vec![]);
    assert_eq!(m.push(e(1, 12, 2)).unwrap(), vec![ev(0, 10, 1)]);
    assert_eq!(m.push(e(0, 20, 3)).unwrap(), vec![ev(1, 12, 2)]);
    assert_eq!(m.push(e(1, 11, 4)).unwrap(), vec![]);
    assert_eq!(m.push(e(1, 12, 1)).unwrap(), vec![]);
    assert_eq!(m.late(), &[ev(1, 11, 4), ev(1, 12, 1)]);
    assert_eq!(m.late_count(), 2);
    assert_eq!(m.buffered(), 1);
    assert_eq!(m.finish(), vec![ev(0, 20, 3)]);
}

#[test]
fn the_feed_does_not_depend_on_interleaving() {
    let a = [e(0, 1, 1), e(0, 4, 4), e(0, 7, 7), e(0, 10, 10)];
    let b = [e(1, 2, 2), e(1, 5, 5), e(1, 8, 8), e(1, 11, 11)];
    let c = [e(2, 3, 3), e(2, 6, 6), e(2, 9, 9), e(2, 12, 12)];
    let want: Vec<u64> = (1..=12).collect();

    let mut round_robin = Vec::new();
    for i in 0..4 {
        round_robin.extend([a[i], b[i], c[i]]);
    }
    let mut m = merger(3, 0);
    let ids: Vec<u64> = run(&mut m, &round_robin).iter().map(|x| x.id).collect();
    assert_eq!(ids, want);
    assert_eq!(m.late_count(), 0);

    let one_by_one: Vec<Input> = a.iter().chain(&b).chain(&c).copied().collect();
    let mut m = merger(3, 0);
    let ids: Vec<u64> = run(&mut m, &one_by_one).iter().map(|x| x.id).collect();
    assert_eq!(ids, want);
    assert_eq!(m.late_count(), 0);
}

#[test]
fn an_unknown_stream_is_an_error() {
    let mut m = merger(2, 0);
    assert_eq!(
        m.push(e(2, 1, 1)),
        Err(MergeError::UnknownStream(StreamId(2)))
    );
    assert_eq!(
        m.push(idle(5, 1)),
        Err(MergeError::UnknownStream(StreamId(5)))
    );
    assert_eq!(
        MergeError::UnknownStream(StreamId(2)).to_string(),
        "unknown stream 2"
    );
}

#[test]
fn buffer_bound_is_rate_times_lateness() {
    assert_eq!(buffer_bound(10_000, 250, 64), Some((2_500, 160_000)));
    assert_eq!(buffer_bound(3, 500, 10), Some((2, 20)));
    assert_eq!(buffer_bound(u64::MAX, 2, 1), None);
}
