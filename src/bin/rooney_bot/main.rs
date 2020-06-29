mod commands;
mod irc_handler;

use failure::Fail;
use std::env;


fn bot(messenger: impl Messenger) -> Result<()> {
    let commands = commands::Commands::new();

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
}


type Result<T, E = Error> = std::result::Result<T, E>;


fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let config = if args.len() == 1 {
        "configuration/DebugConfig.toml"
    } else {
        match args[1].as_str() {
            "release" => "configuration/Config.toml",
            _ => "configuration/DebugConfig.toml"
        }
    };
    
    bot(irc::client::prelude::IrcClient::new(&config)?)
}
