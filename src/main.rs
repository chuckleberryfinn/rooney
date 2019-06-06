mod db;
mod replies;

use irc::client::prelude::*;

fn main() {
    let config = Config::load("configuration/config.toml").unwrap();

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
            println!("{} said {} to {}", message.source_nickname().unwrap(), msg, target);
        }

        Ok(())
    });

    reactor.run().unwrap();
}
