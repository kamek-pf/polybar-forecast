extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_qs;
extern crate toml;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

mod config;
mod weather;

use failure::Error;
use config::get_config;
use weather::{get_info, QueryType, WeatherInfo};

fn get_forecast() -> Result<String, Error> {
    let cfg = get_config()?;
    let c = get_info(&cfg, QueryType::Current)?;
    let f = get_info(&cfg, QueryType::Forecast)?;

    if c.temperature < f.temperature {
        Ok(format_output(c, f, &cfg.display_symbol, ''))
    } else if c.temperature > f.temperature {
        Ok(format_output(c, f, &cfg.display_symbol, ''))
    } else {
        Ok(format_output(c, f, &cfg.display_symbol, ''))
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
        Err(e) => println!("Forecast unavailable ({:?})", e),
    }
}
