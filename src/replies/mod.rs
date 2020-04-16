use super::db;
use std::cmp::Ordering;
use std::str::FromStr;

use chrono::{Duration, NaiveDate, Utc};

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

pub struct Replies {
    db: db::DB,
}

impl Replies {
    pub fn new() -> Self {
        Self {
            db: db::DB::new()
        }
    }

    pub fn handle_message(&self, msg: &str) -> Option<String> {
        let commands: Vec<&str> = msg.split_whitespace().collect();
        let command = commands[0];

        match command {
            "!coin" | "!crack" => price::get_latest_price(
                &self.db, self.get_coin(
                    self.parse_coin_arg(&commands)
                )
            ),
            "!advice" => advice::get_advice(&self.db),
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

        let real_coin = match self.db.nicks_coins.get(&coin) {
            Some(c) => c,
            None => "bitcoin"
        };

        real_coin.to_string()
    }

    fn help(&self) -> String {
        "Commands: !advice !ats !bears !bulls !help !coin !diff !fiat !stats. \
            !help [command] for more information on a specific command.".to_string()
    }
}