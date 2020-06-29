use rooney::db;

use log::{error, info};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::{thread, time};
use toml::Value;

extern crate rustc_serialize;

use rustc_serialize::json;


#[allow(non_snake_case)]
#[derive(Clone, Debug, RustcDecodable)]
struct Market {
    Label: String,
    Name: String,
    Price_btc: f32,
    Price_usd: f32,
    Price_cny: f32,
    Price_eur: f32,
    Price_gbp: f32,
    Price_rur: f32,
    Volume_24h: f32,
    Timestamp: i64
}


impl Market {
    fn get_args(&self) -> (String, String, f32, f32, f32) {
        (self.Name.to_lowercase(), self.Label.split('/').next().unwrap().to_lowercase(), self.Price_btc, self.Price_eur, self.Price_usd)
    }
}


fn read_config(path: &str) -> Value {
    let toml_content = fs::read_to_string(path)
                        .unwrap_or_else(|_| panic!("Unable to read updater config from: {}", path));
    toml::from_str(&toml_content).unwrap_or_else(|_| panic!("Unable to parse TOML from {}", path))
}


fn parse_json(url: &str) -> Result<Vec<Market>, String> {
    let mut res = match reqwest::blocking::get(url) {
        Ok(r) => r,
        Err(e) => return Err(format!("Unable to read response from API: {}", e))
    };

    let mut body = String::new();
    match res.read_to_string(&mut body) {
        Ok(_) => (),
        Err(e) => return Err(format!("Unable to read response to string: {}", e))
    };

    let json: HashMap<String, Vec<Market>> = match json::decode(&body) {
        Ok(h) => h,
        Err(e) => return Err(format!("Unable to decode JSON: {}.", e))
    };

    match json.get("Markets") {
        Some(m) => Ok(m.to_vec()),
        None => Err("JSON has unexpected format.".to_string())
    }
}


fn add_coins(db: &db::DB, markets: &[Market]) -> Result<(), String> {
    let args = markets.iter().map(|m| m.get_args()).collect::<Vec<_>>();
    let transaction = match db.connection.transaction() {
        Ok(t) => t,
        Err(e) => return Err(format!("Failed to create transaction {}", e))
    };

    match transaction.batch_execute("Create temporary table temp_coins(name varchar(255), ticker varchar(100)) on commit drop") {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create temp table {}", e))
    };

    for a in args {
        match transaction.execute("Insert into temp_coins(name, ticker) values ($1, $2) ", &[&a.0, &a.1],) {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to insert into temp_coins table {}", e))
        };
    }

    match transaction.batch_execute("Insert into coins(name, ticker) select tc.name, tc.ticker from temp_coins tc left join coins c using(name) where c.name is null") {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to insert into coins table {}", e))
    };

    match transaction.commit() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to complete transaction {}", e))
    }
}


fn coins_ids(db: &db::DB) -> Result<HashMap<String, i32>, String> {
    match db.connection.query("Select name, coin_id from coins;", &[]) {
        Ok(rows) => Ok(rows.iter().map(|r| (r.get::<usize, String>(0).to_lowercase(), r.get(1))).collect::<HashMap<_, _>>()),
        Err(e) => Err(format!("Unable to find name/coin_id mappings {}", e))
    }
}


fn update_prices(db: &db::DB, coins_ids: HashMap<String, i32>, markets: &[Market]) -> Result<(), String> {
    //This whole function is here to work around the Postgres numeric type and it's incompatibility with Rust.
    let transaction = match db.connection.transaction() {
        Ok(t) => t,
        Err(e) => return Err(format!("Failed to create transaction {}", e))
    };

    match transaction.batch_execute("Create temporary table temp_prices(coin_id integer, euro real, dollar real) on commit drop") {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create temp table {}", e))
    };

    for m in markets {
        let args = m.get_args();
        let coin_id = coins_ids.get(&args.0).unwrap();
        match transaction.execute("Insert into temp_prices(coin_id, euro, dollar) values ($1, $2, $3) ", &[&coin_id, &args.3, &args.4],) {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to insert into temp_prices table {}", e))
        };
    }

    match transaction.batch_execute("Insert into prices(coin_id, euro, dollar) select coin_id, euro::numeric, dollar::numeric from temp_prices") {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to insert into prices table {}", e))
    };

    match transaction.commit() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to complete transaction {}", e))
    }
}


fn run_updater(url: &str) -> Result<(), String> {
    let markets = parse_json(&url)?;
    let db = db::DB::new();
    add_coins(&db, &markets)?;
    let coins_ids = coins_ids(&db)?;

    update_prices(&db, coins_ids, &markets)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let five_mins = time::Duration::from_secs(5*60);
    let config = read_config("configuration/Updater.toml");
    let key = config["updater"]["api_key"].as_str().unwrap();
    let url = format!("https://www.worldcoinindex.com/apiservice/json?key={}", key);

    loop {
        info!("Get updated price");

        match run_updater(&url) {
            Ok(_) => (),
            Err(e) => error!("An unexpected error occurred: {}", e)
        };

        thread::sleep(five_mins);
    }

    Ok(())
}
