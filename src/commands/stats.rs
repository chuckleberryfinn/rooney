use std::fmt;
use titlecase::titlecase;
use chrono::NaiveDate;

use super::{db, Command, CommandArgs, Error, Result};
use super::formatter::format_currency;

pub(super) struct Stats;


struct _Stats {
    pub name: String,
    pub ticker: String,
    pub date: NaiveDate,
    pub min: f32,
    pub average: f32,
    pub median: f32,
    pub std_dev: f32,
    pub max: f32,
}


impl Stats {
    fn query(&self, db: &db::DB, coin: String, date: NaiveDate) -> Option<_Stats> {
        let query =
            "select name, ticker, date, cast(min_euro as real), cast(average_euro as real), cast(median_euro as real), cast(std_dev as real), cast(max_euro as real)
                from daily_stats
                join coins using(coin_id)
                where name = $1
                and date = $2";
        let rows = db.connection.query(&query, &[&coin, &date]).unwrap();

        if rows.is_empty() {
            return None;
        }

        let row = rows.get(0);
        Some(_Stats{
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
}


impl Command for Stats {
    fn name(&self) -> &'static str {
        "!stats"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let date = self.parse_date(&commands);
        let stats = self.query(&db, coin, date);

        match stats {
            Some(s) => Ok(s.to_string()),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!stats [coin|ticker] [date]: Get the statistics for a coin's price over the course of a day. \
            Defaults to btc and yesterday's date."
    }
}


impl CommandArgs for Stats {}


impl fmt::Display for _Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stats for {} ({}) on {}: Min €{} Mean €{} Std Dev €{} Median €{} Max €{}",
                titlecase(&self.name), self.ticker.to_uppercase(), self.date, format_currency(self.min),
                format_currency(self.average), format_currency(self.std_dev),
                format_currency(self.median), format_currency(self.max))
    }
}