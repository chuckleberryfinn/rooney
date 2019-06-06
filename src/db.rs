use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use postgres::{Connection, TlsMode};

pub struct DB {
    pub connection: Connection,
    pub all_coins: HashSet<String>,
    pub nicks_coins: HashMap<String, String>,
}

impl DB {
    pub fn new() -> Self {
        let config = "postgresql://nemo@%2Fvar%2Frun%2Fpostgresql";
        let c = Connection::connect(config, TlsMode::None).expect("Error connection to database");
        let nicks_coins = DB::get_nicks(&c);
        let all_coins = DB::get_coins(&nicks_coins);

        info!("Connection ok");

        Self {
            all_coins: all_coins,
            nicks_coins: nicks_coins,
            connection: c,
        }
    }

    pub fn get_advice(&self) -> String {
        let query = "select response from advice offset floor(random()*(select count(*) from advice)) limit 1;";

        let rows = self.connection.query(query, &[]).unwrap();
        rows.get(0).get(0)
    }

    fn get_nicks(connection: &Connection) -> HashMap<String, String> {
        let query = "Select ticker, name from coins";
        connection.query(query, &[]).unwrap().iter().map(|r| (r.get(0), r.get(1))).collect::<HashMap<String, String>>()
    }

    fn get_coins(nicks_coins: &HashMap<String, String>) -> HashSet<String> {
        HashSet::from_iter(nicks_coins.values().cloned())
    }

    pub fn get_latest_price(&self, coin: String) -> Option<Price> {
        let query =
                    "with daily_prices as (
                        select * from prices
                        join coins using(coin_id)
                        where time >= (select max(time::date) from prices)
                        and name = $1
                        order by time asc
                    ),
                    min_max_prices as (
                        select name, max(euro), min(euro)
                        from daily_prices
                        group by name
                    ),
                    all_prices as (
                        select row_number() over (partition by name order by time desc)
                        rn, name, ticker, euro, dollar, time
                        from daily_prices
                    ),
                    median_prices as (
                        select name, median(euro) as median
                        from all_prices
                        group by name
                    ),
                    latest_prices as (
                        select name, ticker, euro, dollar
                        from all_prices
                        where rn = 1
                    ),
                    first_price as (
                        select name, euro from daily_prices
                        where name=$1
                        limit 1
                    )
                    select name, ticker, cast(lp.euro as real), cast(dollar as real), cast(min as real), cast(max as real),
                    cast(((lp.euro - fp.euro)*100)/fp.euro as real), cast(median as real)
                    from latest_prices as lp
                    join min_max_prices using(name)
                    join first_price as fp using(name)
                    join median_prices using(name)";

        let rows = self.connection.query(query, &[&coin]).unwrap();
        if rows.len() == 0 {
            return None;
        }

        let row = rows.get(0);
        return Some(Price {
            name: row.get(0),
            ticker: row.get(1),
            euro: row.get(2),
            dollar: row.get(3),
            min: row.get(4),
            max: row.get(5),
            change: row.get(6),
            median: row.get(7),
        });
    }
}

pub struct Price {
    pub name: String,
    pub ticker: String,
    pub euro: f32,
    pub dollar: f32,
    pub min: f32,
    pub max: f32,
    pub change: f32,
    pub median: f32
}