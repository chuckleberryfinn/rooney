use super::{db, Command, CommandArgs, Error, Result};

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
