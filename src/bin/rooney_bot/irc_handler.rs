use crate::{commands, Error, Messenger, Result};

use irc::client::prelude::*;
use irc::error::IrcError;
use log::{info, warn};

impl Messenger for IrcClient {
    fn init(&self) -> Result<()> {
        Ok(self.identify()?)
    }

    fn run(&self, mut handler: impl FnMut(&str) -> commands::Result<String>) -> Result<()> {
        Ok(self.for_each_incoming(|message| {
            if let Command::PRIVMSG(ref target, ref msg) = message.command {
                match handler(&msg) {
                    Ok(response) => {
                        self.send_privmsg(message.response_target().unwrap(), &response)
                            .unwrap_or_else(|e| warn!("{}", e))
                    }
                    Err(e) => warn!("{}: {}", target, e),
                }
                info!("{} said {} to {}", message.source_nickname().unwrap(), msg, target);
            }
        })?)
    }
}

impl From<IrcError> for Error {
    fn from(e: IrcError) -> Error {
        Error::Messenger(Box::new(e))
    }
}
