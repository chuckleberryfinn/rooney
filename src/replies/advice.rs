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

pub (super) fn help() -> String {
    "!advice: Some of mooney's sage advice".to_string()
}

pub (super) fn get_advice(db: &db::DB) -> Option<String> {
    db.get_advice()
}