use std::env;
use std::fs::File;
use std::io::prelude::*;

use toml;
use failure::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub api_key: String,
    pub city_id: String,
    pub units: String,
    pub display_symbol: String,
}

pub fn get_config() -> Result<Configuration, Error> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("config.toml");

    let mut f = File::open(dir)
        .map_err(|_| ConfigError::MissingConfigFile)?;

    let mut content = String::new();
    f.read_to_string(&mut content)
        .map_err(|_| ConfigError::CannotReadConfigFile)?;

    let decoded: Configuration = toml::from_str(&content)
        .map_err(|_| ConfigError::InvalidConfigFile)?;

    Ok(decoded)
}

#[derive(Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "Could not find config.toml")] MissingConfigFile,
    #[fail(display = "Could not read config.toml")] CannotReadConfigFile,
    #[fail(display = "Invalid config.toml")] InvalidConfigFile,
}
