use postgres::Connection;

pub fn query(connection: &Connection) -> Option<String> {
        let query = "select response from advice offset floor(random()*(select count(*) from advice)) limit 1;";

        let rows = connection.query(query, &[]).unwrap();
        if rows.is_empty() {
            return None
        }
        Some(rows.get(0).get(0))
}