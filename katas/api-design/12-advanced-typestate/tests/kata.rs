use kata_api_design_12::{Closed, Connection, Open};

#[test]
fn open_query_close() {
    let closed: Connection<Closed> = Connection::new("db");
    let open: Connection<Open> = closed.connect();
    assert_eq!(open.query("select 1"), "db: select 1");
    let closed: Connection<Closed> = open.close();
    assert_eq!(closed.url(), "db");
}
