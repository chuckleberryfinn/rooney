use std::env;

fn main() -> rooney::Result<()> {
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
    
    rooney::bot(irc::client::prelude::IrcClient::new(&config)?)
}
