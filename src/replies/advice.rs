use super::db;

pub fn help() -> String {
    "!advice: Some of mooney's sage advice".to_string()
}

pub fn get_advice(db: &db::DB) -> Option<String> {
    db.get_advice()
}