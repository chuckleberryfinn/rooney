use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

use chrono::{NaiveDate};
use postgres::{Connection, TlsMode};
use toml::Value;

use ats::ATS;
use mover::Mover;
use stats::Stats;
use diff::Diff;
use price::Price;
use fiat::Fiat;

pub mod advice;
pub mod ats;
pub mod diff;
pub mod fiat;
pub mod formatter;
pub mod mover;
pub mod nicks;
pub mod price;
pub mod remarks;
pub mod stats;

fn read_config(path: &str) -> Value {
    let toml_content = fs::read_to_string(path)
                        .unwrap_or_else(|_| panic!("Unable to read DB config from: {}", path));
    toml::from_str(&toml_content).unwrap_or_else(|_| panic!("Unable to parse TOML from {}", path))
}

pub struct DB {
    connection: Connection,
    pub all_coins: HashSet<String>,
    pub nicks_coins: HashMap<String, String>,
}

impl DB {
    pub fn new() -> Self {
        let config = read_config("configuration/DB.toml");
        let c = Connection::connect(config["database"]["connection"].as_str().unwrap(), TlsMode::None)
                                    .expect("Error connecting to database");
        let nicks_coins = DB::get_nicks(&c);
        let all_coins = DB::get_coins(&nicks_coins);

        Self {
            all_coins,
            nicks_coins,
            connection: c,
        }
    }

    pub fn get_advice(&self) -> Option<String> {
        advice::query(&self.connection)
    }

    fn get_nicks(connection: &Connection) -> HashMap<String, String> {
        nicks::query(connection)
    }

    fn get_coins(nicks_coins: &HashMap<String, String>) -> HashSet<String> {
        HashSet::from_iter(nicks_coins.values().cloned())
    }

    pub fn get_latest_price(&self, coin: String) -> Option<Price> {
        price::query(&self.connection, &coin)
    }

    pub fn get_ats(&self, coin: String) -> Option<ATS> {
        ats::query(&self.connection, coin)
    }

    pub fn get_bulls(&self) -> Option<Vec<Mover>> {
        mover::get_bulls(&self.connection)
    }

    pub fn get_bears(&self) -> Option<Vec<Mover>> {
        mover::get_bears(&self.connection)
    }

    pub fn get_stats(&self, coin: String, date: NaiveDate) -> Option<Stats> {
        stats::query(&self.connection, coin, date)
    }

    pub fn get_diff(&self, coin: String, date: NaiveDate) -> Option<Diff> {
        diff::query(&self.connection, coin, date)
    }

    pub fn get_remark(&self, msg: &str) -> Option<String> {
        remarks::query(&self.connection, msg)
    }

    pub fn get_fiat(&self, coin: String, amount: f32) -> Option<Fiat> {
        fiat::query(&self.connection, coin, amount)
    }
}
