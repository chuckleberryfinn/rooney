extern crate irc;

use irc::client::prelude::*;

fn main() {
    let config = Config::load("configuration/config.toml").unwrap();

    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    reactor.register_client_with_handler(client, |client, message| {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            match msg.as_ref() {
                "!coin" => client.send_privmsg(message.response_target().unwrap(), "Hi!")?,
                _ => (),
            }
        }
        Ok(())
    });

    reactor.run().unwrap();
}
