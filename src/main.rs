mod types;
mod weather;

use std::process;

use types::{Configuration, Error};
use weather::{OpenWeatherMap, Output, QueryType};

fn main() {
    match get_forecast() {
        Ok(forecast) => println!("{}", forecast),
        Err(e) => {
            // Line break prevents massive errors from trashing the bar,
            // Polybar displays everything until the first line break
            eprintln!("\n{}", e);
            process::exit(1);
        }
    }
}

fn get_forecast() -> Result<String, Error> {
    let config = Configuration::new()?;
    let owm = OpenWeatherMap::new(&config);
    let current = owm.get_info(QueryType::Current)?;
    let forecast = owm.get_info(QueryType::Forecast)?;

    Output::render(&config.display, current, forecast)
}
