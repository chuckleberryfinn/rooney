use postgres::Connection;

pub fn query(connection: &Connection, msg: &str) -> Option<String> {
    let query =
        "with all_remarks as (
            select remark from replies
            join replies_remarks using(reply_id)
            join remarks using(remark_id)
            where $1 ~ regex
        )
        select * from all_remarks
        offset floor(random() * (select count(*) from all_remarks))
        limit 1;";

    let rows = connection.query(&query, &[&msg]).unwrap();

    if rows.len() == 0 {
        return None;
    }

    Some(rows.get(0).get(0))
}