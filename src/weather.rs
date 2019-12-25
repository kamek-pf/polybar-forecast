use handlebars::Handlebars;
use serde::Serialize;
use serde_json::Value;

use crate::types::{Configuration, Error, Temperature, Unit};

// Implements OpenWeatherMap API calls
#[derive(Debug)]
pub struct OpenWeatherMap<'a> {
    config: &'a Configuration,
}

impl<'a> OpenWeatherMap<'a> {
    pub fn new(config: &'a Configuration) -> OpenWeatherMap<'a> {
        OpenWeatherMap { config }
    }

    pub fn get_info(&self, query: QueryType) -> Result<WeatherInfo, Error> {
        let params = QueryParams {
            app_id: &self.config.api_key,
            city_id: &self.config.city_id,
            units: Unit::Celcius.to_api(),
            cnt: 1,
        };

        let qs = serde_qs::to_string(&params).expect("Could not format query params");

        match query {
            QueryType::Current => {
                let url = "http://api.openweathermap.org/data/2.5/weather?".to_owned() + &qs;
                let res = reqwest::get(&url)?.json()?;

                parse_current(res).ok_or(Error::InvalidResponse)
            }
            QueryType::Forecast => {
                let url = "http://api.openweathermap.org/data/2.5/forecast?".to_owned() + &qs;
                let res = reqwest::get(&url)?.json()?;

                parse_forecast(res).ok_or(Error::InvalidResponse)
            }
        }
    }
}

// Output of API calls
pub struct WeatherInfo {
    icon: char,
    temperature: Temperature,
}

// Type of queries we can send to OpenWeatherMap
pub enum QueryType {
    Current,
    Forecast,
}

// Format a query string to perform HTTP calls
#[derive(Debug, Serialize)]
struct QueryParams<'a> {
    #[serde(rename = "APPID")]
    app_id: &'a str,
    #[serde(rename = "id")]
    city_id: &'a str,
    #[serde(rename = "units")]
    units: &'a str,
    cnt: i32,
}

// Implements formatting functions to render weather data on the bar
#[derive(Debug, Serialize)]
pub struct Output {
    temp_celcius: i16,
    temp_kelvin: i16,
    temp_fahrenheit: i16,
    temp_icon: char,
    trend: char,
    forecast_celcius: i16,
    forecast_kelvin: i16,
    forecast_fahrenheit: i16,
    forecast_icon: char,
}

impl Output {
    pub fn render(
        template: &str,
        current: WeatherInfo,
        forecast: WeatherInfo,
    ) -> Result<String, Error> {
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

        let rendered = reg.render_template(template, &output)?;
        Ok(rendered)
    }
}

// Map icon code to icon
// More details here: https://openweathermap.org/weather-conditions
fn get_icon(code: &str) -> char {
    match code {
        "01d" => '',         // Clear sky - day
        "01n" => '',         // Clear sky - night
        "02d" => '',         // Few clouds (11-25%) - day
        "02n" => '',         // Few clouds (11-25%) - night
        "03d" | "03n" => '', // Scattered clouds (25-50%) - day/night
        "04d" | "04n" => '', // Broken / Overcast clouds (51-84% / 85-100%) - day/night
        "09d" => '',         // Shower rain - day
        "09n" => '',         // Shower rain - night
        "10d" => '',         // Moderate / heavy rain - day
        "10n" => '',         // Moderate / heavy rain - night
        "11d" => '',         // Thunderstorm - day
        "11n" => '',         // Thunderstorm - night
        "13d" => '',         // Snow - day
        "13n" => '',         // Snow - night
        "50d" => '',         // Fog - day
        "50n" => '',         // Fog - night
        _ => '',             // ??
    }
}

fn parse_current(response: Value) -> Option<WeatherInfo> {
    let icon_code = response["weather"][0]["icon"].as_str()?;
    let temperature = response["main"]["temp"].as_f64()?.round();

    Some(WeatherInfo {
        icon: get_icon(icon_code),
        temperature: Temperature(temperature as i16, Unit::Celcius),
    })
}

fn parse_forecast(response: Value) -> Option<WeatherInfo> {
    let icon_code = response["list"][0]["weather"][0]["icon"].as_str()?;
    let temperature = response["list"][0]["main"]["temp"].as_f64()?.round();

    Some(WeatherInfo {
        icon: get_icon(icon_code),
        temperature: Temperature(temperature as i16, Unit::Celcius),
    })
}
