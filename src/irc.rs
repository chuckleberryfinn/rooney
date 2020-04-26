use crate::{commands, Error, Messenger, Result};

use irc::client::prelude::*;
use irc::error::IrcError;
use log::{info, warn};

impl Messenger for IrcClient {
    fn init(&self) -> Result<()> {
        Ok(self.identify()?)
    }

    fn run(&self, handler: impl Fn(&str) -> commands::Result<String>) -> Result<()> {
        Ok(self.for_each_incoming(|message| {
            if let Command::PRIVMSG(channel, msg) = message.command {
                match handler(&msg) {
                    Ok(response) => {
                        info!("{}: {}", channel, response);
                        self.send_privmsg(&channel, &response)
                            .unwrap_or_else(|e| warn!("{}", e))
                    }
                    Err(e) => warn!("{}: {}", channel, e),
                }
            }
        })?)
    }
}

impl From<IrcError> for Error {
    fn from(e: IrcError) -> Error {
        Error::Messenger(Box::new(e))
    }
}
