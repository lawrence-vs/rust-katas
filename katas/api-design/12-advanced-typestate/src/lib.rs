/// A database connection.
///
/// Querying a connection that isn't open must not compile:
///
/// ```compile_fail
/// let conn = kata_api_design_12::Connection::new("db");
/// conn.query("select 1");
/// ```
/// CLUE: The doc test above is marked `compile_fail`. Why does it compile today?
/// CLUE: What if `connect` took `self` by value and returned a different type?
pub struct Connection {
    pub url: String,
    pub open: bool,
}

impl Connection {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            open: false,
        }
    }

    pub fn connect(&mut self) {
        self.open = true;
    }

    pub fn query(&self, sql: &str) -> String {
        assert!(self.open, "query on a closed connection");
        format!("{}: {sql}", self.url)
    }

    pub fn close(&mut self) {
        self.open = false;
    }
}
