use std::fmt;
use titlecase::titlecase;

use super::db;
use super::formatter::format_change;

impl fmt::Display for db::mover::Mover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}) {} Today\x03", titlecase(&self.name), self.ticker.to_uppercase(), format_change(self.diff))
    }
}

pub fn get_bears(db: &db::DB) -> Option<String> {
    let movers = db.get_bears();
    if let Some(ms) = movers {
        return Some(ms.into_iter().map(|m| format!("{}", m)).collect::<Vec<String>>().join(" "));
    }

    None
}

pub fn bears_help() -> String {
    "!bears: Get today's big losers.".to_string()
}

pub fn get_bulls(db: &db::DB) -> Option<String> {
    let movers = db.get_bulls();
    if let Some(ms) = movers {
        return Some(ms.into_iter().map(|m| format!("{}", m)).collect::<Vec<String>>().join(" "));
    }

    None
}

pub fn bulls_help() -> String {
    "!bulls: Get today's big winners.".to_string()
}