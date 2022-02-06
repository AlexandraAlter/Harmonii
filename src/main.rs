use std::path::Path;

use clap::{App, Arg};
use serenity::{client::Client, framework::standard::StandardFramework, http::Http};
use tracing::error;

#[macro_use]
mod utils;
mod commands;
mod conf;

use commands::{cleaning::*, messaging::*, role::*, utils::*};
use conf::*;

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
    let mut config = Config::from_file(config_file);

    tracing_subscriber::fmt::init();

    let http = Http::new_with_token(&config.token);

    match http.get_current_application_info().await {
        Ok(info) => {
            match config.id {
                None => config.id = Some(info.id),
                Some(id) => assert!(id == info.id),
            }
            for guild in &mut config.guilds {
                guild.owners.insert(info.owner.id);
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&config.prefix))
        .group(&UTILS_GROUP)
        .group(&MESSAGING_GROUP)
        .group(&CLEANING_GROUP);

    let mut client = Client::builder(&config.token)
        .event_handler(RoleHandler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ConfigContainer>(config);
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
