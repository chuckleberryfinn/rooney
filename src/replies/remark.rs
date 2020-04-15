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

pub fn get_remark(db: &db::DB, msg: &str) -> Option<String> {
    match get_last_call() {
        None => {
            set_last_call();
            db.get_remark(msg)
        },
        i => {
            match i.unwrap_or_else(Instant::now).elapsed() {
                d if d > Duration::new(60*COOLDOWN, 0) => {
                    set_last_call();
                    db.get_remark(msg)
                },
                _ => None
            }
        }
    }
}