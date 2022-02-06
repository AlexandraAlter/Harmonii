use std::fs;
use std::path::Path;
use std::collections::HashSet;

use serde_derive::{Deserialize, Serialize};

use serenity::prelude::*;
use serenity::model::id::{ChannelId, EmojiId, GuildId, RoleId, UserId};

pub struct ConfigContainer;

impl TypeMapKey for ConfigContainer {
    type Value = Config;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub id: Option<UserId>,
    pub token: String,
    pub prefix: String,
    pub guilds: Vec<Guild>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: GuildId,
    pub name: Option<String>,
    pub owners: HashSet<UserId>,
    pub cleaner: Option<CleanerManager>,
    pub messager: Option<MessagerManager>,
    pub roles: Option<RoleManager>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanerManager {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagerManager {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleManager {
    pub watched: Vec<ChannelId>,
    pub available: Vec<Role>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: RoleId,
    pub name: Option<String>,
    pub emoji: Option<String>,
    pub emoji_id: Option<EmojiId>,
}

impl Config {
    pub fn from_file(path: &Path) -> Config {
        let data = fs::read_to_string(path).expect("Unable to read file");
        let config: Config = toml::from_str(&data).expect("Unable to parse TOML");
        return config;
    }
}
