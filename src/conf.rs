use std::fs;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

use serenity::prelude::*;

pub struct ConfigContainer;

impl TypeMapKey for ConfigContainer {
    type Value = Config;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub roles: Vec<Role>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
}

impl Config {
    pub fn from_file(path: &Path) -> Config {
        let data = fs::read_to_string(path).expect("Unable to read file");
        let config: Config = toml::from_str(&data).expect("Unable to parse TOML");
        return config;
    }

    pub fn save(&self) {
        let data = toml::to_string(&self).unwrap();
        fs::write("/tmp/foo", data).expect("Unable to write file");
    }
}
