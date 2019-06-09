use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use chrono::NaiveDate;
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
        Some(Price {
            name: row.get(0),
            ticker: row.get(1),
            euro: row.get(2),
            dollar: row.get(3),
            min: row.get(4),
            max: row.get(5),
            change: row.get(6),
            median: row.get(7),
        })
    }

    pub fn get_ats(&self, coin: String) -> Option<ATS> {
        let query =
                    "with all_ats as (
                        select min(euro) as lowest, max(euro) as ath
                        from prices
                        join coins using(coin_id)
                        where name = $1
                        union select min_euro as lowest, max_euro as ath
                        from daily_stats
                        join coins using(coin_id)
                        where name = $1
                    ),
                    extremes as (
                        select min(lowest) as minimum, max(ath) as ath
                        from all_ats
                    ),
                    lowest as (
                        select time::date as date, euro as price
                        from prices
                        join coins using(coin_id)
                        where euro=(select minimum from extremes)
                        and name = $1
                        union select date, min_euro as price
                        from daily_stats
                        join coins using(coin_id)
                        where min_euro=(select minimum from extremes)
                        and name = $1
                        limit 1
                    ),
                    highest as (
                        select time::date as date, euro as price
                        from prices
                        join coins using(coin_id)
                        where euro=(select ath from extremes)
                        and name = $1
                        union select date, max_euro as price
                        from daily_stats
                        join coins using(coin_id)
                        where max_euro=(select ath from extremes)
                        and name = $1
                        limit 1
                    )
                    select date, cast(price as real) from lowest union select date, cast(price as real) from highest
                    order by price asc";

        let rows = self.connection.query(query, &[&coin]).unwrap();
        if rows.len() < 2 {
            return None;
        }

        let (lowest, highest) = (rows.get(0), rows.get(1));
        Some(ATS {
            name: coin,
            lowest_date: lowest.get(0),
            lowest: lowest.get(1),
            highest_date: highest.get(0),
            highest: highest.get(1)
        })
    }

    pub fn get_bulls(&self) -> Option<Vec<Mover>> {
        self.get_movers("desc")
    }

    pub fn get_bears(&self) -> Option<Vec<Mover>> {
        self.get_movers("asc")
    }

    fn get_movers(&self, sort: &str) -> Option<Vec<Mover>> {
        let query =
            format!(
            "with movers as (
                select distinct coin_id, first_value(euro) over w as first, last_value(euro) over w as last
                from prices where time::date=(select max(time)::date from prices) WINDOW w as (
                    partition by coin_id order by time range between unbounded preceding and unbounded
                    following) order by coin_id
            )
            select name, ticker, first, last, cast((last-first)*100/first as real) as diff
            from movers
            join coins using(coin_id)
            order by diff {} limit 3;", sort);

        let rows = self.connection.query(&query, &[]).unwrap();
        if rows.len() < 3 {
            return None;
        }

        Some(rows.into_iter().map(|r| Mover {name: r.get(0), ticker: r.get(1), diff: r.get(4)}).collect::<Vec<Mover>>())
    }

    pub fn get_stats(&self, coin: String, date: NaiveDate) -> Option<Stats> {
        let query =
            "select name, ticker, date, cast(min_euro as real), cast(average_euro as real), cast(median_euro as real), cast(std_dev as real), cast(max_euro as real)
                from daily_stats
                join coins using(coin_id)
                where name = $1
                and date = $2";
        let rows = self.connection.query(&query, &[&coin, &date]).unwrap();

        if rows.len() == 0 {
            return None;
        }

        let row = rows.get(0);
        Some(Stats{
            name: row.get(0),
            ticker: row.get(1),
            date: row.get(2),
            min: row.get(3),
            average: row.get(4),
            median: row.get(5),
            std_dev: row.get(6),
            max: row.get(7),
        })
    }

}

pub struct ATS {
    pub name: String,
    pub lowest: f32,
    pub lowest_date: NaiveDate,
    pub highest: f32,
    pub highest_date: NaiveDate,
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

pub struct Mover {
    pub name: String,
    pub ticker: String,
    pub diff: f32
}

pub struct Stats {
    pub name: String,
    pub ticker: String,
    pub date: NaiveDate,
    pub min: f32,
    pub average: f32,
    pub median: f32,
    pub std_dev: f32,
    pub max: f32,
}