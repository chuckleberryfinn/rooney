use super::{db, Command, Cooldown, Error, Result};
use std::cell::RefCell;
use std::time::{Duration, Instant};


const COOLDOWN: u64 = 3;


pub(super) struct Advice {
    last_call: RefCell<Option<Instant>>
}


impl Advice {
    pub(super) fn new() -> Self {
        Self {
            last_call: RefCell::new(None)
        }
    }

    fn query(&self, db: &db::DB) -> Option<String> {
        let query = "select response from advice offset floor(random()*(select count(*) from advice)) limit 1;";

        let rows = db.connection.query(query, &[]).unwrap();
        if rows.is_empty() {
            return None
        }
        Some(rows.get(0).get(0))
    }
}


impl Cooldown for Advice {
    fn get_last_call(&self) -> Option<Instant> {
        *self.last_call.borrow()
    }

    fn set_last_call(&self) {
        *self.last_call.borrow_mut() = Some(Instant::now())
    }

    fn on_cooldown(&self) -> bool {
        match self.get_last_call() {
            None => {
                self.set_last_call();
                false
            },
            last_call => {
                let elapsed = last_call.unwrap_or_else(Instant::now).elapsed();
                if elapsed > Duration::new(60*COOLDOWN, 0) {
                    self.set_last_call();
                    false
                } else {
                    true
                }
            }
        }
    }
}


impl Command for Advice {
    fn name(&self) -> &'static str {
        "!advice"
    }

    fn run(&self, db: &db::DB, _: &Option<&str>) -> Result<String> {
        if self.on_cooldown() {
            Err(Error::Cooldown)
        } else {
            Ok(self.query(&db).unwrap())
        }
    }

    fn help(&self) -> &'static str {
        "!advice: Some of mooney's sage advice"
    }
}
