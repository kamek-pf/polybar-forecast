mod weather;

use std::fmt::{self, Display};
use std::{env, fs, process};

use weather::{get_info, QueryType, WeatherInfo};

use serde::Deserialize;

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

    let content = fs::read_to_string(&dir)?;
    let decoded: Configuration = toml::from_str(&content)?;

    Ok(decoded)
}

fn get_forecast() -> Result<String, Error> {
    let config = get_config()?;
    let c = get_info(&config, QueryType::Current)?;
    let f = get_info(&config, QueryType::Forecast)?;

    if c.temperature < f.temperature {
        Ok(format_output(c, f, &config.display_symbol, ''))
    } else if c.temperature > f.temperature {
        Ok(format_output(c, f, &config.display_symbol, ''))
    } else {
        Ok(format_output(c, f, &config.display_symbol, ''))
    }
}

fn format_output(current: WeatherInfo, forecast: WeatherInfo, unit: &str, trend: char) -> String {
    format!(
        "{ci} {ct}{u} {trend} {fi} {ft}{u}",
        ct = current.temperature,
        ci = current.icon,
        ft = forecast.temperature,
        fi = forecast.icon,
        u = unit,
        trend = trend
    )
}

fn main() {
    match get_forecast() {
        Ok(forecast) => println!("{}", forecast),
        Err(e) => {
            // Line break prevents massive errors from trashing the bar,
            // Polybar displays everything until the first line break
            eprintln!("\nForecast unavailable ({})", e);
            process::exit(1);
        }
    }
}

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    MissingConfigFile(std::io::Error),
    InvalidConfigFile(toml::de::Error),
    InvalidResponse,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            HttpError(e) => write!(f, "Failed to query OpenWeatherMap: {:?}", e),
            MissingConfigFile(e) => write!(f, "Could not find config file: {:?}", e),
            InvalidConfigFile(e) => write!(f, "Could not parse config file as TOML: {:?}", e),
            InvalidResponse => write!(f, "Invalid response format from OpenWeatherMap"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::HttpError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::InvalidConfigFile(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::MissingConfigFile(err)
    }
}
