#[macro_use]
extern crate log;
extern crate env_logger;

mod db;
mod replies;

use irc::client::prelude::*;

pub fn run() {
    let config = Config::load("configuration/Config.toml").unwrap();
    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    let replies = replies::Replies::new();

    client.identify().unwrap();

    reactor.register_client_with_handler(client, move |client, message| {
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            match replies.handle_message(&msg) {
                Some(response) => client.send_privmsg(message.response_target().unwrap(), response)?,
                None => (),
            }
            info!("{} said {} to {}", message.source_nickname().unwrap(), msg, target);
        }

        Ok(())
    });

    reactor.run().unwrap();
}