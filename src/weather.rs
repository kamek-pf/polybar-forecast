use serde_qs;
use reqwest;
use serde_json::Value;

use config::Configuration;

pub struct WeatherInfo {
    pub icon: char,
    pub temperature: i8,
}

pub enum QueryType {
    Current,
    Forecast,
}

#[derive(Debug, Serialize)]
struct QueryParams {
    #[serde(rename = "APPID")] app_id: String,
    #[serde(rename = "id")] city_id: String,
    #[serde(rename = "units")] units: String,
    cnt: i32
}

pub fn get_info(config: &Configuration, query: QueryType) -> Result<WeatherInfo, ServiceError> {
    let params = QueryParams {
        app_id: config.clone().api_key,
        city_id: config.clone().city_id,
        units: config.clone().units,
        cnt: 1
    };

    let qs = &serde_qs::to_string(&params).map_err(|_| ServiceError::QueryError)?;
    
    match query {
        QueryType::Current => {
            let url = "http://api.openweathermap.org/data/2.5/weather?".to_owned() + qs;
            let res: Value = reqwest::get(&url)
                .map_err(|_| ServiceError::QueryError)?
                .json()
                .map_err(|_| ServiceError::QueryError)?;

            parse_current(res).ok_or(ServiceError::ParseError)
        },
        QueryType::Forecast => {
            let url = "http://api.openweathermap.org/data/2.5/forecast?".to_owned() + qs;
            let res: Value = reqwest::get(&url)
                .map_err(|_| ServiceError::QueryError)?
                .json()
                .map_err(|_| ServiceError::QueryError)?;

            parse_forecast(res).ok_or(ServiceError::ParseError)
        },
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

#[derive(Debug, Fail)]
pub enum ServiceError {
    #[fail(display = "Failed to query OpenWeatherMap")] QueryError,
    #[fail(display = "Failed to parse response")] ParseError,
}
