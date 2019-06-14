use crate::db;
use std::str::FromStr;

use chrono::{Duration, NaiveDate, Utc};

mod ats;
mod diff;
mod fiat;
mod formatter;
mod movers;
mod price;
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
        if msg.starts_with("!coin") || msg.starts_with("!crack") {
            return price::get_latest_price(&self.db, self.get_coin(self.parse_coin_arg(msg)));
        }

        if msg == "!advice" {
            return self.db.get_advice();
        }

        if msg.starts_with("!ats") {
            return ats::get_ats(&self.db, self.get_coin(self.parse_coin_arg(msg)));
        }

        if msg == "!bulls" {
            return movers::get_bulls(&self.db);
        }

        if msg == "!bears" {
            return movers::get_bears(&self.db);
        }

        if msg.starts_with("!fiat") {
            let (coin, amount) = self.parse_coin_amount(msg);
            return fiat::get_fiat(&self.db, coin, amount);
        }

        if msg.starts_with("!stats") {
            let (coin, date) = self.parse_coin_date(msg);
            return stats::get_stats(&self.db, coin, date);
        }

        if msg.starts_with("!diff") {
            let (coin, date) = self.parse_coin_date(msg);
            return diff::get_diff(&self.db, coin, date);
        }

        self.db.get_remark(msg)
    }

    fn parse_coin_arg(&self, msg: &str) -> String {
        let words: Vec<&str> = msg.split_whitespace().collect();
        match words.len() {
            1 => "bitcoin".to_string(),
             _ => words[1].to_string().to_lowercase(),
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
}