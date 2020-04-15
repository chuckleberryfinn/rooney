pub fn get_help(command: &str) -> Option<String> {
    match &command[..] {
        "advice" => Some(super::advice::help()),
        "ats" => Some(super::ats::help()),
        "bears" => Some(super::movers::bears_help()),
        "bulls" => Some(super::movers::bulls_help()),
        "coin" => Some(super::price::help()),        
        "diff" => Some(super::diff::help()),
        "fiat" => Some(super::fiat::help()),
        "stats" => Some(super::stats::help()),
        _ => None
    }
}