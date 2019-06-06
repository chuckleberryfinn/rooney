use crate::db;
use std::fmt;

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

    pub fn handle_message(&self, msg: &str) -> Option<String> {
        if msg.starts_with("!coin") || msg.starts_with("!crack") {
            let words: Vec<&str> = msg.split_whitespace().collect();
            let coin = match words.len() {
                1 => "bitcoin".to_string(),
                _ => words[1].to_string().to_lowercase(),
            };
            return self.get_latest_price(self.get_coin(coin));
        }

        if msg.contains("github") {
            return Some("https://github.com/nemo-rb/rooney".to_string());
        }

        if msg == "!advice" {
            return Some(self.db.get_advice());
        }

        None
    }

    fn get_latest_price(&self, coin: String) -> Option<String> {
        let price = self.db.get_latest_price(coin);
        if let Some(p) = price {
            return Some(format!("{}", p));
        }
        None
    }

    pub fn format_currency(value: f32) -> String {
        if value < 1.0 {
            return format!("{:.8}", value);
        }

        let v = (value * 100.0).round() / 100.0;
        v.separated_string()
    }

    pub fn format_change(diff: f32) -> String {
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
