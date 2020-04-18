use super::db;
use std::cmp::Ordering;
use std::str::FromStr;
use std::time;

use chrono::{Duration, NaiveDate, Utc};
use failure::Fail;

mod advice;
mod ats;
mod diff;
mod help;
mod fiat;
mod formatter;
mod movers;
mod price;
mod remark;
mod stats;

const COOLDOWN: u64 = 3;

pub struct Replies {
    db: db::DB,
    last_call: Option<time::Instant>
}


pub struct Commands {
    commands: Vec<Box<dyn Command>>,
    db: db::DB,
    last_call: Option<time::Instant>
}


impl Commands {
    pub(super) fn new() -> Commands {
        Commands {
            commands: vec![Box::new(advice::Advice), Box::new(ats::ATS), Box::new(diff::Diff),
                           Box::new(fiat::Fiat), Box::new(movers::Bulls), Box::new(movers::Bears),
                           Box::new(price::Coin), Box::new(stats::Stats)],
            db: db::DB::new(),
            last_call: None
        }
    }

    pub(super) fn handle(&self, message: &str) -> Result<String> {
        let mut split = message.splitn(2, ' ');
        let command = split.next().unwrap();
        let rest = split.next();

        if command == "!help" {
            return match rest {
                None => self.general_help(),
                Some(r) => {
                    let mut split = r.splitn(2, ' ');
                    match self.find_command(&split.next().unwrap()) {
                        None => self.general_help(),
                        Some(c) => Ok(c.help().to_string()),
                    }
                }
            }
        }

        match self.find_command(&command) {
            Some(c) => c.run(&self.db, &message),
            None => Ok(remark::get_remark(&self.db, &message).unwrap())
        }
    }

    fn find_command(&self, command: &str) -> Option<&Box<dyn Command>> {
        self.commands.iter().find(|c| c.name() == command)
    }

    fn general_help(&self) -> Result<String> {
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

trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, db: &db::DB, args: &Option<&str>) -> Result<String>;
    fn help(&self) -> &'static str;

    fn parse_coin_arg(&self, words: &[&str]) -> String {
        match words.len() {
            1 => "bitcoin".to_string(),
            _ => words[1].to_string().to_lowercase(),
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


impl Replies {
    pub fn new() -> Self {
        Self {
            db: db::DB::new(),
            last_call: None
        }
    }

    fn on_cooldown(&mut self) -> bool {
        match self.last_call {
            Some(l) => {
                match l.elapsed() {
                    d if d > time::Duration::new(60*COOLDOWN, 0) => {
                        self.last_call = Some(time::Instant::now());
                        false
                    },
                    _ => true
                }
            },
            None => {
                self.last_call = Some(time::Instant::now());
                false
            }
        }
    }

    pub fn handle_message(&mut self, msg: &str) -> Option<String> {
        let commands: Vec<&str> = msg.split_whitespace().collect();
        let command = commands[0];

        match command {
            "!coin" | "!crack" => price::get_latest_price(
                &self.db, self.get_coin(
                    self.parse_coin_arg(&commands)
                )
            ),
            "!advice" => {
                if !self.on_cooldown() {
                    advice::get_advice(&self.db)
                } else {
                    None
                }
            },
            "!ats" => ats::get_ats(
                &self.db, self.get_coin(
                    self.parse_coin_arg(&commands)
                )
            ),
            "!bulls" => movers::get_bulls(&self.db),
            "!bears" => movers::get_bears(&self.db),
            "!fiat" => {
                let (coin, amount) = self.parse_coin_amount(&commands);
                fiat::get_fiat(&self.db, coin, amount)
            },
            "!stats" => {
                let (coin, date) = self.parse_coin_date(&commands);
                stats::get_stats(&self.db, coin, date)
            },
            "!diff" => {
                let (coin, date) = self.parse_coin_date(&commands);
                diff::get_diff(&self.db, coin, date)
            },
            "!help" => {
                match self.parse_help(&commands) {
                    None => Some(self.help()),
                    Some(c) => help::get_help(&c)
                }
            },
            _ => remark::get_remark(&self.db, msg)
        }
    }

    fn parse_coin_arg(&self, words: &[&str]) -> String {
        match words.len() {
            1 => "bitcoin".to_string(),
            _ => words[1].to_string().to_lowercase(),
        }
    }

    fn parse_help(&self, words: &[&str]) -> Option<String> {
        match words.len() {
            1 => None,
            _ => Some(words[1].to_string().to_lowercase()),
        }
    }

    fn parse_coin_amount(&self, words: &[&str]) -> (String, f32) {
        let coin = self.get_coin(self.parse_coin_arg(words));

        let amount = match words.len().cmp(&2) {
            Ordering::Equal => words[1],
            Ordering::Greater => words[2],
            Ordering::Less => "1.0"
        };

        (coin, f32::from_str(amount).unwrap_or(1.0))
    }

    fn parse_coin_date(&self, words: &[&str]) -> (String, NaiveDate) {
        let coin = self.get_coin(self.parse_coin_arg(words));

        let date = match words.len().cmp(&2) {
            Ordering::Equal => words[1],
            Ordering::Greater => words[2],
            Ordering::Less => "Yesterday"
        };

        (coin, NaiveDate::from_str(date).unwrap_or(Utc::today().naive_local() - Duration::days(1)))
    }

    fn get_coin(&self, coin: String) -> String {
        if self.db.all_coins.contains(&coin) {
            return coin;
        }

        match self.db.nicks_coins.get(&coin) {
            Some(c) => c,
            None => "bitcoin"
        }.to_string()
    }

    fn help(&self) -> String {
        "Commands: !advice !ats !bears !bulls !help !coin !diff !fiat !stats. \
            !help [command] for more information on a specific command.".to_string()
    }
}