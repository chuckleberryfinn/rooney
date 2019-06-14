use std::fmt;
use titlecase::titlecase;

use super::db;

impl fmt::Display for db::mover::Mover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}) {} Today\x03", titlecase(&self.name), self.ticker.to_uppercase(), super::format_change(self.diff))
    }
}

pub fn get_bears(db: &db::DB) -> Option<String> {
    let movers = db.get_bears();
    if let Some(ms) = movers {
        return Some(ms.into_iter().map(|m| format!("{}", m)).collect::<Vec<String>>().join(" "));
    }

    None
}

pub fn get_bulls(db: &db::DB) -> Option<String> {
    let movers = db.get_bulls();
    if let Some(ms) = movers {
        return Some(ms.into_iter().map(|m| format!("{}", m)).collect::<Vec<String>>().join(" "));
    }

    None
}