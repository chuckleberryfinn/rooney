use std::fmt;
use postgres::Connection;
use titlecase::titlecase;

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

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current price for {} ({}): €{} ${} 24h Low: €{} Median: €{} 24h High: €{} {} Today",
                    titlecase(&self.name), self.ticker.to_uppercase(), super::format_currency(self.euro),
                    super::format_currency(self.dollar), super::format_currency(self.min), super::format_currency(self.median),
                    super::format_currency(self.max), super::format_change(self.change))
    }
}

pub fn query(connection: &Connection, coin: &String) -> Option<Price> {
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

    let rows = connection.query(query, &[&coin]).unwrap();
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