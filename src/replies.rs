use crate::db;
use std::fmt;
use std::str::FromStr;

use chrono::{Duration, NaiveDate, Utc};
use separator::Separatable;
use titlecase::titlecase;

pub struct Replies {
    db: db::DB,
}

impl Replies {
    pub fn new() -> Self {
        Self {
            db: db::DB::new(),
        }
    }

    pub fn handle_message(&self, msg: &str) -> Option<String> {
        if msg.starts_with("!coin") || msg.starts_with("!crack") {
            return self.get_latest_price(self.get_coin(self.parse_coin_arg(msg)));
        }

        if msg.contains("github") {
            return Some("https://github.com/nemo-rb/rooney".to_string());
        }

        if msg == "!advice" {
            return Some(self.db.get_advice());
        }

        if msg.starts_with("!ats") {
            return self.get_ats(self.get_coin(self.parse_coin_arg(msg)));
        }

        if msg == "!bulls" {
            return self.get_bulls();
        }

        if msg == "!bears" {
            return self.get_bears();
        }

        if msg.starts_with("!fiat") {
            let (coin, amount) = self.parse_coin_amount(msg);
            return self.get_fiat(coin, amount);
        }

        if msg.starts_with("!stats") {
            let (coin, date) = self.parse_coin_date(msg);
            return self.get_stats(coin, date);
        }

        if msg.starts_with("!diff") {
            let (coin, date) = self.parse_coin_date(msg);
            return self.get_diff(coin, date);
        }

        None
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
        let date = Utc::today().naive_utc() - Duration::days(1);
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

    fn get_latest_price(&self, coin: String) -> Option<String> {
        let price = self.db.get_latest_price(coin);
        if let Some(p) = price {
            return Some(format!("{}", p));
        }

        None
    }

    fn get_ats(&self, coin: String) -> Option<String> {
        let ats = self.db.get_ats(coin);
        if let Some(a) = ats {
            return Some(format!("{}", a));
        }

        None
    }

    fn get_bulls(&self) -> Option<String> {
        let movers = self.db.get_bulls();
        if let Some(ms) = movers {
            return Some(ms.into_iter().map(|m| format!("{}", m)).collect::<Vec<String>>().join(" "));
        }

        None
    }

    fn get_bears(&self) -> Option<String> {
        let movers = self.db.get_bears();
        if let Some(ms) = movers {
            return Some(ms.into_iter().map(|m| format!("{}", m)).collect::<Vec<String>>().join(" "));
        }

        None
    }

    fn get_fiat(&self, coin: String, amount: f32) -> Option<String> {
        let price = self.db.get_latest_price(coin);
        if let Some(p) = price {
            return Some(format!("{} {} ({}) is worth €{} at €{} per coin", amount, titlecase(&p.name), p.ticker.to_uppercase(),
                                Replies::format_currency(amount * p.euro), Replies::format_currency(p.euro)))
        }

        None
    }

    fn get_stats(&self, coin: String, date: NaiveDate) -> Option<String> {
        let stats = self.db.get_stats(coin, date);
        if let Some(s) = stats {
            return Some(format!("{}", s));
        }

        None
    }

    fn get_diff(&self, coin: String, date: NaiveDate) -> Option<String> {
        let diff = self.db.get_diff(coin, date);
        if let Some(d) = diff {
            return Some(format!("{}", d));
        }

        None
    }

    fn format_currency(value: f32) -> String {
        if value < 1.0 {
            return format!("{:.8}", value);
        }

        let v = (value * 100.0).round() / 100.0;

        v.separated_string()
    }

    fn format_change(diff: f32) -> String {
        if diff < 0.0 {
            return format!("\x0305Down: {:.2}%", diff.abs());
        }

        format!("\x0303Up: {:.2}%", diff)
    }
}

impl fmt::Display for db::Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current price for {} ({}): €{} ${} 24h Low: €{} Median: €{} 24h High: €{} {} Today",
                    titlecase(&self.name), self.ticker.to_uppercase(), Replies::format_currency(self.euro),
                    Replies::format_currency(self.dollar), Replies::format_currency(self.min), Replies::format_currency(self.median),
                    Replies::format_currency(self.max), Replies::format_change(self.change))
    }
}

impl fmt::Display for db::ATS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "All time \x0305Low\x03/\x0303High\x03 Prices for {}, Lowest: \x0305€{}\x03 on {} Highest: \x0303€{}\x03 on {}",
            titlecase(&self.name), Replies::format_currency(self.lowest), self.lowest_date, Replies::format_currency(self.highest), self.highest_date
        )
    }
}

impl fmt::Display for db::Mover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}) {} Today\x03", titlecase(&self.name), self.ticker.to_uppercase(), Replies::format_change(self.diff))
    }
}

impl fmt::Display for db::Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stats for {} ({}) on {}: Min €{} Mean €{} Std Dev €{} Median €{} Max €{}",
                titlecase(&self.name), self.ticker.to_uppercase(), self.date, Replies::format_currency(self.min),
                Replies::format_currency(self.average), Replies::format_currency(self.std_dev),
                Replies::format_currency(self.median), Replies::format_currency(self.max))
    }
}

impl fmt::Display for db::Diff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Diff for {} ({}) from {} to {}: First: €{} Latest: €{} Diff: {} To Date",
                titlecase(&self.name), self.ticker.to_uppercase(), self.start, self.end,
                Replies::format_currency(self.first), Replies::format_currency(self.last), Replies::format_change(self.diff))
    }
}