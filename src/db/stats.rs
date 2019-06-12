use std::fmt;

use chrono::NaiveDate;
use postgres::Connection;
use titlecase::titlecase;

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

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stats for {} ({}) on {}: Min €{} Mean €{} Std Dev €{} Median €{} Max €{}",
                titlecase(&self.name), self.ticker.to_uppercase(), self.date, super::format_currency(self.min),
                super::format_currency(self.average), super::format_currency(self.std_dev),
                super::format_currency(self.median), super::format_currency(self.max))
    }
}

pub fn query(connection: &Connection, coin: String, date: NaiveDate) -> Option<Stats> {
    let query =
        "select name, ticker, date, cast(min_euro as real), cast(average_euro as real), cast(median_euro as real), cast(std_dev as real), cast(max_euro as real)
            from daily_stats
            join coins using(coin_id)
            where name = $1
            and date = $2";
    let rows = connection.query(&query, &[&coin, &date]).unwrap();

    if rows.len() == 0 {
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