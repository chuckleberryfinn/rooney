use std::fmt;
use titlecase::titlecase;

use super::formatter::format_change;

use super::{db, Command, CommandArgs, Error, Result};
pub(super) struct Bulls;


struct Mover {
    pub name: String,
    pub ticker: String,
    pub diff: f32
}


pub struct Movers {
    movers: Vec<Mover>
}


impl Bulls {
    fn query(&self, db: &mut db::DB) -> Option<Movers> {
        let query =
            "with movers as (
                select distinct coin_id, first_value(euro) over w as first, last_value(euro) over w as last
                from prices where time::date=(select max(time)::date from prices) WINDOW w as (
                    partition by coin_id order by time range between unbounded preceding and unbounded
                    following) order by coin_id
            )
            select name, ticker, first, last, cast((last-first)*100/first as real) as diff
            from movers
            join coins using(coin_id)
            where first != 0
            order by diff desc limit 3;";
    
        let rows = db.connection.query(query, &[]).unwrap();
        if rows.len() < 3 {
            return None;
        }
        Some(Movers {movers: rows.into_iter().map(|r| Mover {name: r.get(0), ticker: r.get(1), diff: r.get(4)}).collect::<Vec<Mover>>()})
    }
}


impl Command for Bulls {
    fn name(&self) -> &'static str {
        "!bulls"
    }

    fn run(&self, db: &mut db::DB, _: &Option<&str>) -> Result<String> {
        let movers = self.query(db);

        match movers {
            Some(ms) => Ok(ms.to_string()),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!bulls: Get today's big winners."
    }
}


impl CommandArgs for Bulls {}

pub(super) struct Bears;


impl Bears {
    fn query(&self, db: &mut db::DB) -> Option<Movers> {
        let query =
            "with movers as (
                select distinct coin_id, first_value(euro) over w as first, last_value(euro) over w as last
                from prices where time::date=(select max(time)::date from prices) WINDOW w as (
                    partition by coin_id order by time range between unbounded preceding and unbounded
                    following) order by coin_id
            )
            select name, ticker, first, last, cast((last-first)*100/first as real) as diff
            from movers
            join coins using(coin_id)
            where first != 0
            order by diff asc limit 3;";
    
        let rows = db.connection.query(query, &[]).unwrap();
        if rows.len() < 3 {
            return None;
        }
        Some(Movers {movers: rows.into_iter().map(|r| Mover {name: r.get(0), ticker: r.get(1), diff: r.get(4)}).collect::<Vec<Mover>>()})
    }
}


impl Command for Bears {
    fn name(&self) -> &'static str {
        "!bears"
    }

    fn run(&self, db: &mut db::DB, _: &Option<&str>) -> Result<String> {
        let movers = self.query(db);

        match movers {
            Some(ms) => Ok(ms.to_string()),
            None => Err(Error::Contact)
        }
    }

    fn help(&self) -> &'static str {
        "!bears: Get today's big losers."
    }
}


impl CommandArgs for Bears {}


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
