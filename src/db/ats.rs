use postgres::Connection;

use chrono::NaiveDate;

pub struct ATS {
    pub name: String,
    pub lowest: f32,
    pub lowest_date: NaiveDate,
    pub highest: f32,
    pub highest_date: NaiveDate,
}

pub fn query(connection: &Connection, coin: String) -> Option<ATS> {
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

        let rows = connection.query(query, &[&coin]).unwrap();
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