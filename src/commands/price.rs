use std::fmt;
use titlecase::titlecase;

use super::{db, Command, CommandArgs, Error, Result};
use super::formatter::{format_change, format_currency};

pub(super) struct Coin;

impl Command for Coin {
    fn name(&self) -> &'static str {
        "!coin"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let price = db.get_latest_price(coin);

        match price {
            Some(p) => Ok(format!("{}", p)),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!coin [coin|ticker]: Get current price for a coin. Defaults to btc."
    }
}

impl CommandArgs for Coin {}

impl fmt::Display for db::price::Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current price for {} ({}): €{} ${} 24h Low: €{} Median: €{} 24h High: €{} {} Today",
                    titlecase(&self.name), self.ticker.to_uppercase(), format_currency(self.euro),
                    format_currency(self.dollar), format_currency(self.min), format_currency(self.median),
                    format_currency(self.max), format_change(self.change))
    }
}