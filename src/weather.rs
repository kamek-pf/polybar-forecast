use reqwest;
use serde::Serialize;
use serde_json::Value;
use serde_qs;

use super::{Configuration, Error};

pub struct WeatherInfo {
    pub icon: char,
    pub temperature: i8,
}

pub enum QueryType {
    Current,
    Forecast,
}

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

pub fn get_info(config: &Configuration, query: QueryType) -> Result<WeatherInfo, Error> {
    let params = QueryParams {
        app_id: &config.api_key,
        city_id: &config.city_id,
        units: &config.units,
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
        temperature: temperature as i8,
    })
}

fn parse_forecast(response: Value) -> Option<WeatherInfo> {
    let icon_code = response["list"][0]["weather"][0]["icon"].as_str()?;
    let temperature = response["list"][0]["main"]["temp"].as_f64()?.round();

    Some(WeatherInfo {
        icon: get_icon(icon_code),
        temperature: temperature as i8,
    })
}
