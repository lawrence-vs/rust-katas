/// CLUE: `Clone` on a `Vec` copies the elements; after `clone()`, who else can see a later `push`?
#[derive(Clone, Default)]
pub struct FakeTransport {
    sent: Vec<String>,
}

impl FakeTransport {
    pub fn send(&mut self, frame: &str) {
        self.sent.push(frame.to_owned());
    }

    /// CLUE: `sent` reads through `&self`; what must the field be so a clone sees the same frames?
    pub fn sent(&self) -> Vec<String> {
        self.sent.clone()
    }
}
