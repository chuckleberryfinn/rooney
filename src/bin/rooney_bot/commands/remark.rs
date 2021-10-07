use super::{db, Command, Cooldown, Error, Result};
use std::cell::RefCell;
use std::time::{Duration, Instant};


const COOLDOWN: u64 = 3;


pub(super) struct Remark {
    last_call: RefCell<Option<Instant>>
}


impl Remark {
    pub(super) fn new() -> Self {
        Self {
            last_call: RefCell::new(None)
        }
    }

    pub fn query(&self, db: &mut db::DB, msg: &str) -> Option<String> {
        let query =
            "with all_remarks as (
                select remark from replies
                join replies_remarks using(reply_id)
                join remarks using(remark_id)
                where $1 ~ regex
            )
            select * from all_remarks
            offset floor(random() * (select count(*) from all_remarks))
            limit 1;";
    
        let rows = db.connection.query(query, &[&msg]).unwrap();
    
        if rows.is_empty() {
            return None;
        }
    
        rows.get(0).unwrap().get(0)
    }
}


impl Cooldown for Remark {
    fn get_last_call(&self) -> Option<Instant> {
        *self.last_call.borrow()
    }

    fn set_last_call(&self) {
        *self.last_call.borrow_mut() = Some(Instant::now())
    }

    fn on_cooldown(&self) -> bool {
        match self.get_last_call() {
            None => {
                false
            },
            last_call => {
                let elapsed = last_call.unwrap_or_else(Instant::now).elapsed();
                elapsed <= Duration::new(60*COOLDOWN, 0)
            }
        }
    }
}


impl Command for Remark {
    fn name(&self) -> &'static str {
        "remark"
    }

    fn run(&self, db: &mut db::DB, msg: &Option<&str>) -> Result<String> {
        if self.on_cooldown() {
            Err(Error::Cooldown)
        } else {
            match self.query(db, msg.unwrap()) {
                Some(r) => {
                    self.set_last_call();
                    Ok(r)
                },
                None => Err(Error::Reply)
            }
        }
    }

    fn help(&self) -> &'static str {
        ""
    }
}