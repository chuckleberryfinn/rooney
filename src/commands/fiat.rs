use super::{db, Command, CommandArgs, Error, Result};

pub(super) struct Fiat;

impl Command for Fiat {
    fn name(&self) -> &'static str {
        "!fiat"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let amount = self.parse_amount(&commands);
        let fiat = db.get_fiat(coin, amount);

        match fiat {
            Some(f) => Ok(format!("{}", f)),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!fiat [coin|ticker] [amount]: Get the current price in fiat for an amount of coins. \
            Defaults to btc and 1 coin."
    }
}

impl CommandArgs for Fiat {}