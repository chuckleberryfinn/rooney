use super::{db, Command, CommandArgs, Error, Result};

pub(super) struct Stats;

impl Command for Stats {
    fn name(&self) -> &'static str {
        "!stats"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let date = self.parse_date(&commands);
        let stats = db.get_stats(coin, date);

        match stats {
            Some(s) => Ok(format!("{}", s)),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!stats [coin|ticker] [date]: Get the statistics for a coin's price over the course of a day. \
            Defaults to btc and yesterday's date."
    }
}

impl CommandArgs for Stats {}
