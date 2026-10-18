/// A connection that borrows its URL from somewhere else.
pub struct Connection<'a> {
    pub url: &'a str,
}

/// Points the connection at a new address and returns the address it ends up with.
pub fn reconnect() -> String {
    let mut db = Connection { url: "localhost" };
    {
        let temp = String::from("192.168.1.1");
        db.url = Box::leak(temp.into_boxed_str());
    }
    db.url.to_string()
}
