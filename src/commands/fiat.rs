use std::fmt;

use titlecase::titlecase;

use super::{db, Command, CommandArgs, Error, formatter::format_currency, price, Result};

pub(super) struct Fiat;


struct _Fiat {
    name: String,
    ticker: String,
    amount: f32,
    euro: f32,
}


impl Fiat {
    fn query(&self, db: &db::DB, coin: String, amount: f32) -> Option<_Fiat> {
        let price = price::Coin.query(db, &coin).unwrap();
    
        Some(_Fiat {
            name: coin,
            amount,
            ticker: price.ticker,
            euro: price.euro
        })
    }
}


impl Command for Fiat {
    fn name(&self) -> &'static str {
        "!fiat"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let amount = self.parse_amount(&commands);
        let fiat = self.query(&db, coin, amount);

        match fiat {
            Some(f) => Ok(f.to_string()),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!fiat [coin|ticker] [amount]: Get the current price in fiat for an amount of coins. \
            Defaults to btc and 1 coin."
    }
}


impl CommandArgs for Fiat {}


impl fmt::Display for _Fiat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} ({}) is worth €{} at €{} per coin", self.amount, titlecase(&self.name),
                self.ticker.to_uppercase(), format_currency(self.amount * self.euro), format_currency(self.euro))
    }
}
