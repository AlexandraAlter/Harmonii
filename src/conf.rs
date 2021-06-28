use std::path::Path;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    key: String,
    prefix: String,
    roles: Vec<Role>,
}

#[derive(Serialize, Deserialize)]
struct Role {
    name: String,
}


impl Config {
    fn from_file(path: Path): Config {
        let data = fs::read_to_string(path).expect("Unable to read file");
        println!("{}", data);
        let config: Config = toml::from_str(data).unwrap();
        return config
    }

    fn save(&self) {
        let data = toml::to_string(&self).unwrap();
        fs::write("/tmp/foo", data).expect("Unable to write file");
    }
}
