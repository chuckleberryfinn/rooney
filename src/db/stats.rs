use chrono::NaiveDate;
use postgres::Connection;

pub struct Stats {
    pub name: String,
    pub ticker: String,
    pub date: NaiveDate,
    pub min: f32,
    pub average: f32,
    pub median: f32,
    pub std_dev: f32,
    pub max: f32,
}

pub fn query(connection: &Connection, coin: String, date: NaiveDate) -> Option<Stats> {
    let query =
        "select name, ticker, date, cast(min_euro as real), cast(average_euro as real), cast(median_euro as real), cast(std_dev as real), cast(max_euro as real)
            from daily_stats
            join coins using(coin_id)
            where name = $1
            and date = $2";
    let rows = connection.query(&query, &[&coin, &date]).unwrap();

    if rows.is_empty() {
        return None;
    }

    let row = rows.get(0);
    Some(Stats{
        name: row.get(0),
        ticker: row.get(1),
        date: row.get(2),
        min: row.get(3),
        average: row.get(4),
        median: row.get(5),
        std_dev: row.get(6),
        max: row.get(7),
    })
}