use rooney::db;

use log::info;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::{thread, time};
use toml::Value;

extern crate rustc_serialize;

use rustc_serialize::json;


#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable)]
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


fn parse_json(url: &str) -> HashMap<String, Vec<Market>> {
    let mut res = reqwest::blocking::get(url).unwrap_or_else(|e| panic!("Unable to read response from API: {}", e));
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap_or_else(|e| panic!("Unable to read response to string: {}", e));
    json::decode(&body).unwrap_or_else(|e| panic!("Unable to decode JSON: {}.", e))
}


fn add_coins(db: &db::DB, markets: &[Market]) {
    let args = markets.iter().map(|m| m.get_args()).collect::<Vec<_>>();
    let transaction = db.connection.transaction().unwrap();
    transaction.batch_execute("Create temporary table temp_coins(name varchar(255), ticker varchar(100)) on commit drop").unwrap();
    for a in args {
        transaction.execute("Insert into temp_coins(name, ticker) values ($1, $2) ", &[&a.0, &a.1],).unwrap();
    }
    transaction.batch_execute("Insert into coins(name, ticker) select tc.name, tc.ticker from temp_coins tc left join coins c using(name) where c.name is null").unwrap();
    transaction.commit().unwrap();
}


fn coins_ids(db: &db::DB) -> Option<HashMap<String, i32>> {
    let rows = db.connection.query("Select name, coin_id from coins;", &[]).unwrap();

    if rows.is_empty() {
        return None;
    }

    Some(rows.iter().map(|r| (r.get::<usize, String>(0).to_lowercase(), r.get(1))).collect::<HashMap<_, _>>())
}


fn update_prices(db: &db::DB, coins_ids: HashMap<String, i32>, markets: &[Market]) {
    //This whole function is here to work around the Postgres numeric type and it's incompatibility with Rust.
    let transaction = db.connection.transaction().unwrap();
    transaction.batch_execute("Create temporary table temp_prices(coin_id integer, euro real, dollar real) on commit drop").unwrap();
    for m in markets {
        let args = m.get_args();
        let coin_id: i32 = *coins_ids.get(&args.0).unwrap();
        transaction.execute("Insert into temp_prices(coin_id, euro, dollar) values ($1, $2, $3) ", &[&coin_id, &args.3, &args.4],).unwrap();
    }
    transaction.batch_execute("Insert into prices(coin_id, euro, dollar) select coin_id, euro::numeric, dollar::numeric from temp_prices").unwrap();
    transaction.commit().unwrap();
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let config = read_config("configuration/Updater.toml");
    let key = config["updater"]["api_key"].as_str().unwrap();
    let url = format!("https://www.worldcoinindex.com/apiservice/json?key={}", key);
    let five_mins = time::Duration::from_secs(5*60);

    loop {
        info!("Get updated price");
        let json = parse_json(&url);
        let markets = json.get("Markets").unwrap_or_else(|| panic!("JSON has unexpected format."));
        let db = db::DB::new();
        add_coins(&db, &markets);
        let coins_ids = coins_ids(&db).unwrap_or_else(|| panic!("Unable to retrieve Coins from DB."));
        update_prices(&db, coins_ids, markets);
        db.connection.finish()?;
        thread::sleep(five_mins);
    }

    Ok(())
}