use super::{db, Command, CommandArgs, Error, Result};

pub(super) struct Diff;

impl Command for Diff {
    fn name(&self) -> &'static str {
        "!diff"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let date = self.parse_date(&commands);
        let diff = db.get_diff(coin, date);

        match diff {
            Some(d) => Ok(format!("{}", d)),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!diff [coin|ticker] [date]: Get the difference in price between the start date and current price. \
            Defaults to btc and yesterday's date"
    }
}

impl CommandArgs for Diff {}
