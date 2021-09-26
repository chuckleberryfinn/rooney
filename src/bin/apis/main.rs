#[macro_use] extern crate rocket;
use rooney::db;

use log::{error, info};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


fn get_coin(db: &db::DB, coin: String) -> String {
    if db.all_coins.contains(&coin) {
        return coin;
    }

    match db.nicks_coins.get(&coin) {
        Some(c) => c,
        None => "bitcoin"
    }.to_string()
}


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Prices {
    Prices: Vec<Price>
}


#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    name: String,
    ticker: String,
    euro: f32,
    dollar: f32,
    time: NaiveDateTime,
}


pub fn query(db: &db::DB, coin: &str) -> Option<Vec<Price>> {
    let query =
        "select name, ticker, cast(euro as real), cast(dollar as real), time
        from prices
        join coins using(coin_id)
        where time >= now() - interval '24 hours'
        and name = $1
        order by time asc";

    let rows = db.connection.query(query, &[&coin]).unwrap();
    if rows.is_empty() {
        return None;
    }

    Some(rows
        .iter()
        .map(|row| Price {
            name: row.get(0),
            ticker: row.get(1),
            euro: row.get(2),
            dollar: row.get(3),
            time: row.get(4),
            }
        )
        .collect())
}


#[get("/prices/<coin>")]
fn get_prices_last_24_hours(coin: &str) -> String {
    let db = db::DB::new().expect("Unable to access DB");
    let c = get_coin(&db, coin.to_string());
    let prices = query(&db, &c).unwrap();
    let j = serde_json::to_string(&prices).unwrap();
    format!("{}", j)
}


#[get("/coin/<coin>")]
fn get_last_price(coin: &str) -> String {
    let db = db::DB::new().expect("Unable to access DB");
    let c = get_coin(&db, coin.to_string());
    format!("Most recent price {}\n", c)
}


#[launch]
fn rocket() -> _ {
    info!("Launching rocket");

    rocket::build()
        .mount("/", routes![get_prices_last_24_hours, get_last_price])
}
