use std::fmt;
use titlecase::titlecase;

use super::{db, Command, Error, Result};
use super::NaiveDate;
use super::formatter::{format_change, format_currency};

pub(super) struct Diff;

impl Command for Diff {
    fn name(&self) -> &'static str {
        "!diff"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let date = self.parse_date(&commands);
        let diff = db.get_diff(coin, date);

        match diff {
            Some(d) => Ok(format!("{}", d)),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!diff [coin|ticker] [date]: Get the difference in price between the start date and current price. \
            Defaults to btc and yesterday's date"
    }
}

impl fmt::Display for db::diff::Diff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Diff for {} ({}) from {} to {}: First: €{} Latest: €{} Diff: {} To Date",
                titlecase(&self.name), self.ticker.to_uppercase(), self.start, self.end,
                format_currency(self.first), format_currency(self.last), format_change(self.diff))
    }
}

pub fn help() -> String {
    "!diff [coin|ticker] [date]: Get the difference in price between the start date and current price. \
        Defaults to btc and yesterday's date".to_string()
}

pub fn get_diff(db: &db::DB, coin: String, date: NaiveDate) -> Option<String> {
    let diff = db.get_diff(coin, date);
    if let Some(d) = diff {
        return Some(format!("{}", d));
    }

    None
}