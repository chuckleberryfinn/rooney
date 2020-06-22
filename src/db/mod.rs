use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

use chrono::{NaiveDate};
use postgres::{Connection, TlsMode};
use toml::Value;

use stats::Stats;

pub mod formatter;
pub mod nicks;
pub mod stats;

fn read_config(path: &str) -> Value {
    let toml_content = fs::read_to_string(path)
                        .unwrap_or_else(|_| panic!("Unable to read DB config from: {}", path));
    toml::from_str(&toml_content).unwrap_or_else(|_| panic!("Unable to parse TOML from {}", path))
}

pub struct DB {
    pub connection: Connection,
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

    fn get_nicks(connection: &Connection) -> HashMap<String, String> {
        nicks::query(connection)
    }

    fn get_coins(nicks_coins: &HashMap<String, String>) -> HashSet<String> {
        HashSet::from_iter(nicks_coins.values().cloned())
    }

    pub fn get_stats(&self, coin: String, date: NaiveDate) -> Option<Stats> {
        stats::query(&self.connection, coin, date)
    }
}
