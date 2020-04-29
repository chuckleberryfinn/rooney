use std::fmt;
use titlecase::titlecase;

use super::formatter::format_currency;
use super::price;

use postgres::Connection;


pub struct Fiat {
    pub name: String,
    pub ticker: String,
    pub amount: f32,
    pub euro: f32,
}

pub(super) fn query(connection: &Connection, coin: String, amount: f32) -> Option<Fiat> {
    let price = price::query(connection, &coin).unwrap();

    Some(Fiat {
        name: coin,
        amount,
        ticker: price.ticker,
        euro: price.euro
    })
}

impl fmt::Display for Fiat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} ({}) is worth €{} at €{} per coin", self.amount, titlecase(&self.name),
                self.ticker.to_uppercase(), format_currency(self.amount * self.euro), format_currency(self.euro))
    }
}