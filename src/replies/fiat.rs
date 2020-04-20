use titlecase::titlecase;

use super::{db, formatter::format_currency, Command, Error, Result};

pub(super) struct Fiat;

impl Command for Fiat {
    fn name(&self) -> &'static str {
        "!fiat"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap_or("").split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let amount = self.parse_amount(&commands);
        let price = db.get_latest_price(coin);

        match price {
            Some(p) => Ok(format!("{} {} ({}) is worth €{} at €{} per coin", amount, titlecase(&p.name),
                          p.ticker.to_uppercase(), format_currency(amount * p.euro), format_currency(p.euro))),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!fiat [coin|ticker] [amount]: Get the current price in fiat for an amount of coins. \
            Defaults to btc and 1 coin."
    }
}
