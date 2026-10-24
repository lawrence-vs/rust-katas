//! 05 intermediate: take, don't clone
//!
//! Think further, from the kata README:
//! 1. Call `finish` twice on the same job. What does each call return, and is that the behaviour you want from a "collect the result" method?
//! 2. What changes for the caller if `finish` consumed the job instead of borrowing it mutably? What can they no longer do afterwards?
//! 3. What if `output` were a plain `String` instead of an `Option<String>`? What would "nothing left behind" have to mean?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

struct Job {
    output: Option<String>,
}

impl Job {
    // Think further 1: take() leaves None behind, so a second finish returns None, not the output again.
    fn finish(&mut self) -> Option<String> {
        self.output.take()
    }
}

// Think further 2: consuming the job means the caller can't use it afterwards, which is the point.
fn finish_consuming(job: Job) -> Option<String> {
    job.output
}

fn main() {
    let mut job = Job {
        output: Some("report".to_string()),
    };
    println!("first finish: {:?}", job.finish());
    println!("second finish: {:?}", job.finish());

    let job = Job {
        output: Some("report 2".to_string()),
    };
    println!("consuming finish: {:?}", finish_consuming(job));

    // Think further 3: a plain String has no "nothing left" value, so mem::take gives an empty one.
    let mut plain = String::from("data");
    let taken = std::mem::take(&mut plain);
    println!("taken {taken:?}, left {plain:?}");
}
