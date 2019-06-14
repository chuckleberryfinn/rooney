use std::fmt;
use titlecase::titlecase;

use super::db;
use super::NaiveDate;

impl fmt::Display for db::stats::Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stats for {} ({}) on {}: Min €{} Mean €{} Std Dev €{} Median €{} Max €{}",
                titlecase(&self.name), self.ticker.to_uppercase(), self.date, super::format_currency(self.min),
                super::format_currency(self.average), super::format_currency(self.std_dev),
                super::format_currency(self.median), super::format_currency(self.max))
    }
}

pub fn get_stats(db: &db::DB, coin: String, date: NaiveDate) -> Option<String> {
    let stats = db.get_stats(coin, date);
    if let Some(s) = stats {
        return Some(format!("{}", s));
    }

    None
}