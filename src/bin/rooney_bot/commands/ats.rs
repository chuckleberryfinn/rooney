use std::fmt;
use titlecase::titlecase;

use super::{db, Command, CommandArgs, Error, formatter::format_currency, Result};

use chrono::NaiveDate;


pub(super) struct ATS;


struct _ATS {
    name: String,
    lowest: f32,
    lowest_date: NaiveDate,
    highest: f32,
    highest_date: NaiveDate,
}


impl ATS {
    fn query(&self, db: &mut db::DB, coin: String) -> Option<_ATS> {
        let query =
            "with all_ats as (
                select min(euro) as lowest, max(euro) as ath
                from prices
                join coins using(coin_id)
                where name = $1
                union select min_euro as lowest, max_euro as ath
                from daily_stats
                join coins using(coin_id)
                where name = $1
            ),
            extremes as (
                select min(lowest) as minimum, max(ath) as ath
                from all_ats
            ),
            lowest as (
                select time::date as date, euro as price
                from prices
                join coins using(coin_id)
                where euro=(select minimum from extremes)
                and name = $1
                union select date, min_euro as price
                from daily_stats
                join coins using(coin_id)
                where min_euro=(select minimum from extremes)
                and name = $1
                limit 1
            ),
            highest as (
                select time::date as date, euro as price
                from prices
                join coins using(coin_id)
                where euro=(select ath from extremes)
                and name = $1
                union select date, max_euro as price
                from daily_stats
                join coins using(coin_id)
                where max_euro=(select ath from extremes)
                and name = $1
                limit 1
            )
            select date, cast(price as real) from lowest union select date, cast(price as real) from highest
            order by price asc";

        let rows = db.connection.query(query, &[&coin]).unwrap();
        if rows.len() < 2 {
            return None;
        }

        let (lowest, highest) = (rows.get(0).unwrap(), rows.get(1).unwrap());
        Some(_ATS {
            name: coin,
            lowest_date: lowest.get(0),
            lowest: lowest.get(1),
            highest_date: highest.get(0),
            highest: highest.get(1)
        })
    }
}


impl Command for ATS {
    fn name(&self) -> &'static str {
        "!ats"
    }

    fn run(&self, db: &mut db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let ats = self.query(db, coin);

        match ats {
            Some(a) => Ok(a.to_string()),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!ats [coin]: All time highs and lows for a coin. Defaults to bitcoin."
    }
}


impl fmt::Display for _ATS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "All time \x0305Low\x03/\x0303High\x03 Prices for {}, Lowest: \x0305€{}\x03 on {} Highest: \x0303€{}\x03 on {}",
            titlecase(&self.name), format_currency(self.lowest), self.lowest_date, format_currency(self.highest), self.highest_date
        )
    }
}


impl CommandArgs for ATS {}
