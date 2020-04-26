#[macro_use]
extern crate log;
extern crate env_logger;

mod db;
mod commands;

use std::env;

use irc::client::prelude::*;

pub fn run() {
    let args: Vec<String> = env::args().collect();


    let config = if args.len() == 1 {
        Config::load("configuration/DebugConfig.toml").unwrap()
    } else {
        match args[1].as_str() {
            "release" => Config::load("configuration/Config.toml").unwrap(),
            _ => Config::load("configuration/DebugConfig.toml").unwrap()
        }
    };

    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    let commands = commands::Commands::new();

    client.identify().unwrap();

    reactor.register_client_with_handler(client, move |client, message| {
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            match commands.handle(&msg) {
                Ok(response) => client.send_privmsg(message.response_target().unwrap(), response)?,
                Err(e) => info!("Error encountered: {:?}", e),
            }
            info!("{} said {} to {}", message.source_nickname().unwrap(), msg, target);
        }

        Ok(())
    });

    reactor.run().unwrap();
}