use kata_errors_218::{OrderError, Wire, place_order};

struct FakeWire {
    writes: Vec<String>,
    fail_write: bool,
    fail_read: bool,
}

impl FakeWire {
    fn new(fail_write: bool, fail_read: bool) -> Self {
        Self {
            writes: Vec::new(),
            fail_write,
            fail_read,
        }
    }
}

impl Wire for FakeWire {
    fn write(&mut self, msg: &str) -> Result<(), String> {
        if self.fail_write {
            return Err("connection refused".into());
        }
        self.writes.push(msg.to_string());
        Ok(())
    }

    fn read_reply(&mut self) -> Result<String, String> {
        if self.fail_read {
            return Err("connection reset".into());
        }
        Ok("ack".into())
    }
}

#[test]
fn happy_path_returns_the_reply() {
    let mut wire = FakeWire::new(false, false);
    assert_eq!(place_order(&mut wire, "buy 1").as_deref(), Ok("ack"));
    assert_eq!(wire.writes, ["buy 1"]);
}

#[test]
fn failed_write_is_retryable() {
    let mut wire = FakeWire::new(true, false);
    let err = place_order(&mut wire, "buy 1").unwrap_err();
    assert_eq!(err, OrderError::DialTimeout);
    assert!(err.is_retryable());
}

#[test]
fn lost_reply_is_not_retryable() {
    let mut wire = FakeWire::new(false, true);
    let err = place_order(&mut wire, "buy 1").unwrap_err();
    assert!(
        matches!(err, OrderError::ReplyLost(_)),
        "a lost reply must not be retried"
    );
    assert!(!err.is_retryable(), "a lost reply must not be retried");
}

#[test]
fn retry_loop_sends_the_order_once_when_the_reply_is_lost() {
    let mut wire = FakeWire::new(false, true);
    for _ in 0..5 {
        match place_order(&mut wire, "buy 1") {
            Err(e) if e.is_retryable() => continue,
            _ => break,
        }
    }
    assert_eq!(wire.writes.len(), 1, "a lost reply must not be retried");
}
