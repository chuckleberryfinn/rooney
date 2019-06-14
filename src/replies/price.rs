use std::fmt;
use titlecase::titlecase;

use super::db;
use super::formatter::{format_change, format_currency};

impl fmt::Display for db::price::Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current price for {} ({}): €{} ${} 24h Low: €{} Median: €{} 24h High: €{} {} Today",
                    titlecase(&self.name), self.ticker.to_uppercase(), format_currency(self.euro),
                    format_currency(self.dollar), format_currency(self.min), format_currency(self.median),
                    format_currency(self.max), format_change(self.change))
    }
}

pub fn get_latest_price(db: &db::DB, coin: String) -> Option<String> {
    let price = db.get_latest_price(coin);
    if let Some(p) = price {
        return Some(format!("{}", p));
    }

    None
}