/// A finished job holds its output until someone collects it.
pub struct Job {
    pub output: Option<String>,
}

impl Job {
    /// Hands the output to the caller. The job keeps nothing.
    /// CLUE: After this returns, what should `self.output` hold, and who owns the String?
    /// CLUE: Look at the methods on `Option` that take `&mut self`.
    pub fn finish(&mut self) -> Option<String> {
        self.output
    }
}
