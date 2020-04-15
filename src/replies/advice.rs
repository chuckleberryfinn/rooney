use std::cell::RefCell;
use std::time::{Duration, Instant};

use super::db;

const COOLDOWN: u64 = 3;

thread_local! {
    pub static LAST_CALL: RefCell<Option<Instant>> = RefCell::new(None);
}

fn get_last_call() -> Option<Instant> {
    LAST_CALL.with(|last_call| *last_call.borrow())
}

fn set_last_call() {
    LAST_CALL.with(|last_call| {
        *last_call.borrow_mut() = Some(Instant::now())
    });
}

pub fn help() -> String {
    "!advice: Some of mooney's sage advice".to_string()
}

pub fn get_advice(db: &db::DB) -> Option<String> {
    match get_last_call() {
        None => {
            set_last_call();
            db.get_advice()
        },
        i => {
            match i.unwrap_or_else(Instant::now).elapsed() {
                d if d > Duration::new(60*COOLDOWN, 0) => {
                    set_last_call();
                    db.get_advice()
                },
                _ => None
            }
        }
    }
}