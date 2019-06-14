use std::fmt;
use titlecase::titlecase;

use super::db;

impl fmt::Display for db::price::Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current price for {} ({}): €{} ${} 24h Low: €{} Median: €{} 24h High: €{} {} Today",
                    titlecase(&self.name), self.ticker.to_uppercase(), super::format_currency(self.euro),
                    super::format_currency(self.dollar), super::format_currency(self.min), super::format_currency(self.median),
                    super::format_currency(self.max), super::format_change(self.change))
    }
}

pub fn get_latest_price(db: &db::DB, coin: String) -> Option<String> {
    let price = db.get_latest_price(coin);
    if let Some(p) = price {
        return Some(format!("{}", p));
    }

    None
}