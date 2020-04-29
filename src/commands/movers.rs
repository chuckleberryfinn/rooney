use super::{db, Command, CommandArgs, Error, Result};

pub(super) struct Bulls;

impl Command for Bulls {
    fn name(&self) -> &'static str {
        "!bulls"
    }

    fn run(&self, db: &db::DB, _: &Option<&str>) -> Result<String> {
        let movers = db.get_bulls();

        match movers {
            Some(ms) => Ok(ms.into_iter().map(|m| format!("{}", m)).collect::<Vec<String>>().join(" ")),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!bulls: Get today's big winners."
    }
}

impl CommandArgs for Bulls {}

pub(super) struct Bears;

impl Command for Bears {
    fn name(&self) -> &'static str {
        "!bears"
    }

    fn run(&self, db: &db::DB, _: &Option<&str>) -> Result<String> {
        let movers = db.get_bears();

        match movers {
            Some(ms) => Ok(ms.into_iter().map(|m| format!("{}", m)).collect::<Vec<String>>().join(" ")),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!bears: Get today's big losers."
    }
}

impl CommandArgs for Bears {}
