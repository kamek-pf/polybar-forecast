mod types;
mod weather;

use std::{env, fs, process};

use handlebars::Handlebars;
use types::{Configuration, Error, Output, Unit};
use weather::{get_info, QueryType};

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

pub fn get_config() -> Result<Configuration, Error> {
    let content = dirs::config_dir()
        .and_then(|mut path| {
            // Check in .config first
            path.push("polybar-forecast/config.toml");
            fs::read_to_string(&path).ok()
        })
        .or_else(|| {
            // Otherwise, check in the same folder as the executable
            let mut dir = env::current_exe().ok()?;
            dir.pop();
            dir.push("config.toml");
            fs::read_to_string(&dir).ok()
        })
        .ok_or(Error::MissingConfigFile)?;

    let decoded: Configuration = toml::from_str(&content)?;

    Ok(decoded)
}

fn get_forecast() -> Result<String, Error> {
    let config = get_config()?;
    let current = get_info(&config, QueryType::Current)?;
    let forecast = get_info(&config, QueryType::Forecast)?;

    let mut reg = Handlebars::new();
    reg.set_strict_mode(true);

    let output = Output {
        temp_celcius: current.temperature.0,
        temp_kelvin: current.temperature.as_unit(Unit::Kelvin).0,
        temp_fahrenheit: current.temperature.as_unit(Unit::Fahrenheit).0,
        temp_icon: current.icon,
        trend: match (current.temperature, forecast.temperature) {
            (c, f) if c < f => '',
            (c, f) if c > f => '',
            _ => '',
        },
        forecast_celcius: forecast.temperature.0,
        forecast_kelvin: forecast.temperature.as_unit(Unit::Kelvin).0,
        forecast_fahrenheit: forecast.temperature.as_unit(Unit::Fahrenheit).0,
        forecast_icon: forecast.icon,
    };

    let rendered = reg.render_template(&config.display, &output)?;
    Ok(rendered)
}
