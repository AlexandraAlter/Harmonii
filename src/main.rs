use std::path::Path;

use serenity::{
    client::Client,
    framework::standard::StandardFramework,
};
use clap::{App, Arg};

mod conf;
mod role;
mod messaging;
mod cleaning;

use conf::*;
use role::*;
use messaging::*;
use cleaning::*;

#[tokio::main]
async fn main() {
    let matches = App::new("Harmonii Discord Bot")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .default_value("harmonii.toml"),
        )
        .get_matches();

    let config_file = Path::new(matches.value_of("config").unwrap());
    let config = Config::from_file(config_file);

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&config.prefix))
        .group(&MESSAGING_GROUP)
        .group(&CLEANING_GROUP);

    let mut client = Client::builder(&config.token)
        .event_handler(role::RoleHandler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ConfigContainer>(config);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
