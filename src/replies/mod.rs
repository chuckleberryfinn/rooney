use super::db;
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

        return match command {
            "!coin" | "!crack" => price::get_latest_price(
                &self.db, self.get_coin(
                    self.parse_coin_arg(msg)
                )
            ),
            "!advice" => return advice::get_advice(&self.db),
            "!ats" => return ats::get_ats(
                &self.db, self.get_coin(
                    self.parse_coin_arg(msg)
                )
            ),
            "!bulls" => return movers::get_bulls(&self.db),
            "!bears" => return movers::get_bears(&self.db),
            "!fiat" => {
                let (coin, amount) = self.parse_coin_amount(msg);
                return fiat::get_fiat(&self.db, coin, amount)
            },
            "!stats" => {
                let (coin, date) = self.parse_coin_date(msg);
                return stats::get_stats(&self.db, coin, date)
            },
            "!diff" => {
                let (coin, date) = self.parse_coin_date(msg);
                return diff::get_diff(&self.db, coin, date)
            },
            "!help" => {
                match self.parse_help(msg) {
                    None => return Some(self.help()),
                    Some(c) => return help::get_help(&c)
                }
            },
            _ => remark::get_remark(&self.db, msg)
        }
    }

    fn parse_coin_arg(&self, msg: &str) -> String {
        let words: Vec<&str> = msg.split_whitespace().collect();
        match words.len() {
            1 => "bitcoin".to_string(),
            _ => words[1].to_string().to_lowercase(),
        }
    }

    fn parse_help(&self, msg: &str) -> Option<String> {
        let words: Vec<&str> = msg.split_whitespace().collect();
        match words.len() {
            1 => None,
            _ => Some(words[1].to_string().to_lowercase()),
        }
    }

    fn parse_coin_amount(&self, msg: &str) -> (String, f32) {
        let coin = self.get_coin(self.parse_coin_arg(msg));
        let amount = 1.0;
        let words: Vec<&str> = msg.split_whitespace().collect();

        if words.len() == 2 {
            return match f32::from_str(words[1]) {
                Ok(f) => (coin, f),
                Err(_e) => (coin, amount),
            };
        }

        if words.len() > 2 {
            return match f32::from_str(words[2]) {
                Ok(f) => (coin, f),
                Err(_e) => (coin, amount),
            };
        }

        return (coin, amount);
    }

    fn parse_coin_date(&self, msg: &str) -> (String, NaiveDate) {
        let coin = self.get_coin(self.parse_coin_arg(msg));
        let date = Utc::today().naive_local() - Duration::days(1);
        let words: Vec<&str> = msg.split_whitespace().collect();

        if words.len() == 2 {
            return match NaiveDate::from_str(words[1]) {
                Ok(f) => (coin, f),
                Err(_e) => (coin, date),
            };
        }

        if words.len() > 2 {
            return match NaiveDate::from_str(words[2]) {
                Ok(f) => (coin, f),
                Err(_e) => (coin, date),
            };
        }

        return (coin, date);
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