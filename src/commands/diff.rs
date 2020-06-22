use chrono::{NaiveDate, NaiveDateTime};
use std::fmt;
use titlecase::titlecase;

use super::{db, Command, CommandArgs, Error, Result};

use super::formatter::{format_change, format_currency};

pub(super) struct Diff;


struct _Diff {
    name: String,
    ticker: String,
    start: NaiveDate,
    end: NaiveDateTime,
    first: f32,
    last: f32,
    diff: f32,
}


impl Diff {
    fn query(&self, db: &db::DB, coin: String, date: NaiveDate) -> Option<String> {
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
    
        let rows = db.connection.query(&query, &[&coin, &date]).unwrap();
    
        if rows.is_empty() {
            return None;
        }
    
        let row = rows.get(0);
        let d = _Diff{
            name: row.get(0),
            ticker: row.get(1),
            start: row.get(2),
            end: row.get(3),
            first: row.get(4),
            last: row.get(5),
            diff: row.get(6),
        };
        Some(d.to_string())
    }
    
}


impl fmt::Display for _Diff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Diff for {} ({}) from {} to {}: First: €{} Latest: €{} Diff: {} To Date",
                titlecase(&self.name), self.ticker.to_uppercase(), self.start, self.end,
                format_currency(self.first), format_currency(self.last), format_change(self.diff))
    }
}


impl Command for Diff {
    fn name(&self) -> &'static str {
        "!diff"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let date = self.parse_date(&commands);
        let diff = self.query(&db, coin, date);

        match diff {
            Some(d) => Ok(d),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!diff [coin|ticker] [date]: Get the difference in price between the start date and current price. \
            Defaults to btc and yesterday's date"
    }
}

impl CommandArgs for Diff {}
