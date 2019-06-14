use postgres::Connection;

use chrono::{NaiveDate, NaiveDateTime};

pub struct Diff {
    pub name: String,
    pub ticker: String,
    pub start: NaiveDate,
    pub end: NaiveDateTime,
    pub first: f32,
    pub last: f32,
    pub diff: f32,
}

pub fn query(connection: &Connection, coin: String, date: NaiveDate) -> Option<Diff> {
    let query =
        "with first as (
            select coin_id, date, average_euro as first
            from daily_stats
            join coins using(coin_id)
            where name = ($1)
            and date = ($2)
        )
        select name, ticker, date, date_trunc('minute', time) as latest, cast(first as real), cast(euro as real) as last,
        cast((euro-first)*100/first as real) as diff
        from first
        join prices using(coin_id)
        join coins using(coin_id)
        where name = ($1)
        order by time desc limit 1;";

    let rows = connection.query(&query, &[&coin, &date]).unwrap();

    if rows.len() == 0 {
        return None;
    }

    let row = rows.get(0);
    Some(Diff{
        name: row.get(0),
        ticker: row.get(1),
        start: row.get(2),
        end: row.get(3),
        first: row.get(4),
        last: row.get(5),
        diff: row.get(6),
    })
}