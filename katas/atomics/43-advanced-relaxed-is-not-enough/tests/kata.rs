use kata_atomics_43::Snapshot;
use loom::sync::Arc;
use loom::thread;

#[test]
fn nothing_before_publish() {
    loom::model(|| {
        let snap = Snapshot::default();
        assert_eq!(snap.read(), None);
    });
}

#[test]
fn ready_means_the_data_is_there() {
    loom::model(|| {
        let snap = Arc::new(Snapshot::default());
        let writer = {
            let snap = Arc::clone(&snap);
            thread::spawn(move || snap.publish(101, 5))
        };
        if let Some(got) = snap.read() {
            assert_eq!(got, (101, 5), "reader saw ready but stale data");
        }
        writer.join().unwrap();
        assert_eq!(snap.read(), Some((101, 5)));
    });
}
