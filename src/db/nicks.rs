use std::collections::HashMap;

use postgres::Connection;

pub fn query(connection: &Connection) -> HashMap<String, String> {
    let query = "Select ticker, name from coins";
    connection.query(query, &[]).unwrap().iter().map(|r| (r.get(0), r.get(1))).collect::<HashMap<String, String>>()
}