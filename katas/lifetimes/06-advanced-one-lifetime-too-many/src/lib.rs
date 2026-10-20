/// Splits a string on a delimiter, lazily.
/// CLUE: The items come out of `haystack`. Where does `delim` come from?
pub struct StrSplit<'a> {
    rest: Option<&'a str>,
    delim: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delim: &'a str) -> Self {
        Self {
            rest: Some(haystack),
            delim,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    /// CLUE: Which borrow should the returned `&str` be tied to?
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let rest = self.rest.as_mut()?;
        match rest.find(self.delim) {
            Some(i) => {
                let head = &rest[..i];
                *rest = &rest[i + self.delim.len()..];
                Some(head)
            }
            None => self.rest.take(),
        }
    }
}

/// Everything before the first `c`.
/// CLUE: `delim` dies at the end of this function. Does the result point into it?
pub fn until_char(s: &str, c: char) -> &str {
    let delim = c.to_string();
    StrSplit::new(s, &delim)
        .next()
        .expect("split always yields once")
}
