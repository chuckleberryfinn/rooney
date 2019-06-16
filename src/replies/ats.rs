use std::fmt;
use titlecase::titlecase;

use super::db;
use super::formatter::format_currency;

impl fmt::Display for db::ats::ATS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "All time \x0305Low\x03/\x0303High\x03 Prices for {}, Lowest: \x0305€{}\x03 on {} Highest: \x0303€{}\x03 on {}",
            titlecase(&self.name), format_currency(self.lowest), self.lowest_date, format_currency(self.highest), self.highest_date
        )
    }
}

pub fn help() -> String {
    return "!ats [coin]: All time highs and lows for a coin. Defaults to bitcoin.".to_string()
}

pub fn get_ats(db: &db::DB, coin: String) -> Option<String> {
    let ats = db.get_ats(coin);
    if let Some(a) = ats {
        return Some(format!("{}", a));
    }

    None
}