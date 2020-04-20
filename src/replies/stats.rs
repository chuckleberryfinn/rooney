use std::fmt;
use titlecase::titlecase;

use super::{db, formatter::format_currency, Command, Error, Result};

pub(super) struct Stats;

impl Command for Stats {
    fn name(&self) -> &'static str {
        "!stats"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap_or("").split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let date = self.parse_date(&commands);
        let stats = db.get_stats(coin, date);

        match stats {
            Some(s) => Ok(format!("{}", s)),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!stats [coin|ticker] [date]: Get the statistics for a coin's price over the course of a day. \
            Defaults to btc and yesterday's date."
    }
}


impl fmt::Display for db::stats::Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stats for {} ({}) on {}: Min €{} Mean €{} Std Dev €{} Median €{} Max €{}",
                titlecase(&self.name), self.ticker.to_uppercase(), self.date, format_currency(self.min),
                format_currency(self.average), format_currency(self.std_dev),
                format_currency(self.median), format_currency(self.max))
    }
}
