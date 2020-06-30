use std::{collections::HashMap, error::Error, fmt, fs, io::Read, thread, time};

use rooney::db;

use log::{error, info};
use rustc_serialize::json;
use toml::Value;


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


#[derive(Debug)]
struct JSONParseError {
    message: String,
}


impl fmt::Display for JSONParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}


impl Error for JSONParseError {}


fn read_config(path: &str) -> Value {
    let toml_content = fs::read_to_string(path)
                        .unwrap_or_else(|_| panic!("Unable to read updater config from: {}", path));
    toml::from_str(&toml_content).unwrap_or_else(|_| panic!("Unable to parse TOML from {}", path))
}


fn get_json(url: &str) -> Result<String, Box<dyn Error>> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();

    res.read_to_string(&mut body)?;
    Ok(body)
}


fn parse_json(body: &str) -> Result<Vec<Market>, Box<dyn Error>> {
    let json: HashMap<String, Vec<Market>> = json::decode(&body)?;
    match json.get("Markets") {
        Some(m) => Ok(m.to_vec()),
        None => Err(Box::new(JSONParseError{ message: "Unable to parse markets".to_string() }))
    }
}


fn add_coins(db: &db::DB, markets: &[Market]) -> Result<(), Box<dyn Error>> {
    let args = markets.iter().map(|m| m.get_args()).collect::<Vec<_>>();
    let transaction = db.connection.transaction()?;
    transaction.batch_execute("Create temporary table temp_coins(name varchar(255), ticker varchar(100)) on commit drop")?;

    for a in args {
        transaction.execute("Insert into temp_coins(name, ticker) values ($1, $2) ", &[&a.0, &a.1],)?;
    }

    transaction.batch_execute("Insert into coins(name, ticker) select tc.name, tc.ticker from temp_coins tc left join coins c using(name) where c.name is null")?;
    transaction.commit()?;
    Ok(())
}


fn coins_ids(db: &db::DB) -> Result<HashMap<String, i32>, String> {
    match db.connection.query("Select name, coin_id from coins;", &[]) {
        Ok(rows) => Ok(rows.iter().map(|r| (r.get::<usize, String>(0).to_lowercase(), r.get(1))).collect::<HashMap<_, _>>()),
        Err(e) => Err(format!("Unable to find name/coin_id mappings {}", e))
    }
}


fn update_prices(db: &db::DB, coins_ids: HashMap<String, i32>, markets: &[Market]) -> Result<(), Box<dyn Error>> {
    //This whole function is here to work around the Postgres numeric type and it's incompatibility with Rust.
    let transaction = db.connection.transaction()?;
    transaction.batch_execute("Create temporary table temp_prices(coin_id integer, euro real, dollar real) on commit drop")?;

    for m in markets {
        let args = m.get_args();
        let coin_id = coins_ids.get(&args.0).unwrap();
        transaction.execute("Insert into temp_prices(coin_id, euro, dollar) values ($1, $2, $3) ", &[&coin_id, &args.3, &args.4],)?;
    }

    transaction.batch_execute("Insert into prices(coin_id, euro, dollar) select coin_id, euro::numeric, dollar::numeric from temp_prices")?;
    transaction.commit()?;
    Ok(())
}


fn get_updates(url: &str) -> Result<(), String> {
    let json = match get_json(&url) {
        Ok(j) => j,
        Err(e) => return Err(format!("Unable to get JSON: {}", e))
    };

    let markets = match parse_json(&json) {
        Ok(m) => m,
        Err(e) => return Err(format!("Unable to parse JSON: {}", e))
    };

    let db = db::DB::new();
    match add_coins(&db, &markets) {
        Ok(()) => (),
        Err(e) => return Err(format!("Unable to add coins: {}", e))
    };

    let coins_ids = coins_ids(&db)?;
    match update_prices(&db, coins_ids, &markets) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Unable to update prices: {}", e))
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let five_mins = time::Duration::from_secs(5*60);
    let config = read_config("configuration/Updater.toml");
    let key = config["updater"]["api_key"].as_str().expect("Updater.toml must include an api_key field in the updater section.");
    let url = format!("https://www.worldcoinindex.com/apiservice/json?key={}", key);

    loop {
        info!("Get updated price");

        match get_updates(&url) {
            Ok(_) => (),
            Err(e) => error!("An unexpected error occurred: {}", e)
        };

        thread::sleep(five_mins);
    }
}
