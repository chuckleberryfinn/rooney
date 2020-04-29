use super::{db, Command, CommandArgs, Error, Result};

pub(super) struct ATS;

impl Command for ATS {
    fn name(&self) -> &'static str {
        "!ats"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let ats = db.get_ats(coin);

        match ats {
            Some(a) => Ok(format!("{}", a)),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!ats [coin]: All time highs and lows for a coin. Defaults to bitcoin."
    }
}

impl CommandArgs for ATS {}
