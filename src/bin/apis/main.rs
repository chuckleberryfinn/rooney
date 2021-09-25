#[macro_use] extern crate rocket;
use rooney::db;

use log::{error, info};


fn get_coin(db: &db::DB, coin: String) -> String {
    if db.all_coins.contains(&coin) {
        return coin;
    }

    match db.nicks_coins.get(&coin) {
        Some(c) => c,
        None => "bitcoin"
    }.to_string()
}


#[get("/prices/<coin>")]
fn get_prices_last_24_hours(coin: &str) -> String {
    let db = db::DB::new().expect("Unable to access DB");
    let c = get_coin(&db, coin.to_string());
    format!("Prices 24 hours {}", c)
}


#[get("/coin/<coin>")]
fn get_last_price(coin: &str) -> String {
    let db = db::DB::new().expect("Unable to access DB");
    let c = get_coin(&db, coin.to_string());
    format!("Most recent price {}", c)
}


#[launch]
fn rocket() -> _ {
    info!("Launching rocket");

    rocket::build()
        .mount("/", routes![get_prices_last_24_hours, get_last_price])
}
