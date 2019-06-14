use std::fmt;
use titlecase::titlecase;

use super::db;
use super::NaiveDate;
use super::formatter::{format_change, format_currency};

impl fmt::Display for db::diff::Diff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Diff for {} ({}) from {} to {}: First: €{} Latest: €{} Diff: {} To Date",
                titlecase(&self.name), self.ticker.to_uppercase(), self.start, self.end,
                format_currency(self.first), format_currency(self.last), format_change(self.diff))
    }
}

pub fn get_diff(db: &db::DB, coin: String, date: NaiveDate) -> Option<String> {
    let diff = db.get_diff(coin, date);
    if let Some(d) = diff {
        return Some(format!("{}", d));
    }

    None
}