use rooney::db;
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
    pub(super) fn new() -> Result<Commands> {
        let db = match db::DB::new() {
            Ok(db) => db,
            Err(_) => return Err(Error::Contact)
        };

        Ok(Self {
            commands: vec![Box::new(advice::Advice::new()), Box::new(ats::ATS), Box::new(diff::Diff),
                           Box::new(fiat::Fiat), Box::new(movers::Bulls), Box::new(movers::Bears),
                           Box::new(price::Coin), Box::new(price::Coin24), Box::new(stats::Stats)],
            remark: Box::new(remark::Remark::new()),
            db
        })
    }

    pub(super) fn handle(&mut self, message: &str) -> Result<String> {
        let mut split = message.splitn(2, ' ');
        let command = split.next().unwrap();
        let rest = split.next();

        if command == "!help" {
            return match rest {
                None => self.help(),
                Some(r) => {
                    let mut split = r.splitn(2, ' ');
                    let c = self.commands.iter().find(|c| c.name() == split.next().unwrap()).unwrap_or(&self.remark);
                    match c.name() {
                        "remark" => self.help(),
                        _ => Ok(c.help().to_string()),
                    }
                }
            }
        }
        let c = self.commands.iter().find(|c| c.name() == command).unwrap_or(&self.remark);
        c.run(&mut self.db, &Some(message))
    }

    fn help(&self) -> Result<String> {
        Ok("Commands: !advice !ats !bears !bulls !help !coin !diff !fiat !stats. \
            !help [command] for more information on a specific command.".to_string())
    }
}


#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "Cannot contact database")]
    Contact,
    #[fail(display = "Command is on cooldown")]
    Cooldown,
    #[fail(display = "No reply")]
    Reply,
}


pub(super) type Result<T, E = Error> = std::result::Result<T, E>;


trait Cooldown {
    fn get_last_call(&self) -> Option<Instant>;
    fn set_last_call(&self);
    fn on_cooldown(&self) -> bool;
}


trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, db: &mut db::DB, args: &Option<&str>) -> Result<String>;
    fn help(&self) -> &'static str;
}


trait CommandArgs {
    fn parse_coin_arg(&self, words: &[&str]) -> String {
        match words.len() {
            1 => "bitcoin",
            _ => words[1],
        }.to_string().to_lowercase()
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
        let date = match words.len().cmp(&2) {
            Ordering::Equal => words[1],
            Ordering::Greater => words[2],
            Ordering::Less => "Yesterday"
        };

        NaiveDate::from_str(date).unwrap_or(Utc::today().naive_local() - Duration::days(1))
    }

    fn parse_amount(&self, words: &[&str]) -> f32 {
        let amount = match words.len().cmp(&2) {
            Ordering::Equal => words[1],
            Ordering::Greater => words[2],
            Ordering::Less => "1.0"
        };

        f32::from_str(amount).unwrap_or(1.0)
    }
}