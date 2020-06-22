use std::fmt;
use titlecase::titlecase;
use super::{db, Command, CommandArgs, Error, formatter::format_change, formatter::format_currency, Result};

pub(super) struct Coin;


pub struct _Coin {
    pub name: String,
    pub ticker: String,
    pub euro: f32,
    pub dollar: f32,
    pub min: f32,
    pub max: f32,
    pub change: f32,
    pub median: f32
}


impl Coin {
    pub fn query(&self, db: &db::DB, coin: &str) -> Option<_Coin> {
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
    
        let rows = db.connection.query(query, &[&coin]).unwrap();
        if rows.is_empty() {
            return None;
        }
    
        let row = rows.get(0);
        let c = _Coin {
            name: row.get(0),
            ticker: row.get(1),
            euro: row.get(2),
            dollar: row.get(3),
            min: row.get(4),
            max: row.get(5),
            change: row.get(6),
            median: row.get(7),
        };

        Some(c)
    }
}


impl fmt::Display for _Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current price for {} ({}): €{} ${} 24h Low: €{} Median: €{} 24h High: €{} {} Today",
                    titlecase(&self.name), self.ticker.to_uppercase(), format_currency(self.euro),
                    format_currency(self.dollar), format_currency(self.min), format_currency(self.median),
                    format_currency(self.max), format_change(self.change))
    }
}


impl Command for Coin {
    fn name(&self) -> &'static str {
        "!coin"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        let commands: Vec<&str> = msg.unwrap().split_whitespace().collect();
        let coin = self.get_coin(&db, self.parse_coin_arg(&commands));
        let price = self.query(&db, &coin);

        match price {
            Some(p) => Ok(format!("{}", p)),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!coin [coin|ticker]: Get current price for a coin. Defaults to btc."
    }
}


impl CommandArgs for Coin {}
