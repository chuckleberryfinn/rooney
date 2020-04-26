use super::db;
use std::cmp::Ordering;
use std::str::FromStr;

use chrono::{Duration, NaiveDate, Utc};
use std::time::Instant;
use failure::Fail;

mod advice;
mod ats;
mod diff;
mod fiat;
mod formatter;
mod movers;
mod price;
mod remark;
mod stats;


pub struct Commands {
    commands: Vec<Box<dyn Command>>,
    remark: Box<dyn Command>,
    db: db::DB
}


impl Commands {
    pub(super) fn new() -> Commands {
        Commands {
            commands: vec![Box::new(advice::Advice::new()), Box::new(ats::ATS), Box::new(diff::Diff),
                           Box::new(fiat::Fiat), Box::new(movers::Bulls), Box::new(movers::Bears),
                           Box::new(price::Coin), Box::new(stats::Stats)],
            remark: Box::new(remark::Remark::new()),
            db: db::DB::new()
        }
    }

    pub(super) fn handle(&self, message: &str) -> Result<String> {
        let mut split = message.splitn(2, ' ');
        let command = split.next().unwrap();
        let rest = split.next();

        if command == "!help" {
            return match rest {
                None => self.help(),
                Some(r) => {
                    let mut split = r.splitn(2, ' ');
                    let c = self.find_command(&split.next().unwrap());
                    match c.name() {
                        "remark" => self.help(),
                        _ => Ok(c.help().to_string()),
                    }
                }
            }
        }
        self.find_command(&command).run(&self.db, &Some(message))
    }

    fn find_command(&self, command: &str) -> &Box<dyn Command> {
        self.commands.iter().find(|c| c.name() == command).unwrap_or(&self.remark)
    }

    fn help(&self) -> Result<String> {
        Ok("Commands: !advice !ats !bears !bulls !help !coin !diff !fiat !stats. \
            !help [command] for more information on a specific command.".to_string())
    }
}


#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "Unknown command")]
    Unknown,
    #[fail(display = "Cannot contact database")]
    Contact,
}


pub(super) type Result<T, E = Error> = std::result::Result<T, E>;


trait Cooldown {
    fn get_last_call(&self) -> Option<Instant>;
    fn set_last_call(&self);
    fn on_cooldown(&self) -> bool;
}


trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, db: &db::DB, args: &Option<&str>) -> Result<String>;
    fn help(&self) -> &'static str;

    fn parse_coin_arg(&self, words: &[&str]) -> String {
        match words.len() {
            0 => "bitcoin".to_string(),
            _ => words[0].to_string().to_lowercase(),
        }
    }
    
    fn get_coin(&self, db: &db::DB, coin: String) -> String {
        if db.all_coins.contains(&coin) {
            return coin;
        }

        match db.nicks_coins.get(&coin) {
            Some(c) => c,
            None => "bitcoin"
        }.to_string()
    }
    
    fn parse_date(&self, words: &[&str]) -> NaiveDate {
        let date = match words.len().cmp(&1) {
            Ordering::Equal => words[0],
            Ordering::Greater => words[1],
            Ordering::Less => "Yesterday"
        };

        NaiveDate::from_str(date).unwrap_or(Utc::today().naive_local() - Duration::days(1))
    }

    fn parse_amount(&self, words: &[&str]) -> f32 {
        let amount = match words.len().cmp(&1) {
            Ordering::Equal => words[0],
            Ordering::Greater => words[1],
            Ordering::Less => "1.0"
        };

        f32::from_str(amount).unwrap_or(1.0)
    }
}
