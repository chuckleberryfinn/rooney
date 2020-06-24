use log::info;
use std::io::Read;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Updater");
    let url = "https://www.worldcoinindex.com/apiservice/json";
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}