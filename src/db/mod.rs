use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use chrono::{NaiveDate};
use postgres::{Connection, TlsMode};
use separator::Separatable;

use ats::ATS;
use mover::Mover;
use stats::Stats;
use diff::Diff;
use price::Price;

pub mod advice;
pub mod ats;
pub mod diff;
pub mod mover;
pub mod nicks;
pub mod price;
pub mod remarks;
pub mod stats;

fn format_change(diff: f32) -> String {
    if diff < 0.0 {
        return format!("\x0305Down: {:.2}%", diff.abs());
    }

    format!("\x0303Up: {:.2}%", diff)
}

fn format_currency(value: f32) -> String {
    if value < 1.0 {
        return format!("{:.8}", value);
    }

    let v = (value * 100.0).round() / 100.0;

    v.separated_string()
}

pub struct DB {
    connection: Connection,
    pub all_coins: HashSet<String>,
    pub nicks_coins: HashMap<String, String>,
}

impl DB {
    pub fn new() -> Self {
        let config = "postgresql://nemo@%2Fvar%2Frun%2Fpostgresql";
        let c = Connection::connect(config, TlsMode::None).expect("Error connection to database");
        let nicks_coins = DB::get_nicks(&c);
        let all_coins = DB::get_coins(&nicks_coins);

        Self {
            all_coins: all_coins,
            nicks_coins: nicks_coins,
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
}
