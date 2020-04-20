use super::{db, Command, Result};

pub(super) struct Advice;

impl Command for Advice {
    fn name(&self) -> &'static str {
        "!advice"
    }

    fn run(&self, db: &db::DB, _: &Option<&str>) -> Result<String> {
        Ok(db.get_advice().unwrap())
    }

    fn help(&self) -> &'static str {
        "!advice: Some of mooney's sage advice"
    }
}
