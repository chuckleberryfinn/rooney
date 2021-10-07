use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

use postgres::{Client, Error, NoTls};
use toml::Value;


fn read_config(path: &str) -> Value {
    let toml_content = fs::read_to_string(path)
                        .unwrap_or_else(|_| panic!("Unable to read DB config from: {}", path));
    toml::from_str(&toml_content).unwrap_or_else(|_| panic!("Unable to parse TOML from {}", path))
}


pub struct DB {
    pub connection: Client,
    pub all_coins: HashSet<String>,
    pub nicks_coins: HashMap<String, String>,
}


impl DB {
    pub fn new() -> Result<Self, Error> {
        let config = read_config("configuration/DB.toml");
        let mut c = Client::connect(config["database"]["connection"].as_str().unwrap(), NoTls)?;
        let nicks_coins = DB::get_nicks(&mut c);
        let all_coins = DB::get_coins(&nicks_coins);

        Ok(Self {
            all_coins,
            nicks_coins,
            connection: c,
        })
    }

    fn get_nicks(connection: &mut Client) -> HashMap<String, String> {
        let query = "Select ticker, name from coins";
        connection.query(query, &[]).unwrap().iter().map(|r| (r.get(0), r.get(1))).collect::<HashMap<String, String>>()
    }

    fn get_coins(nicks_coins: &HashMap<String, String>) -> HashSet<String> {
        HashSet::from_iter(nicks_coins.values().cloned())
    }
}
