use std::fmt;
use titlecase::titlecase;

use super::{db, Command, Error, Result};
use super::formatter::format_currency;

pub(super) struct ATS;

impl Command for ATS {
    fn name(&self) -> &'static str {
        "!ats"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap_or("").split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let ats = db.get_ats(coin);

        match ats {
            Some(a) => Ok(format!("{}", a)),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!ats [coin]: All time highs and lows for a coin. Defaults to bitcoin."
    }
}

impl fmt::Display for db::ats::ATS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "All time \x0305Low\x03/\x0303High\x03 Prices for {}, Lowest: \x0305€{}\x03 on {} Highest: \x0303€{}\x03 on {}",
            titlecase(&self.name), format_currency(self.lowest), self.lowest_date, format_currency(self.highest), self.highest_date
        )
    }
}
