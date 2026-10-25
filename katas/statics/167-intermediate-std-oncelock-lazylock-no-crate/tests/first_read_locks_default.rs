//! Its own process: nothing has set the worker count before this test reads it.
use kata_statics_167::*;

#[test]
fn reading_first_locks_in_the_default() {
    assert_eq!(workers(), DEFAULT_WORKERS);
    assert_eq!(
        set_workers(2),
        Err(ConfigError::AlreadySet {
            current: DEFAULT_WORKERS
        })
    );
    assert_eq!(workers(), DEFAULT_WORKERS);
}
