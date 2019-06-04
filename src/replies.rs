use crate::db;

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

        if self.db.all_coins.contains_key(&coin) {
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
                _ => words[1].to_string(),
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
        let mut output = None;
        let price = self.db.get_latest_price(coin);
        if let Some(p) = price {
            let response = format!("Current price for {} ({}): €{} ${} 24h Low: €{} Median: €{} 24h High: €{} {} Today",
                                    titlecase(&p.name), p.ticker.to_uppercase(), self.format_currency(p.euro),
                                    self.format_currency(p.dollar), self.format_currency(p.min), self.format_currency(p.median),
                                    self.format_currency(p.max), self.format_change(p.change));
            output = Some(response);
        }
        output
    }

    fn format_currency(&self, value: f32) -> String {
        if value < 1.0 {
            return format!("{:.8}", value);
        }

        let v = (value * 100.0).round() / 100.0;
        v.separated_string()
    }


    fn format_change(&self, diff: f32) -> String {
        if diff < 0.0 {
            return format!("\x0305Down: {:.2}%", diff.abs());
        }

        format!("\x0303Up: {:.2}%", diff)
    }
}
