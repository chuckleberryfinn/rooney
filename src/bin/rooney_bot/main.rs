mod commands;
mod irc_handler;

use failure::Fail;
use log::{error, info};
use std::{env, thread, time};


fn bot(messenger: impl Messenger) -> Result<()> {
    let commands = match commands::Commands::new() {
        Ok(c) => c,
        Err(_) => return Err(Error::Contact)
    };

    messenger.init()?;
    messenger.run(|m| commands.handle(m))
}


trait Messenger {
    fn init(&self) -> Result<()>;
    fn run(&self, handler: impl Fn(&str) -> commands::Result<String>) -> Result<()>;
}


#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Messenger error {}", _0)]
    Messenger(#[cause] Box<dyn Fail>),
    #[fail(display = "Cannot contact database")]
    Contact,
}


type Result<T, E = Error> = std::result::Result<T, E>;


fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let five_mins = time::Duration::from_secs(5*60);

    let config = if args.len() == 1 {
        "configuration/DebugConfig.toml"
    } else {
        match args[1].as_str() {
            "release" => "configuration/Config.toml",
            _ => "configuration/DebugConfig.toml"
        }
    };

    loop {
        info!("Starting bot");

        match bot(irc::client::prelude::IrcClient::new(&config)?) {
            Ok(_) => (),
            Err(e) => error!("An unexpected error occurred: {}", e)
        };

        thread::sleep(five_mins);
    }
}
