use titlecase::titlecase;

use super::formatter::format_currency;

pub fn get_fiat(db: &super::db::DB, coin: String, amount: f32) -> Option<String> {
    let price = db.get_latest_price(coin);
    if let Some(p) = price {
        return Some(format!("{} {} ({}) is worth €{} at €{} per coin", amount, titlecase(&p.name), p.ticker.to_uppercase(),
                            format_currency(amount * p.euro), format_currency(p.euro)))
    }

    None
}