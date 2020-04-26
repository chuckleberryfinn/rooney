mod commands;
mod db;
mod irc;

use failure::Fail;

pub fn bot(messenger: impl Messenger) -> Result<()> {
    let commands = commands::Commands::new();

    messenger.init()?;
    messenger.run(|m| commands.handle(m))
}

pub trait Messenger {
    fn init(&self) -> Result<()>;
    fn run(&self, handler: impl Fn(&str) -> commands::Result<String>) -> Result<()>;
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Messenger error {}", _0)]
    Messenger(#[cause] Box<dyn Fail>),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;