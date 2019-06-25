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

fn get_icon(code: &str) -> char {
    match code {
        "01d" => '',
        "01n" => '',
        "02d" => '',
        "02n" => '',
        "03d" | "03n" => '',
        "04d" | "04n" => '',
        "09d" => '',
        "09n" => '',
        "10d" => '',
        "10n" => '',
        "11d" => '',
        "11n" => '',
        "13d" => '',
        "13n" => '',
        "50d" => '',
        "50n" => '',
        _ => '',
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
