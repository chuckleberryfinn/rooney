use std::fmt;
use titlecase::titlecase;

use super::formatter::format_change;

use postgres::Connection;


struct Mover {
    pub name: String,
    pub ticker: String,
    pub diff: f32
}

pub struct Movers {
    movers: Vec<Mover>
}

pub fn get_bears(connection: &Connection) -> Option<Movers> {
    query(connection, "asc")
}

pub fn get_bulls(connection: &Connection) -> Option<Movers> {
    query(connection, "desc")
}

fn query(connection: &Connection, sort: &str) -> Option<Movers> {
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

        let rows = connection.query(&query, &[]).unwrap();
        if rows.len() < 3 {
            return None;
        }

        Some(Movers {movers: rows.into_iter().map(|r| Mover {name: r.get(0), ticker: r.get(1), diff: r.get(4)}).collect::<Vec<Mover>>()})
}


impl fmt::Display for Mover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}) {} Today\x03", titlecase(&self.name), self.ticker.to_uppercase(), format_change(self.diff))
    }
}


impl fmt::Display for Movers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.movers[0], self.movers[1], self.movers[2])
    }
}