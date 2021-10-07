use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use rooney::db;

use log::{error, info};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


fn get_coin(db: &mut db::DB, coin: String) -> String {
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


pub fn query(db: &mut db::DB, coin: &str) -> Option<Vec<Price>> {
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


#[get("/prices/{coin}")]
async fn get_prices_last_24_hours(coin: web::Path<String>) -> impl Responder {
    let coin = coin.into_inner();
    let mut db = db::DB::new().expect("Unable to access DB");
    let c = get_coin(&mut db, coin.to_string());
    let prices = query(&mut db, &c).unwrap();
    let j = serde_json::to_string(&prices).unwrap();
    HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .body(j)
}


#[get("/coin/{coin}")]
async fn get_last_price(coin: web::Path<String>) -> impl Responder {
    let coin = coin.into_inner();
    let mut db = db::DB::new().expect("Unable to access DB");
    let c = get_coin(&mut db, coin.to_string());
    HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .body(format!("Most recent price {}\n", c))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Launching Actix");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("pems/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("pems/cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(get_prices_last_24_hours)
            .service(get_last_price)
    })
        .bind_openssl("0.0.0.0:8000", builder)?
        .run()
        .await
}
