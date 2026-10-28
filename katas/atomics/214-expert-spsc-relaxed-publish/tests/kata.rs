use kata_atomics_214::Spsc;
use std::sync::Arc;
use std::thread;

#[test]
fn usable_capacity_is_cap_minus_one() {
    let q = Spsc::with_capacity(4);
    assert!(q.push(1).is_ok());
    assert!(q.push(2).is_ok());
    assert!(q.push(3).is_ok());
    assert_eq!(q.push(4), Err(4));
    assert_eq!(q.pop(), Some(1));
    assert!(q.push(4).is_ok());
}

#[test]
fn one_thread_sends_another_receives_in_order() {
    let q = Arc::new(Spsc::with_capacity(8));
    let producer = {
        let q = Arc::clone(&q);
        thread::spawn(move || {
            for i in 0..100u32 {
                let mut v = i;
                while let Err(back) = q.push(v) {
                    v = back;
                    thread::yield_now();
                }
            }
        })
    };
    let mut got = Vec::new();
    while got.len() < 100 {
        match q.pop() {
            Some(v) => got.push(v),
            None => thread::yield_now(),
        }
    }
    producer.join().unwrap();
    assert_eq!(got, (0..100).collect::<Vec<u32>>());
}
